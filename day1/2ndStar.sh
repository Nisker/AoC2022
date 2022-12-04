#!/bin/bash
#Quick bashscript to find the 3 elfs with most calories in their bag
cat input | sed -z 's/\n/+/g' | sed 's/++/\n/g' | bc | sort -n | tail -3 | awk '{sum += $1} END {print sum}'
