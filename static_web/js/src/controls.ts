namespace CodeVisual {
    export namespace internal {
        const TIMEOUT = 10000;
        let $controls: JQuery;
        let shown = true;
        let timeout: number;

        function hideControls() {
            if ($player.find(".widget").is(":visible")) {
                timeout = setTimeout(hideControls, TIMEOUT);
                return;
            }
            if (shown) {
                $controls.animate({
                    bottom: "-32px"
                });
                shown = false;
                $player.find(".widget").hide();
            }
        }

        function showControls() {
            if (!shown) {
                $controls.animate({
                    bottom: 0
                });
                shown = true;
            }
            if (timeout) {
                clearTimeout(timeout);
            }
            timeout = setTimeout(hideControls, TIMEOUT);
        }

        on_before_main_loop.push(() => {
            $controls = $player.find(".controls");
            timeout = setTimeout(hideControls, TIMEOUT);
            $player.on("mousemove touchmove touchstart mousedown", () => {
                showControls();
            });
        });
    }
}