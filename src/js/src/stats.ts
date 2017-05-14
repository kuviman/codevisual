// Original at https://github.com/mrdoob/stats.js/blob/master/src/Stats.js

namespace CodeVisual {
    export class Stats {
        dom: HTMLDivElement;
        private container: HTMLDivElement;
        private beginTime: number;
        private prevTime: number;
        private frames: number;
        private fpsPanel: Stats.Panel;
        private msPanel: Stats.Panel;
        private mode = 0;

        constructor() {
            let container = document.createElement('div');
            this.container = container;
            container.style.cssText = 'position:absolute;top:0;left:0;cursor:pointer;opacity:0.9;z-index:10000';
            container.addEventListener('click', event => {
                event.preventDefault();
                this.showPanel(++this.mode % container.children.length);

            }, false);

            this.beginTime = (performance || Date).now();
            this.prevTime = this.beginTime;
            this.frames = 0;

            this.fpsPanel = this.addPanel(new Stats.Panel('FPS', '#0ff', '#002'));
            this.msPanel = this.addPanel(new Stats.Panel('MS', '#0f0', '#020'));

            this.showPanel(0);

            this.dom = container;
        }

        addPanel(panel: Stats.Panel) {
            this.container.appendChild(panel.dom);
            return panel;
        }

        showPanel(id: number) {
            for (var i = 0; i < this.container.children.length; i++) {
                (this.container.children[i] as HTMLCanvasElement).style.display = i === id ? 'block' : 'none';
            }
            this.mode = id;
        }

        begin() {
            this.beginTime = (performance || Date).now();
        }

        end() {
            this.frames++;
            let time = (performance || Date).now();
            this.msPanel.update(time - this.beginTime, 200);
            if (time > this.prevTime + 1000) {
                this.fpsPanel.update((this.frames * 1000) / (time - this.prevTime), 100);
                this.prevTime = time;
                this.frames = 0;
            }
            return time;
        }

        update() {
            this.beginTime = this.end();
        }
    }

    export namespace Stats {
        let PR = Math.round(window.devicePixelRatio || 1);
        let WIDTH = 80 * PR, HEIGHT = 48 * PR,
            TEXT_X = 3 * PR, TEXT_Y = 2 * PR,
            GRAPH_X = 3 * PR, GRAPH_Y = 15 * PR,
            GRAPH_WIDTH = 74 * PR, GRAPH_HEIGHT = 30 * PR;

        export class Panel {
            private min = Infinity;
            private max = 0;
            private canvas: HTMLCanvasElement;
            private context: CanvasRenderingContext2D;
            dom: Element;
            constructor(private name: string, private bg: string, private fg: string) {
                let round = Math.round;

                let canvas = document.createElement('canvas');
                canvas.width = WIDTH;
                canvas.height = HEIGHT;
                canvas.style.cssText = 'width:80px;height:48px';

                let context = canvas.getContext('2d');
                context.font = 'bold ' + (9 * PR) + 'px Helvetica,Arial,sans-serif';
                context.textBaseline = 'top';

                context.fillStyle = bg;
                context.fillRect(0, 0, WIDTH, HEIGHT);

                context.fillStyle = fg;
                context.fillText(name, TEXT_X, TEXT_Y);
                context.fillRect(GRAPH_X, GRAPH_Y, GRAPH_WIDTH, GRAPH_HEIGHT);

                context.fillStyle = bg;
                context.globalAlpha = 0.9;
                context.fillRect(GRAPH_X, GRAPH_Y, GRAPH_WIDTH, GRAPH_HEIGHT);

                this.canvas = canvas;
                this.context = context;
                this.dom = canvas;
            }

            update(value: number, maxValue: number) {
                this.min = Math.min(this.min, value);
                this.max = Math.max(this.max, value);

                this.context.fillStyle = this.bg;
                this.context.globalAlpha = 1;
                this.context.fillRect(0, 0, WIDTH, GRAPH_Y);
                this.context.fillStyle = this.fg;
                this.context.fillText(Math.round(value) + ' ' + this.name + ' (' + Math.round(this.min) + '-' + Math.round(this.max) + ')', TEXT_X, TEXT_Y);

                this.context.drawImage(this.canvas, GRAPH_X + PR, GRAPH_Y, GRAPH_WIDTH - PR, GRAPH_HEIGHT, GRAPH_X, GRAPH_Y, GRAPH_WIDTH - PR, GRAPH_HEIGHT);

                this.context.fillRect(GRAPH_X + GRAPH_WIDTH - PR, GRAPH_Y, PR, GRAPH_HEIGHT);

                this.context.fillStyle = this.bg;
                this.context.globalAlpha = 0.9;
                this.context.fillRect(GRAPH_X + GRAPH_WIDTH - PR, GRAPH_Y, PR, Math.round((1 - (value / maxValue)) * GRAPH_HEIGHT));
            }
        }
    }
}