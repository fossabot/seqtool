#!/bin/bash

cargo build --features=exprtk

seqtool=target/debug/st
wiki=../seqtool.wiki
main=_README.md

# prepend table of contents if there are H3 headings
prepend_toc() {
  contents=$1
  toc=`grep '^### ' $contents |
    sed -E 's/^### (.*)/* [\1]   #\1/g' |
    awk -F'   ' '{ gsub(" ", "-", $2); rep=gsub("[()]", "", $2); print sprintf("%s(%s)", $1, tolower($2)) }'`

  if [ `printf "$toc" | wc -l` -gt 1 ]; then
    printf "## Contents\n\n$toc\n\n" | cat - $contents > tmp_out
    mv tmp_out $contents
  fi
}


cat doc/_head.md > $main

# generate command files

printf "\n## Commands" >> $main

cmd=(
  ">Basic conversion / editing" pass
  ">Information about sequences" view count stat
  ">Subsetting/shuffling sequences" head tail slice sample filter split interleave
  ">Searching and replacing" find replace
  ">Modifying commands" del set trim mask upper lower revcomp concat
)

# create one MD file per command

for c in "${cmd[@]}"; do
  echo "$c"

  if [[ "$c" = ">"* ]]; then
    # category name
    c=$(echo "$c" | cut -c2-)
    printf "\n### $c\n" >> $main
    continue
  fi

  out=$wiki/$c.md
  echo -n > $out

  opts=$($seqtool "$c" -h 2>&1 | sed -n '/Input options/q;p')
  desc=$(echo "$opts" | sed -n '/Usage:/q;p')

  # add command to overview
  echo "* **[$c](wiki/$c)**: $desc" >> $main

  # add custom help content if file exists in doc dir
  desc_f=doc/$c.md
  if [ -f $desc_f ]; then
    echo "## Description" >> $out
    cat $desc_f >> $out
  fi

  # add variable help if present
  vars=$($seqtool $c --help-vars 2>&1 | sed -n '/Standard variables/q;p' )
  if [ ! -z "$vars" -a "$vars" != " "  ] && [[ "$vars" != Invalid* ]]; then
    printf "\n\n### Provided variables\n\`\`\`\n$vars\n\`\`\`\n\n" >> $out
  fi

  prepend_toc $out

  # prepend usage info
  usage=$(echo "$opts" | sed '/Usage:/,$!d' | sed 's/\[-p <prop>\.\.\.\] *\[-l <list>\.\.\.\]//g')
  printf "$desc\n\n\`\`\`\n$usage\n\`\`\`\n\n[See this page](opts) for the options common to all commands.\n\n" |
    cat - $out > tmp_out
    mv tmp_out $out

done


echo >> $main
cat doc/_desc.md >> $main

# variables
out=$wiki/variables.md
cp doc/variables.md $out
printf "\n## List of variables\n" >> $out
$seqtool --help-vars 2>&1 | scripts/parse_varhelp.py >> $out

# global opts
out=$wiki/opts.md
printf "\n\n### Options recognized by all commands\n\n" > $out
echo "\`\`\`" >> $out
$seqtool . -h 2>&1 | sed '/Input options:/,$!d' >> $out
echo "\`\`\`" >> $out

# other files

cp doc/lists.md doc/ranges.md doc/attributes.md $wiki

# replace URLs in readme

sed -E 's|wiki/|https://github.com/markschl/seqtool/wiki/|g' $main > README.md
cp README.md $wiki/Home.md
rm $main
