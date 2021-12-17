#!/bin/bash

for file in *.txt; do
  cat $file | cargo run -- > ${file/txt/res}
done
