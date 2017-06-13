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
    export const $placeholder = $("#codevisual");

    export let $player: JQuery;

    export let $loadingScreen: JQuery;
    export let $gameScreen: JQuery;
    export let $failedScreen: JQuery;

    export let $canvas: JQuery;
    export let canvas: HTMLCanvasElement;

    export let canvasScaling = 1;

    function updateOrientation() {
        let screen = window.screen as any;
        if (isFullscreen()) {
            screen.lockOrientationUniversal = screen.lockOrientation || screen.mozLockOrientation || screen.msLockOrientation;
            if (screen.lockOrientationUniversal) {
                screen.lockOrientationUniversal("landscape");
            } else {
                screen.orientation.lock("landscape").catch(function () { });
            }
        } else {
            screen.unlockOrientationUniversal = screen.unlockOrientation || screen.mozUnlockOrientation || screen.msUnlockOrientation;
            if (screen.unlockOrientationUniversal) {
                screen.unlockOrientationUniversal();
            } else {
                screen.orientation.unlock();
            }
        }
    }
    setInterval(updateOrientation, 300);

    function goFullscreen(elem: any) {
        if (elem.requestFullscreen) {
            elem.requestFullscreen();
        } else if (elem.msRequestFullscreen) {
            elem.msRequestFullscreen();
        } else if (elem.mozRequestFullScreen) {
            elem.mozRequestFullScreen();
        } else if (elem.webkitRequestFullscreen) {
            elem.webkitRequestFullscreen();
        } else {
            return;
        }
    };

    function cancelFullscreen() {
        let document = window.document as any;
        if (document.cancelFullScreen) {
            document.cancelFullScreen();
        } else if (document.msExitFullscreen) {
            document.msExitFullscreen();
        } else if (document.mozCancelFullScreen) {
            document.mozCancelFullScreen();
        } else if (document.webkitCancelFullScreen) {
            document.webkitCancelFullScreen();
        } else {
            return;
        }
    };

    export function isFullscreen(): boolean {
        let document = window.document as any;
        return document.fullscreenElement ||
            document.mozFullScreenElement ||
            document.webkitFullscreenElement ||
            document.msFullscreenElement;
    };

    export function toggleFullscreen() {
        if (isFullscreen()) {
            cancelFullscreen();
        } else {
            goFullscreen($player[0]);
        }
        updateOrientation();
    }


    export namespace internal {
        export function init_css(css: string) {
            $(document.head).append($("<style>" + css + "</style>"));
        }
        export function init_html(html: string) {
            $placeholder.html(html);

            $player = $placeholder.find(".codevisual-player");

            $loadingScreen = $player.find(".loading-screen");
            $gameScreen = $player.find(".game-screen");
            $failedScreen = $player.find(".failed-screen");

            $canvas = $player.find("canvas");
            $canvas.on("contextmenu", () => false);
            canvas = $canvas[0] as HTMLCanvasElement;

            Module.canvas = canvas;

            $gameScreen.append(stats.dom);
            $gameScreen.find(".fullscreen-button").click(function () {
                toggleFullscreen();
            });
        }
        export function before_main_loop() {
            $loadingScreen.hide();
            $gameScreen.show();
            setInterval(() => {
                canvas.width = $canvas.width() / canvasScaling;
                canvas.height = $canvas.height() / canvasScaling;
                GLctx.viewport(0, 0, canvas.width, canvas.height);
            }, 300);
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
        export function load_texture(args: { path: string, texture_handle: number }) {
            let texture = GL.textures[args.texture_handle];
            let image = new Image();
            image.onload = function () {
                var cur = GLctx.getParameter(GLctx.TEXTURE_BINDING_2D);
                GLctx.bindTexture(GLctx.TEXTURE_2D, texture);
                GLctx.texImage2D(GLctx.TEXTURE_2D, 0, GLctx.RGBA, GLctx.RGBA, GLctx.UNSIGNED_BYTE, image);
                GLctx.texParameteri(GLctx.TEXTURE_2D, GLctx.TEXTURE_MIN_FILTER, GLctx.NEAREST);
                GLctx.texParameteri(GLctx.TEXTURE_2D, GLctx.TEXTURE_MAG_FILTER, GLctx.NEAREST);
                GLctx.bindTexture(GLctx.TEXTURE_2D, cur);
            };
            image.src = args.path;
        }
    }
}