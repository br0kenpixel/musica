import { emit } from "@tauri-apps/api/event";

export class Counter {
    end: number;
    paused: boolean;
    private current: number;
    private callback: (curent: number) => void;
    private tick_triggered: boolean;
    private stop: boolean;

    constructor(length: number, callback?: (current: number) => void) {
        this.end = length;
        this.current = 0;
        this.paused = true;
        this.tick_triggered = false;
        this.stop = false;

        if (callback) {
            this.callback = callback;
            this.callback(0);
        } else {
            this.callback = (_) => { };
        }
    }

    cancel() {
        this.stop = true;
    }

    resume() {
        if (!this.paused) {
            console.warn("Counter.resume() called on non-paused instance");
            return;
        }

        if (!this.tick_triggered) {
            this._tick();
            this.tick_triggered = true;
        }
        this.paused = false;
    }

    pause() {
        this.paused = true;
    }

    private _tick() {
        if (this.stop) {
            return;
        }

        if (!this.paused && this.current < this.end) {
            this.current++;
            this.callback(this.current);
        } else if (this.current >= this.end) {
            emit("track_ended");
        }

        setTimeout(() => {
            this._tick()
        }, 1000);
    }
}