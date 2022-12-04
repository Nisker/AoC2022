#!/bin/bash
#Quick bashscript to find the elf with most calories in bag
cat input | sed -z 's/\n/+/g' | sed 's/++/\n/g' | bc | sort -n | tail -1
