import { defineStore } from 'pinia';
import { Song } from '../backend/types';

export const useShortHistoryStore = defineStore("short-history", {
    state: () => ({
        songs: [] as Array<Song>,
        current: -1,
    }),

    getters: {
        empty: (state) => state.songs.length == 0,
        canGoBack: (state) => state.current - 1 < state.songs.length && state.current - 1 >= 0,
        canGoNext: (state) => state.current + 1 < state.songs.length && state.current + 1 >= 0,
        currentTrack: (state) => state.songs[state.current]
    },

    actions: {
        push(track: Song) {
            this.songs.push(track);
            this.current = this.songs.length - 1;
        },
        previous() {
            this.current--;
        },
        next() {
            this.current++;
        }
    }
});