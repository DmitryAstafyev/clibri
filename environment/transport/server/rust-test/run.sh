#!/bin/sh
exec 2>&1 # redirect output of stderr to stdout 
ulimit -n 409600
exec ./target/release/server-test