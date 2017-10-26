namespace CodeVisual {
    let $canvas: JQuery;

    export namespace internal {
        export let canvas: HTMLCanvasElement;

        on_init.push(() => {
            $canvas = $player.find("canvas");
            $canvas.on("contextmenu", () => false);
            canvas = $canvas[0] as HTMLCanvasElement;
            Module.canvas = canvas;

            let scalingSettingValue = 1;
            setInterval(() => {
                canvas.width = $canvas.width() / scalingSettingValue;
                canvas.height = $canvas.height() / scalingSettingValue;
                GLctx.viewport(0, 0, canvas.width, canvas.height);
            }, 300);

            // const scalingSetting = new NumberSetting("Canvas scale", 1, 4, 1, 0.01);
            // settings.add(scalingSetting);
            // setInterval(() => {
            //     scalingSettingValue = scalingSetting.value;
            // }, 300);
        });

        export function set_cursor(cursor: string) {
            canvas.style.cursor = cursor;
        }
    }
}