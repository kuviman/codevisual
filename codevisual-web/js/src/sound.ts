namespace CodeVisual.internal {
    let howls: Howl[] = [];

    export function load_sound(path: string, on_load: (id: number) => void) {
        let id = howls.length;
        // TODO: onerror
        let howl = new Howl({
            src: path,
            onload: () => on_load(id)
        });
        howls.push(howl);
    }

    export function play_sound(id: number, volume: number) {
        let howl = howls[id];
        howl.volume(volume, howl.play());
    }
}