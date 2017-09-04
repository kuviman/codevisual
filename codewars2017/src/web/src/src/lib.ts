namespace CodeWars {
    export function stream_download(path: string, callback: (addr: number) => void) {
        let xhr = new XMLHttpRequest();
        xhr.open('GET', path);
        xhr.addEventListener("error", function (e) {
            console.error("error: " + e);
        });
        xhr.send();

        const MAX_BYTES_AT_ONCE = 16 * 1024;
        const MAX_LINES_AT_ONCE = 16;
        let responsePos = 0;
        let line_start = 0;
        let finished = false;
        let text = "";

        let buf_addr = 0;
        let buf_len = 0;
        function sendLine(line: string) {
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
        }

        function update() {
            if (xhr.readyState == 3 || xhr.readyState == 4) {
                let maxPos = Math.min(text.length, responsePos + MAX_BYTES_AT_ONCE);
                let lineLimit = MAX_LINES_AT_ONCE;
                while (responsePos < maxPos) {
                    if (text[responsePos++] == '\n') {
                        sendLine(text.substring(line_start, responsePos));
                        line_start = responsePos;
                        if (--lineLimit <= 0) {
                            break;
                        }
                    }
                }
                if (responsePos == text.length) {
                    text = xhr.responseText;
                    if (responsePos == text.length && xhr.readyState == 4) {
                        if (line_start < text.length) {
                            sendLine(text.substring(line_start));
                        }
                        finished = true;
                    }
                }
            }
            if (!finished) {
                setTimeout(update, 0);
            }
        }

        update();
    }

    export function set_loaded_percent(percent: number) {
        $(".timeline-loaded").width(percent + "%");
    }
}