#!/bin/bash
for a in $(cat $1) ; do 
    for b in $(cat $1) ; do 
      if [[ "$a" != "$b" ]] ; then
        (echo $a ; echo $b) | target/debug/first | tail -1 ; 
        (echo $b ; echo $a) | target/debug/first | tail -1 ; 
      fi
    done ; 
 done | sort -n | tail -1 
