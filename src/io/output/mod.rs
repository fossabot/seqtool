use std::io;
use std::path::PathBuf;
use std::fs::File;

use flate2;
use bzip2;
use lz4;

use error::{CliError, CliResult};
use lib::thread_io;

use super::{fasta, fastq, Compression, Record, SeqWriter};

pub use self::writer::*;

pub mod prop;
pub mod csv;
pub mod writer;

lazy_static! {
    static ref STDOUT: io::Stdout = io::stdout();
}

#[derive(Clone, Debug)]
pub struct OutputOptions {
    pub kind: OutputKind,
    pub format: OutFormat,
    pub compression: Option<Compression>,
    pub threaded: bool,
    //pub qfile: Option<PathBuf>,
}

impl Default for OutputOptions {
    fn default() -> OutputOptions {
        OutputOptions {
            kind: OutputKind::Stdout,
            format: OutFormat::FASTA(vec![], None),
            compression: None,
            threaded: false,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum OutputKind {
    Stdout,
    File(PathBuf),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum OutFormat {
    // q64, wrap_width, Vec<(prop_name, prop_value)>, default_seqattr_for_props
    FASTA(Vec<(String, String)>, Option<usize>),
    FASTQ(Vec<(String, String)>),
    //    FA_QUAL(PathBuf),
    CSV(u8, Vec<String>),
}

impl OutFormat {
    pub fn default_ext(&self) -> &'static str {
        match *self {
            OutFormat::FASTA(..) => "fasta",
            OutFormat::FASTQ(..) => "fastq",
            OutFormat::CSV(delim, _) => if delim == b'\t' {
                "txt"
            } else {
                "csv"
            },
        }
    }
}

pub fn writer<F, O>(opts: Option<&OutputOptions>, func: F) -> CliResult<O>
where
    F: FnOnce(&mut Writer) -> CliResult<O>,
{
    if let Some(o) = opts {
        io_writer_compr(&o.kind, o.compression, o.threaded, |io_writer| {
            let mut w = from_format(io_writer, &o.format)?;
            func(&mut w)
        })
    } else {
        func(&mut NoOutput)
    }
}

pub fn io_writer<F, O>(opts: Option<&OutputOptions>, func: F) -> CliResult<O>
where
    F: FnOnce(&mut io::Write) -> CliResult<O>,
{
    if let Some(o) = opts {
        io_writer_compr(&o.kind, o.compression, o.threaded, func)
    } else {
        func(&mut io::sink())
    }
}

pub fn from_format<'a, W>(io_writer: W, format: &OutFormat) -> CliResult<Box<Writer + 'a>>
where
    W: io::Write + 'a,
{
    Ok(match *format {
        OutFormat::FASTA(ref props, ref wrap) => {
            let writer = fasta::FastaWriter::new(io_writer, *wrap);
            Box::new(prop::PropWriter::new(writer, props.clone()))
        }
        OutFormat::FASTQ(ref props) => {
            let writer = fastq::FastqWriter::new(io_writer);
            // if q64 {
            //     writer = writer.q64();
            // }
            Box::new(prop::PropWriter::new(writer, props.clone()))
        }
        OutFormat::CSV(delim, ref fields) => {
            Box::new(csv::CsvWriter::new(io_writer, fields.clone(), delim))
        }
    })
}

pub fn from_kind(kind: &OutputKind) -> io::Result<Box<io::Write>> {
    Ok(match *kind {
        OutputKind::Stdout => Box::new(io::BufWriter::new(STDOUT.lock())),
        OutputKind::File(ref p) => Box::new(io::BufWriter::new(File::create(p)?)),
    })
}

pub fn compr_writer(
    writer: Box<io::Write>,
    compression: Compression,
) -> io::Result<Box<io::Write>> {
    Ok(match compression {
        Compression::GZIP => Box::new(flate2::write::GzEncoder::new(
            writer,
            flate2::Compression::default(),
        )),
        Compression::BZIP2 => Box::new(bzip2::write::BzEncoder::new(
            writer,
            bzip2::Compression::Default,
        )),
        Compression::LZ4 => Box::new(lz4::EncoderBuilder::new().build(writer)?),
    })
}

fn io_writer_compr<F, O>(
    kind: &OutputKind,
    compr: Option<Compression>,
    threaded: bool,
    func: F,
) -> CliResult<O>
where
    F: FnOnce(&mut io::Write) -> CliResult<O>,
{
    if compr.is_some() || threaded {
        // compressed input
        // TODO: not configurable
        let bufsize = 1 << 22;
        thread_io::write::writer_with(
            bufsize,
            4,
            || {
                let mut writer = from_kind(kind)?;
                if let Some(compr) = compr {
                    writer = compr_writer(writer, compr)?;
                }
                Ok::<_, CliError>(writer)
            },
            |mut w| func(&mut w),
        ).unwrap()
    } else {
        let mut writer = from_kind(kind)?;
        func(&mut writer)
    }
}
