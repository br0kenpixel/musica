export function prettifyTime(len: number): string {
    var mins = Math.floor(len / 60);
    var secs = len % 60;

    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
}