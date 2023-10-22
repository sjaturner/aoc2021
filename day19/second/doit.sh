#!/bin/bash
# Augment the input data with centers and see where those end up by comparing with the cloud without centers
cat <(cd ../first/ && cat ../input | cargo run --release) <(cd ../first/ && cat ../input | awk '{print $0; if("---" == $1){printf "0,0,0\n";}}' | cargo run --release) | sort | uniq -c | grep -v '^ *2' | xargs -n4 | cut -d' ' -f2- | cargo run
# TBH Quite pleased with that
