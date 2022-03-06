#!/usr/bin/env bash
# test file that compares out file with test case *output.txt
# assumes all test cases have output, description and input file
# compares using diff command, files are exact with white space

desc=(*_description.txt)
tot=${#desc[@]}
inputs=(*_inputs.txt)
outputs=(*_outputs.txt)
RED=$(tput setaf 1)
GREEN=$(tput setaf 2)
NORMAL=$(tput sgr0)

for i in $(seq 0 $tot); do
  # check if no more files are left
  if test -z "${desc[$i]}"
  then
    continue
  else
    # run exe
    ../target/debug/./A4 "${desc[$i]}" "${inputs[$i]}" out.txt

    # check if differ
    if `diff --brief out.txt "${outputs[$i]}"`
    then
      echo "${GREEN}Passed ${NORMAL} ${outputs[$i]}"
    else
      echo "${RED}Failed ${NORMAL} ${outputs[$i]}"
    fi
  fi
done