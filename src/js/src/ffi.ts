declare var GLctx: WebGLRenderingContext;
declare var GL: { textures: WebGLTexture[] };
declare var ENV: { [name: string]: any };

namespace CodeVisual.ffi {
    export function init_css(css: string) {
        $(document.head).append($("<style>" + css + "</style>"));
    }
    export function init_html(html: string) {
        $placeholder.html(html);

        $loadingScreen = $placeholder.find(".loading-screen");
        $gameScreen = $placeholder.find(".game-screen");
        $failedScreen = $placeholder.find(".failed-screen");

        $canvas = $placeholder.find("canvas");
        canvas = $canvas[0] as HTMLCanvasElement;
    }
    export function before_main_loop() {
        $loadingScreen.hide();
        $gameScreen.show();
        setInterval(() => {
            canvas.width = $canvas.width() / canvasScaling;
            canvas.height = $canvas.height() / canvasScaling;
            // GLctx.viewport(0, 0, canvas.width, canvas.height);
        }, 500);
    }
}