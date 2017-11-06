/// <reference path="src/core.ts" />
/// <reference path="src/fullscreen.ts" />
/// <reference path="src/settings.ts" />
/// <reference path="src/stats.ts" />
/// <reference path="src/canvas.ts" />
/// <reference path="src/controls.ts" />
/// <reference path="src/sound.ts" />

(window as any).CodeVisual = CodeVisual;
(window as any).Module = Module;
$(() => $.getScript("code.js"));