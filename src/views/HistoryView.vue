<script setup lang="ts">
import { get_history, play_track } from '../backend/backend';
import { Song } from '../backend/types';
import SongList from '../components/SongList.vue';
</script>

<template>
    <p class="h3">Recently Played</p>

    <SongList ref="songlist" @selected="selected" />
</template>

<script lang="ts">
export default {
    mounted() {
        this.load_history();
    },
    methods: {
        async load_history() {
            const tracks = await get_history();

            (this.$refs.songlist as any).songs = tracks;
        },
        async selected(track: Song) {
            await play_track(track.id);
        }
    }
}
</script>