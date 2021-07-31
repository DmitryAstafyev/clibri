#!/bin/sh
npm run build
exec 2>&1 # redirect output of stderr to stdout 
ulimit -n 409600
exec node ./dist/index.js