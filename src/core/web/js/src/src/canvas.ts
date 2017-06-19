namespace CodeVisual {
    let $canvas: JQuery;

    export namespace internal {
        export let canvas: HTMLCanvasElement;

        internal.on_init.push(() => {
            const scalingSetting = new NumberSetting("Canvas scale", 1, 4, 1, 0.01);
            settings.add(scalingSetting);
            $canvas = $player.find("canvas");
            $canvas.on("contextmenu", () => false);
            canvas = $canvas[0] as HTMLCanvasElement;
            Module.canvas = canvas;
            setInterval(() => {
                canvas.width = $canvas.width() / scalingSetting.value;
                canvas.height = $canvas.height() / scalingSetting.value;
                GLctx.viewport(0, 0, canvas.width, canvas.height);
            }, 300);
        });

        export function set_cursor(cursor: string) {
            canvas.style.cursor = cursor;
        }
    }
}