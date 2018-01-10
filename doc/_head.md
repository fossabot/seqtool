**Seqtool** is a  general purpose command line program for dealing with
large amounts of biological sequences. It can read and write 
**FASTA**, **FASTQ** and **CSV** files and supports different compression
algorithms (**GZIP**, **BZIP2**, **LZ4**), auto-recognizing the 
extensions.

The tool is written in [Rust](https://www.rust-lang.org) and aims at solving simple tasks that might otherwise only be solved by writing
custom scripts. This is possible with the use of 
[variables and mathematical expressions](wiki/variables).
In contrast to [biopieces](https://github.com/maasha/biopieces),
no custom format is used for passing information between commands.
Instead, it is possible to use '[attributes](wiki/attributes)', which are key=value strings added to the sequence headers, or custom CSV fields.

It uses the [Rust-Bio](http://rust-bio.github.io/) and 
[seq_io](https://github.com/markschl/seq_io) libraries, amongst others
and compiles to a standalone binary.


[![UNIX build status](https://travis-ci.org/markschl/seqtool.svg?branch=master)](https://travis-ci.org/markschl/seqtool/)
[![Windows build status](https://ci.appveyor.com/api/projects/status/github/markschl/seqtool?svg=true)](https://ci.appveyor.com/project/markschl/seqtool)
