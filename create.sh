#!/bin/bash

filename=$1

if [ -f $filename ]; then
    echo "no file created"
    exit 1
fi

cp template.rs src/bin/$filename.rs
echo "File created"
sed -i "s/__DAY__/$filename/g" src/bin/$filename.rs
echo "Templates renamed"
