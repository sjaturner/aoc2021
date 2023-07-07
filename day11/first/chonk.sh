#!/bin/bash
chonk=0
while read line ; do
  if [[ $line == "" ]]; then
    chonk=$(( $chonk + 1 ))
    continue
  fi
  echo $line >> $(printf "%04d.pbm" $chonk)
done
