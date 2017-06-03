namespace CodeVisual {
    export const $placeholder = $("#codevisual");

    export let $player: JQuery;

    export let $loadingScreen: JQuery;
    export let $gameScreen: JQuery;
    export let $failedScreen: JQuery;

    export let $canvas: JQuery;
    export let canvas: HTMLCanvasElement;

    export let canvasScaling = 1;

    export const stats = new Stats();

    setInterval(function () {
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
    }, 500);

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
    }
}