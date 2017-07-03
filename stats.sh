#!/bin/bash
find playground src "(" -name "*.rs" -o -name "*.ts" -o -name "*.less" -o -name "*.pug" -o -name "*.html" ")" "!" -name "index.d.ts" | xargs wc -l -w
