declare var GLctx: WebGLRenderingContext;
declare var GL: { textures: WebGLTexture[] };
declare var ENV: { [name: string]: any };

ENV.RUST_BACKTRACE = "1";

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
        export function show_error(error: string) {
            console.error(error);
            $player.addClass("error");
            $loadingScreen.hide();
            $gameScreen.hide();
            $failedScreen.show();
            $failedScreen.find(".error-message").text(error);
        }
        export function load_texture(path: string, texture_handle: number, on_load: (width: number, height: number) => void) {
            let texture = GL.textures[texture_handle];
            let image = new Image();
            image.onload = function () {
                var cur = GLctx.getParameter(GLctx.TEXTURE_BINDING_2D);
                GLctx.bindTexture(GLctx.TEXTURE_2D, texture);
                GLctx.texImage2D(GLctx.TEXTURE_2D, 0, GLctx.RGBA, GLctx.RGBA, GLctx.UNSIGNED_BYTE, image);
                GLctx.bindTexture(GLctx.TEXTURE_2D, cur);
                on_load(image.width, image.height);
            };
            image.src = path;
        }
        export function set_load_progress(loaded_count: number, total_count: number) {
            $player.find(".resource-loading-progress-bar").width(loaded_count * 100 / total_count + "%");
        }
    }
}