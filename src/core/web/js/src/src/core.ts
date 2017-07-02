declare var GLctx: WebGLRenderingContext;
declare var GL: { textures: WebGLTexture[] };
declare var ENV: { [name: string]: any };

declare namespace Module {
    export let canvas: HTMLCanvasElement;
    export function printErr(s: string): void;
}

Module.printErr = function (s: string) {
    if (s.indexOf("bad name in getProcAddress: ") == 0) {
        return;
    }
    console.error(s);
}

namespace CodeVisual {
    const $placeholder = $("#codevisual");

    export let $player: JQuery;

    let $loadingScreen: JQuery;
    let $gameScreen: JQuery;
    let $failedScreen: JQuery;

    export namespace internal {
        export const on_init = [] as [() => void];
        export function init(html: string, css: string) {
            $placeholder.html(html);
            $(document.head).append($("<style>" + css + "</style>"));

            $player = $placeholder.find(".codevisual-player");

            $loadingScreen = $player.find(".loading-screen");
            $gameScreen = $player.find(".game-screen");
            $failedScreen = $player.find(".failed-screen");

            for (let f of on_init) {
                f();
            }
        }
        export function before_main_loop() {
            $loadingScreen.hide();
            $gameScreen.show();
        }
        export function show_error(json: any) {
            console.error(json);
            $player.addClass("error");
            $loadingScreen.hide();
            $gameScreen.hide();
            $failedScreen.show();
            let message = json.error;
            if (json.location) {
                message = "@" + json.location.file + "(line " + json.location.line + "): " + message;
            }
            $failedScreen.find(".error-message").text(message);
        }
        export function load_texture(path: string, texture_handle: number, on_load: () => void) {
            let texture = GL.textures[texture_handle];
            let image = new Image();
            image.onload = function () {
                var cur = GLctx.getParameter(GLctx.TEXTURE_BINDING_2D);
                GLctx.bindTexture(GLctx.TEXTURE_2D, texture);
                GLctx.texImage2D(GLctx.TEXTURE_2D, 0, GLctx.RGBA, GLctx.RGBA, GLctx.UNSIGNED_BYTE, image);
                GLctx.texParameteri(GLctx.TEXTURE_2D, GLctx.TEXTURE_MIN_FILTER, GLctx.NEAREST);
                GLctx.texParameteri(GLctx.TEXTURE_2D, GLctx.TEXTURE_MAG_FILTER, GLctx.NEAREST);
                GLctx.bindTexture(GLctx.TEXTURE_2D, cur);
                on_load();
            };
            image.src = path;
        }
        export function set_load_progress(loaded_count: number, total_count: number) {
            $player.find(".resource-loading-progress-bar").width(loaded_count * 100 / total_count + "%");
        }
    }
}