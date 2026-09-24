#!/usr/bin/env bash

clang -arch arm64 cat.S -o cata || exit 1 

test_string="Hello, World!"
result=$(echo $test_string | ./cat)

if [[ $result == $test_string ]]; then
  echo "🥳 TEST PASSED"
else
  echo "💩 TEST FAILED"
  echo "Expected: $test_string"
  echo "Got:      $result"
fi
