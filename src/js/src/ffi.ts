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

namespace CodeVisual.ffi {
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
        }, 500);
    }
    export function error(json: any) {
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