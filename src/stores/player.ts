import { defineStore } from 'pinia';
import { Song } from '../backend/types';

export const usePlayerStore = defineStore('player', {
    state: () => ({
        track: null as (null | Song),
        playtime: 0.0
    }),

    getters: {
        title: (state) => state.track?.title,
        album: (state) => state.track?.album,
        artist: (state) => state.track?.artist,
        length: (state) => state.track?.length,
        trackSet: (state) => state.track !== null,
        playtime: (state) => state.playtime
    },

    actions: {
        setTrack(track: Song) {
            this.track = track;
            this.playtime = 0.0;
        },
        incrementPlaytime() {
            if (this.trackSet) {
                this.playtime += 1;
            }
        }
    }
});