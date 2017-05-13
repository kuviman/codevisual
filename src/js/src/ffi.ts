declare var GLctx: WebGLRenderingContext;
declare var GL: { textures: WebGLTexture[] };
declare var ENV: { [name: string]: any };

namespace CodeVisual.ffi {
    export function init_css(css: string) {
        $(document.head).append($("<style>" + css + "</style>"));
    }
    export function init_html(html: string) {
        $placeholder.html(html);
    }
}