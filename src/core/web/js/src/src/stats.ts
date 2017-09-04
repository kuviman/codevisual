// Original at https://github.com/mrdoob/stats.js/blob/master/src/Stats.js

namespace CodeVisual {
    class Stats {
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
            container.className = "stats";
            this.container = container;
            container.style.cssText = 'position:absolute;top:0;left:0;cursor:pointer;opacity:0.9;z-index:10000;display:none;';
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

    namespace Stats {
        let PR = Math.round(window.devicePixelRatio || 1);
        let HISTORY_LENGTH = 74;
        let WIDTH = 80 * PR, HEIGHT = 48 * PR,
            TEXT_X = 3 * PR, TEXT_Y = 2 * PR,
            GRAPH_X = 3 * PR, GRAPH_Y = 15 * PR,
            GRAPH_WIDTH = HISTORY_LENGTH * PR, GRAPH_HEIGHT = 30 * PR;

        type ValueAndMax = { value: number, max: number };

        class MaxStack {
            q: ValueAndMax[] = [];

            max(): number {
                return this.q.length == 0 ? -Infinity : this.q[this.q.length - 1].max;
            }

            push(value: number) {
                this.q.push({value: value, max: Math.max(value, this.max())});
            }

            pop(): number {
                return this.q.pop().value;
            }

            get length(): number {
                return this.q.length;
            }
        }

        class MaxQueue {
            q1: MaxStack = new MaxStack();
            q2: MaxStack = new MaxStack();

            max(): number {
                return Math.max(this.q1.max(), this.q2.max());
            }

            push(value: number) {
                this.q2.push(value);
            }

            pop(): number {
                if (this.q1.length == 0) {
                    while (this.q2.length != 0) {
                        this.q1.push(this.q2.pop());
                    }
                }
                return this.q1.pop();
            }

            get length(): number {
                return this.q1.length + this.q2.length;
            }
        }

        class MinMaxQueue {
            minQueue: MaxQueue = new MaxQueue();
            maxQueue: MaxQueue = new MaxQueue();

            push(value: number) {
                this.maxQueue.push(value);
                this.minQueue.push(-value);
            }

            pop(): number {
                this.minQueue.pop();
                return this.maxQueue.pop();
            }

            max(): number {
                return this.maxQueue.max();
            }

            min(): number {
                return -this.minQueue.max();
            }

            get length(): number {
                return this.maxQueue.length;
            }
        }

        export class Panel {
            private min = Infinity;
            private max = 0;
            private canvas: HTMLCanvasElement;
            private context: CanvasRenderingContext2D;
            private minMaxQueue: MinMaxQueue = new MinMaxQueue();
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
                this.minMaxQueue.push(value);
                this.min = this.minMaxQueue.min();
                this.max = this.minMaxQueue.max();
                if (this.minMaxQueue.length > HISTORY_LENGTH) {
                    this.minMaxQueue.pop();
                }

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

    export namespace internal {
        const stats = new Stats();
        let $stats: JQuery;

        export function update_stats() {
            stats.update();
        }

        on_init.push(() => {
            $player.find(".game-screen").append(stats.dom);
            $stats = $player.find(".stats");
            settings.add(new BooleanSetting("Show stats", true, (show) => {
                if (show)
                    $stats.fadeIn();
                else
                    $stats.fadeOut();
            }));
        });
    }
}