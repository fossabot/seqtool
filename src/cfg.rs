use std::cell::Cell;
use std::io;

use error::CliResult;
use io::*;
use opt;
use var;

#[derive(Debug, Clone)]
pub struct Config<'a> {
    input_opts: Vec<input::InputOptions>,
    output_opts: output::OutputOptions,
    var_opts: var::VarOpts<'a>,
    started: Cell<bool>,
}

impl<'a> Config<'a> {
    pub fn from_args(args: &'a opt::Args) -> CliResult<Config<'a>> {
        Self::new(args, None)
    }

    pub fn from_args_with_help(
        args: &'a opt::Args,
        custom_help: &var::VarHelp,
    ) -> CliResult<Config<'a>> {
        Self::new(args, Some(custom_help))
    }

    pub fn new(args: &'a opt::Args, custom_help: Option<&var::VarHelp>) -> CliResult<Config<'a>> {
        let input_opts = args.get_input_opts()?;

        let out_opts = args.get_output_opts(Some(&input_opts[0].format))?;

        let var_opts = args.get_env_opts()?;

        if var_opts.var_help {
            let h = if let Some(h) = custom_help {
                format!("{}\n\n{}", h.format(), var::var_help())
            } else {
                var::var_help()
            };
            return fail!(h);
        }

        Ok(Config {
            output_opts: out_opts,
            input_opts: input_opts,
            var_opts: var_opts,
            started: Cell::new(false),
        })
    }

    pub fn vars(&self) -> CliResult<var::Vars> {
        let mut vars = var::get_vars(&self.var_opts, &self.input_opts[0].format)?;
        vars.out_opts(&self.output_opts)?;
        Ok(vars)
    }

    pub fn input_opts(&self) -> &[input::InputOptions] {
        &self.input_opts
    }

    pub fn writer<F, O>(&self, func: F) -> CliResult<O>
    where
        F: FnOnce(&mut output::Writer<&mut io::Write>, var::Vars) -> CliResult<O>,
    {
        output::writer(&self.output_opts, |writer| {
            let mut vars = self.vars()?;
            vars.build(|b| writer.register_vars(b))?;
            func(writer, vars)
        })
    }

    pub fn writer_with<F, O, I, V>(&self, init: I, func: F) -> CliResult<O>
    where
        F: FnOnce(&mut output::Writer<&mut io::Write>, var::Vars, V) -> CliResult<O>,
        I: FnOnce(&mut var::Vars) -> CliResult<V>,
        V: var::VarProvider,
    {
        output::writer(&self.output_opts, |writer| {
            let mut vars = self.vars()?;
            let mut var_provider = init(&mut vars)?;
            vars.build_with(Some(&mut var_provider), |b| writer.register_vars(b))?;
            func(writer, vars, var_provider)
        })
    }

    pub fn io_writer<F, O>(&self, func: F) -> CliResult<O>
    where
        F: FnOnce(&mut io::Write, var::Vars) -> CliResult<O>,
    {
        output::io_writer(&self.output_opts, |writer| {
            let vars = self.vars()?;
            func(writer, vars)
        })
    }

    pub fn read_sequential<F>(&self, mut func: F) -> CliResult<()>
    where
        F: FnMut(&Record) -> CliResult<bool>,
    {
        self.check_repetition()?;
        input::io_readers(&self.input_opts, |o, rdr| {
            input::run_reader(rdr, &o.format, o.cap, o.max_mem, &mut func)
        })?;
        Ok(())
    }

    pub fn all_readers<F>(&self, func: F) -> CliResult<()>
    where
        F: FnMut(usize, &Record) -> CliResult<()>,
    {
        self.check_repetition()?;
        input::all_readers(&self.input_opts, func)
    }

    pub fn num_readers(&self) -> usize {
        self.input_opts.len()
    }

    pub fn read_sequential_var<F>(&self, vars: &mut var::Vars, mut func: F) -> CliResult<()>
    where
        F: FnMut(&Record, &mut var::Vars) -> CliResult<bool>,
    {
        self.check_repetition()?;
        input::io_readers(&self.input_opts, |o, rdr| {
            vars.new_input(o)?;
            input::run_reader(rdr, &o.format, o.cap, o.max_mem, &mut |rec| {
                vars.set_record(&rec)?;
                func(&rec, vars)
            })
        })?;
        Ok(())
    }

    pub fn parallel<W, F, O>(&self, n_threads: u32, work: W, mut func: F) -> CliResult<Vec<()>>
    where
        W: Fn(&Record, &mut O) -> CliResult<()> + Send + Sync,
        F: FnMut(&Record, &mut O) -> CliResult<bool>,
        O: Send + Default,
    {
        self.check_repetition()?;
        input::io_readers(&self.input_opts, |o, rdr| {
            input::read_parallel(
                o,
                rdr,
                n_threads,
                || Ok(()),
                Default::default,
                |rec, out, _| work(rec, out),
                &mut func,
            )
        })
    }

    pub fn var_parallel_init<Si, S, Di, W, F, D>(
        &self,
        mut vars: &mut var::Vars,
        n_threads: u32,
        local_init: Si,
        data_init: Di,
        work: W,
        mut func: F,
    ) -> CliResult<Vec<()>>
    where
        W: Fn(&Record, &mut D, &mut S) -> CliResult<()> + Send + Sync,
        F: FnMut(&Record, &mut D, &mut var::Vars) -> CliResult<bool>,
        Di: Fn() -> D + Send + Sync,
        D: Send,
        S: Send,
        Si: Fn() -> CliResult<S> + Send + Sync,
    {
        self.check_repetition()?;
        input::io_readers(&self.input_opts, |in_opts, rdr| {
            vars.new_input(in_opts)?;
            input::read_parallel(
                in_opts,
                rdr,
                n_threads,
                &local_init,
                &data_init,
                &work,
                |rec, out| {
                    vars.set_record(rec)?;
                    func(rec, out, &mut vars)
                },
            )
        })
    }

    pub fn var_parallel<W, F, O>(
        &self,
        mut vars: &mut var::Vars,
        n_threads: u32,
        work: W,
        mut func: F,
    ) -> CliResult<Vec<()>>
    where
        W: Fn(&Record, &mut O) -> CliResult<()> + Send + Sync,
        F: FnMut(&Record, &mut O, &mut var::Vars) -> CliResult<bool>,
        O: Send + Default,
    {
        self.parallel(n_threads, work, |rec, out| {
            vars.set_record(rec)?;
            func(rec, out, &mut vars)
        })
    }

    pub fn has_stdin(&self) -> bool {
        self.input_opts
            .iter()
            .any(|o| o.kind == input::InputType::Stdin)
    }

    /// ensures that STDIN cannot be read twice
    /// (would result in empty input on second attempt)
    fn check_repetition(&self) -> CliResult<()> {
        if self.started.get() && self.has_stdin() {
            return fail!("Cannot read twice from STDIN");
        }
        self.started.set(true);
        Ok(())
    }

    pub fn other_writer<'c>(
        &self,
        path: &str,
        vars: Option<&mut var::Vars>,
        other_mod: Option<&mut var::VarProvider>,
    ) -> CliResult<Box<output::Writer<Box<output::WriteFinish>> + 'c>> {
        let mut o = self.output_opts.clone();
        o.kind = output::OutputKind::File(path.into());
        let io_writer = output::io_writer_from_kind(&o.kind)?;
        let io_writer = output::compr_writer(io_writer, o.compression, o.compression_level)?;
        let mut w = output::from_format(io_writer, &o.format)?;
        if let Some(v) = vars {
            v.build_with(other_mod, |b| w.register_vars(b))?;
        }
        Ok(w)
    }
}
