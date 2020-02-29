#!/usr/bin/env bash

./quine.cat | diff -u - quine.cat
python quine.py | diff -u - quine.py
rustc quine.rs && ./quine | diff -u - quine.rs
rustc quine-rustc.rs 2>&1 | diff -u - quine-rustc.rs
