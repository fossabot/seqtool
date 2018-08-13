# Including metadata (lists)

It often occurs that information from text files (created manually or
using another program) needs to be associated with sequences.
Such metadata can be included using the `-l` / `--list`
option. Sequence IDs are expected to be in the first column. If not,
specify `--id-col`.

Fields are accessible as [variables](variables) in this form: `l:<column_index>`
or `l:<column_name>`. Consider this list containing taxonomic information about
sequences (_taxonomy.txt_):

```
id  lineage
seq1  d:Bacteria,p:"Actinobacteria",c:Actinobacteria,o:Actinomycetales,f:Actinomycetaceae,g:Actinomyces;
seq2  d:Bacteria,p:"Actinobacteria",c:Actinobacteria,o:Actinomycetales,f:Pseudonocardiaceae,g:Amycolatopsis;
(...)
```

The lineages can be added to the FASTA header using this command:

```bash
st set -l taxonomy.txt -d {l:lineage} seqs.fa > seqs_with_taxonomy.fa
```
*seqs_with_taxonomy.fa:*
```
>seq1 d:Bacteria,p:"Actinobacteria",c:Actinobacteria,o:Actinomycetales,f:Actinomycetaceae,g:Actinomyces;
SEQUENCE
>seq2 d:Bacteria,p:"Actinobacteria",c:Actinobacteria,o:Actinomycetales,f:Pseudonocardiaceae,g:Amycolatopsis;
SEQUENCE
(...)
```

By default IDs in the text file (first column) are expected to be in the same order
as sequence IDs. If not, there will be an error message:

```
ID mismatch: expected 'seq1' but found 'seq12'. Use -u/--unordered if sequences and lists are not in same order.
```
Therefore we add `-u`. This will consume more memory, especially if the list file
is large.

```bash
st set -ul taxonomy.txt -d {l:lineage} seqs.fa > seqs_with_taxonomy.fa
```

Additionally, the tool expects all IDs to be present in associated list.
If this is not true, it is necessary to explicitly to specify `-m/--missing`.

### Filtering given an ID list

It is possible to keep/exclude sequences based on whether they occur in a
list or not. This can be achieved by using the [filter command](filter#undefined-values).
The `def()` function returns `true` if the ID (in the first column here)
is present.

```bash
st filter -uml id_list.txt "def(l:1)" seqs.fa > in_list.fa
```
