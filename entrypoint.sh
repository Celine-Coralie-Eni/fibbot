#!/bin/sh -l

results=$(echo "$@" | xargs fibbot)
echo "results=$results" >> $GITHUB_OUTPUT
