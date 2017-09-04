namespace CodeWars {
    export function stream_download(path: string, callback: (addr: number) => void) {
        let xhr = new XMLHttpRequest();
        xhr.open('GET', path);
        let responsePos = 0;
        let line_start = 0;
        let finished = false;
        let lines: string[] = [];

        const MAX_LINES_AT_ONCE = 3;
        const MAX_BYTES_AT_ONCE = 10 * 1024;
        let text = "";

        function checkXHR() {
            if (xhr.readyState == 3 || xhr.readyState == 4) {
                let checkedCur = 0;
                while (responsePos < text.length && checkedCur < MAX_BYTES_AT_ONCE) {
                    if (text[responsePos++] == '\n') {
                        lines.push(text.substring(line_start, responsePos));
                        line_start = responsePos;
                    }
                    checkedCur++;
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
            if (!finished) {
                setTimeout(checkXHR, 0);
            }
        }

        xhr.addEventListener("error", function (e) {
            console.error("error: " + e);
        });
        xhr.send();
        let lastLine = 0;

        let buf_addr: number = 0;
        let buf_len: number = 0;

        function update() {
            if (lines.length - lastLine >= MAX_LINES_AT_ONCE || finished) {
                let updated: number = 0;
                while (lastLine < lines.length && updated < MAX_LINES_AT_ONCE) {
                    let line = lines[lastLine];
                    let Module = (window as any).Module;
                    if (line.length + 1 > buf_len) {
                        if (buf_len != 0) {
                            Module._free(buf_addr);
                        }
                        buf_len = line.length + 1;
                        buf_addr = Module._malloc(buf_len);
                    }
                    Module.writeAsciiToMemory(line, buf_addr);
                    callback(buf_addr);
                    updated++;
                    lines[lastLine++] = undefined;
                }
            }
            if (!finished || lastLine < lines.length) {
                setTimeout(update, (finished || lines.length - lastLine >= MAX_LINES_AT_ONCE) ? 0 : 50);
            } else {
                let Module = (window as any).Module;
                if (buf_len != 0) {
                    Module._free(buf_addr);
                }
            }
        }

        checkXHR();
        update();
    }

    export function set_loaded_percent(percent: number) {
        $(".timeline-loaded").width(percent + "%");
    }
}