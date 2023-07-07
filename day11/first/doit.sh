#!/bin/bash
rm 0*.pbm
cargo run < ../input | sed 's/./& /g' | awk -v h='\nP2\n10 10\n10' '{if (block==0 || NF==0){printf("%s",h);if(block==0) printf("\n");++block};print $0}' | ./chonk.sh
convert -delay 20 -scale 5000% -loop 0 0*.pbm animate.gif 
rm 0*.pbm
