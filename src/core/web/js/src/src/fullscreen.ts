namespace CodeVisual {
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

    function toggleFullscreen() {
        if (isFullscreen()) {
            cancelFullscreen();
        } else {
            goFullscreen($player[0]);
        }
        updateOrientation();
    }

    internal.on_init.push(() => {
        $player.find(".fullscreen-button").click(function () {
            toggleFullscreen();
        });
    });
}