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
}