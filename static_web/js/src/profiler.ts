namespace CodeVisual.internal {
    let $info: JQuery;
    on_init.push(() => {
        $info = $player.find(".profiler-info");
        // settings.add(new BooleanSetting("Profiler", false, (show) => {
        //     if (show) {
        //         $info.show();
        //     } else {
        //         $info.hide();
        //     }
        // }));
    });

    export class ProfiledRegion {
        name: string;
        time_consumed: number;
        invocation_count: number;
        children: [ProfiledRegion];
    }

    let fps: number;

    function prettyPrint(region: ProfiledRegion, super_total: number, root: boolean): string {
        let result = "<div>";
        result += "<span>";
        if (root) {
            fps = region.invocation_count;
            result += "FPS: " + fps;
        } else {
            result += (100.0 * region.time_consumed / super_total).toFixed(2) + "%";
            let nanos = Math.round(region.time_consumed * 1000000.0 / fps);
            result += " (avg " + Math.floor(nanos / 1000.0) + "." + Math.floor(nanos % 1000 / 100.0) + " ms)";
            result += " - " + region.name;
        }
        result += "</span>";
        region.children.sort((a, b) => b.time_consumed - a.time_consumed);
        for (let child of region.children) {
            result += prettyPrint(child, region.time_consumed, false);
        }
        result += "</div>";
        return result;
    }

    export function profiler_data(data: ProfiledRegion) {
        $info.html(prettyPrint(data, data.time_consumed, true));
    }
}