call make-web.bat
rsync -avz --delete ../target/web/asmjs/codewars2017/* pi@pi.kuviman.com:/home/pi/codevisual/asmjs/codewars2017
