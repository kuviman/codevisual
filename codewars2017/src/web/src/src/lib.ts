namespace CodeWars {
    let currentTick: number = 0;
    let paused: boolean = true;

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
        let downloadedTicks = 0;

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
            downloadedTicks++;
        }

        function update() {
            if (xhr.readyState == 3 || xhr.readyState == 4) {
                let maxPos = Math.min(text.length, responsePos + MAX_BYTES_AT_ONCE);
                let lineLimit = MAX_LINES_AT_ONCE;
                let left = downloadedTicks - currentTick;
                while (left > 100 && lineLimit > 1 && !paused) {
                    left -= 100;
                    lineLimit--;
                }
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

    export function set_playback_position(tick: number, tickCount: number) {
        currentTick = tick;
        $(".timeline-position").css("left", tick * 100 / tickCount + "%");
    }

    export function set_play_button_callback(callback: () => void) {
        $(".play-stop-button").click(function () {
            callback();
        });
    }

    export function set_paused(new_paused: boolean) {
        paused = new_paused;
        let $glyph = $(".play-stop-button .glyphicon");
        let PAUSE_ICON = "glyphicon-pause";
        let PLAY_ICON = "glyphicon-play";
        $glyph.removeClass(paused ? PAUSE_ICON : PLAY_ICON);
        $glyph.addClass(paused ? PLAY_ICON : PAUSE_ICON);
    }

    export function set_timeline_callback(callback: (pos: number) => void) {
        function set_pos(e: any, elem: any) {
            let pos = (e.pageX - elem.offset().left) / elem.width();
            callback(Math.round(1000 * pos));
        }

        let $timeline = $(".timeline");

        function handler(this: any, e: any) {
            // if (e.buttons as number & 1) {
            set_pos(e, $timeline);
            // }
            return false;
        }

        $timeline.click(handler);

        // TODO: not working on mobile
        // $timeline.mousecapture({
        //     "down": handler,
        //     "move": handler
        // });
    }

    export function init_overlay(html: string, css: string) {
        $("<style>" + css + "</style>").appendTo("head");
        $(".codevisual-player .game-screen").append($(html));
    }

    export function set_scores(score1: number, score2: number) {
        $(".codewars-overlay .score-1").text(score1.toString());
        $(".codewars-overlay .score-2").text(score2.toString());
    }
}