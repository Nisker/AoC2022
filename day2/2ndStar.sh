#!/bin/bash
#using sed to replace the rock paper scissor to the output, and bc to calculate output
cat inpit | sed -z -e 's/\n/+/g' -e 's/A X/3+0/g' -e 's/A Y/1+3/g' -e 's/A Z/2+6/g' -e 's/B X/1+0/g' -e 's/B Y/2+3/g' -e 's/B Z/3+6/g' -e 's/C X/2+0/g' -e 's/C Y/3+3/g' -e 's/C Z/1+6/g' -e 's/++/\n/g' | bc
