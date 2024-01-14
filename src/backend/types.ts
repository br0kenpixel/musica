export type Song = {
    readonly id: number,
    readonly title: string,
    readonly album: string,
    readonly artist: string,
    readonly format: string,
    readonly length: number,
};

export type Settings = {
    readonly theme: string,
    readonly library_scan: string,
    readonly home: string
};

export type PlaybackStarted = { Started: Song };

export type PlayerEvent = string | PlaybackStarted;