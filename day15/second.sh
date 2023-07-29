cat $1 | sed 's/./& /g' > /tmp/second 
octave -q second.m 2> /dev/null
grep -v '#' expanded | sed 's/ //g' | grep -v '^$'
