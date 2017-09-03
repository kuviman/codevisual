namespace CodeWars {
    export function stream_download(path: string, callback: (addr: number) => void) {
        let xhr = new XMLHttpRequest();
        xhr.open('GET', path);
        let responsePos = 0;
        let line_start = 0;
        let finished = false;
        let lines: string[] = [];

        const MAX_LINES_AT_ONCE = 10;
        let text = "";

        function checkXHR() {
            if (xhr.readyState == 3 || xhr.readyState == 4) {
                let checkedCur = 0;
                while (responsePos < text.length) {
                    let pos = text.indexOf('\n', responsePos);
                    if (pos < 0) {
                        responsePos = text.length;
                        break;
                    }
                    responsePos = pos + 1;
                    if (text[pos] == '\n') {
                        lines.push(text.substring(line_start, pos));
                        line_start = pos + 1;
                        if (checkedCur++ >= MAX_LINES_AT_ONCE) {
                            break;
                        }
                    }
                }
                if (responsePos == text.length) {
                    text = xhr.responseText;
                    if (responsePos == text.length && xhr.readyState == 4) {
                        if (line_start < text.length) {
                            lines.push(text.substring(line_start));
                        }
                        finished = true;
                    }
                }
            }
        }

        xhr.addEventListener("error", function (e) {
            console.error("error: " + e);
        });
        xhr.send();
        let lastLine = 0;

        function update() {
            if (lines.length - lastLine < MAX_LINES_AT_ONCE) {
                checkXHR();
            }
            let updated: number = 0;
            while (lastLine < lines.length && updated < MAX_LINES_AT_ONCE) {
                let line = lines[lastLine];
                let Module = (window as any).Module;
                let addr = Module._malloc(line.length + 1);
                Module.writeAsciiToMemory(line, addr);
                callback(addr);
                updated++;
                lines[lastLine++] = undefined;
            }
            if (!finished || lastLine < lines.length) {
                setTimeout(update, 0);
            }
        }

        update();
    }

    export function set_loaded_percent(percent: number) {
        $(".timeline-loaded").width(percent + "%");
    }
}