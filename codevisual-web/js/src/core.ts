declare var GLctx: WebGLRenderingContext;
declare var GL: { textures: WebGLTexture[] };

namespace Module {
    export let canvas: HTMLCanvasElement;

    export function printErr(s: string) {
        if (s.indexOf("bad name in getProcAddress: ") == 0) {
            return;
        }
        console.error(s);
    }
}

namespace CodeVisual {
    export let $player: JQuery;

    let $loadingScreen: JQuery;
    let $gameScreen: JQuery;
    let $failedScreen: JQuery;
    let $help: JQuery;
    let $helpContent: JQuery;

    export namespace internal {
        export const on_init = [] as [() => void];
        export const on_before_main_loop = [] as [() => void];

        export function toggleWidget(widget: JQuery) {
            $player.find(".widget").each(function (this: any) {
                if (!$(this).is(widget)) {
                    $(this).hide();
                }
            });
            widget.slideToggle();
        }

        export function init() {
            $player = $(".codevisual-player");

            $loadingScreen = $player.find(".loading-screen");
            $gameScreen = $player.find(".game-screen");
            $failedScreen = $player.find(".failed-screen");

            $help = $player.find(".help");
            $helpContent = $help.find(".content");
            $player.find(".help-button").click(function () {
                toggleWidget($help);
            });

            for (let f of on_init) {
                f();
            }
        }

        export function before_main_loop() {
            $('[data-toggle="tooltip"]').tooltip().click(function (this: any) {
                $(this).tooltip("hide");
                return true;
            });
            $loadingScreen.fadeOut();
            $gameScreen.show();
            for (let f of on_before_main_loop) {
                f();
            }
        }

        export function show_error(error: string) {
            console.error(error);
            $player.addClass("error");
            $loadingScreen.hide();
            $gameScreen.hide();
            $failedScreen.show();
            $failedScreen.find(".error-message").text(error);
        }

        export function set_help_html(html: string) {
            $helpContent.html(html);
        }

        export function load_texture(path: string, texture_handle: number, on_load: (width: number, height: number) => void) {
            let texture = GL.textures[texture_handle];
            let image = new Image();
            image.onload = function () {
                let cur = GLctx.getParameter(GLctx.TEXTURE_BINDING_2D);
                GLctx.bindTexture(GLctx.TEXTURE_2D, texture);
                GLctx.texImage2D(GLctx.TEXTURE_2D, 0, GLctx.RGBA, GLctx.RGBA, GLctx.UNSIGNED_BYTE, image);
                GLctx.bindTexture(GLctx.TEXTURE_2D, cur);
                on_load(image.width, image.height);
            };
            image.onerror = function () {
                show_error("Error downloading '" + path + "'");
            };
            image.src = path;
        }

        export function set_load_progress(loaded_count: number, total_count: number) {
            $player.find(".resource-loading-progress-bar").width(loaded_count * 100 / total_count + "%");
        }
    }
}