#!/bin/bash
find codewars2017 run examples src "(" -name "*.rs" -o -name "*.ts" -o -name "*.less" -o -name "*.pug" -o -name "*.html" ")" "!" -name "index.d.ts" | xargs wc -l -w
