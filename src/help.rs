macro_rules! common_opts { () => (r"
Input options:
    --format <format>   Input format: fasta (default), fastq or csv. Compression:
                        <format>.<compression> (.gz, .bz2 or .lz4). Only
                        needed if the format cannot be guessed from the extension.
    --fields <fields>   CSV fields: 'id,seq,desc' (in order) or 'id:2,desc:6,seq:9'
                        (col. num.) or headers: 'id:id,seq:sequence,desc:desc'
                        [default: id,seq,desc]
    --delim <delim>     TXT/CSV delimiter. Defaults: '\t' for txt; ',' for csv
    --header            Specify if CSV file has a header. Auto-enabled with headers.
    --fa                FASTA input. Short for '--format fasta'.
    --fq                FASTQ input. Short for '--format fastq'.
    --csv <fields>      CSV input. Short for '--format csv --fields <fields>'
    --txt <fields>      TXT input. Short for '--format txt --fields <fields>'

Output options:
    -o, --output <f>    Write output to <file> instead of STDOUT [default: -].
    --no-out            No output at all
    --outformat <fmt>   Output format and compression. See --format. Only needed
                        if not guessed from the extension (default: input format).
    --wrap <width>      Wrap FASTA sequences to maximum <width> characters
    --out-delim <d>     TXT/CSV delimiter. Defaults: '\t' for txt; ',' for csv
    --outfields <f>     TXT/CSV fields (variables allowed). [default: id,seq,desc]
    --to-fa             FASTA output. Short for: '--outformat fasta'
    --to-fq             FASTQ output. Short for: '--outformat fastq'
    --to-csv <fields>   CSV output. Short for '--outformat csv --outfields <f>'
    --to-txt <fields>   TXT output. Short for '--outformat txt --outfields <f>'

Attribute options:
    -a, --attr <a>      Add an attribute in the form name=value to FASTA/FASTQ
                        headers (multiple '-a key=value' args possible)
    --adelim <delim>    Attribute delimiter inserted before. If not a space,
                        attributes are appended to the ID (default: ' ')
    --aval-delim <d>    Delimiter between attribute names and values [default: =]

Associated lists:
    -l, --list <path>   Path to list with metadata (multiple -l args possible)
    --ldelim <delim>    Delimiter for list [default: \t]
    --lheader           List contains a header row. Automatically enabled if
                        variables in the form {l:<name>} are found.
    --id-col <no>       ID column number [default: 1]
    -u, --unordered     Allow lists to in different order than sequences.
    -m, --missing       Allow missing rows with '-u'. Variable output is empty.

General Information:
    -v, --verbose       Print more detailed information.
    -h, --help          Display this message
    --help-vars         List and explain all available variables

Advanced Options:
    --buf-cap <size>    Initial capacity of internal reader buffer [default: 68K]
    --max-mem <size>    Buffer size limit. Larger sequences will cause an error.
                        [default: 1G]
    --read-thread       Read from a different thread. Enabled with compressed input.
    --write-thread      Write in a different thread. Enabled with compressed output.
    --read-tbufsize S   Buffer size of threaded reader [default: 4M].
    --write-tbufsize S  Buffer size of threaded reader [default: 4M].
")}


macro_rules! command_list { () => ("
    pass        No processing done, useful for converting and attribute setting
    .           shorthand for 'pass'

Information about sequences
    count       Returns the sequence count
    stat        Per-sequence statistics

Subsetting / shuffling sequences
    head        Return the first N sequences
    tail        Return the last N sequences
    slice       Get a slice of the sequences within a defined range
    sample      Get a random subset of sequences
    filter      Filter based on different criteria
    split       Distribute sequences into multiple files

Searching and replacing
    find        Find one or more patterns with optional filtering/replacement
    replace     Fast pattern replacement

Modifying commands
    set         Set a new sequence and/or header
    del         Delete description fields and/or attributes
    trim        Trim sequences on the left and/or right
    mask        Soft or hard mask sequence ranges
    upper       Convert sequences to uppercase
    lower       Convert sequences to lowercase (soft mask)
    revcomp     Reverse complement DNA sequences.

For information about how to use a command use
    seqtool <command> -h/--help

List and explain available variables:
    seqtool --help-vars
    seqtool <command> --help-vars

")}

pub static USAGE: &'static str = concat!("
Tool for processing of biological sequences. It can read and write the formats
FASTA, FASTQ and CSV/TXT.

Usage:
    seqtool <command> [<opts>...]
    seqtool <command> (-h | --help)
    seqtool <command> --help-vars
    seqtool [options]

Options:
    -h, --help    Display this message
    --version     Print version info and exit
    --help-vars   Display variables accepted by all commands

Commands:
", command_list!());
