# Explanation of ranges

Ranges in seqtool are used by commands like [trim](trim) and [mask](mask),
and returned by the [find](find) command.

They look like this: `<start>..<end>`. Open ranges are possible: `<start>..`
(trims only at the start, including the end of the sequence) and `..<end>`
(trims only at the end, including the start of the sequence to `<end>`).
The coordinates are 1-based, meaning that `1` denotes the first character
(unless `-0` is used). It is also possible to use negative numbers, which
will tell the tool to count from the end of the sequence:

<pre>
sequence:    A   T  <b>G   C   A   T</b>   G   C
from start:  1   2  <b>3   4   5   6</b>   7   8
from end:   -8  -7 <b>-6  -5  -4  -3</b>  -2  -1
0-based:     0   1  <b>2   3   4   5</b>   6   7
</pre>

In this example, the following commands will trim output the range printed in bold
letters.

```bash
# 1-based positive
seqtool trim '3..6' seqs.fa

# 1-based negative
# space before range and quote necessary
seqtool trim ' -6..-3' seqs.fa.

# 0-based
seqtool trim 2..6 seqs.fa
```

**Note**: There is a problem with ranges starting with a negative number
being interpreted as command line arguments. However, insertion of a
space before the minus sign like this will work.


#### Empty ranges

Note that ranges of zero length are only possible if
the start is greater than the end, e.g.: `5..4`.
