<script setup lang="ts">
import { get_library, play_track } from '../backend/backend';
import { Song } from '../backend/types';
import SongList from '../components/SongList.vue';
</script>

<template>
    <p class="h3">Library</p>

    <ul v-show="loading" class="list-inline">
        <li class="list-inline-item">
            <v-progress-circular indeterminate></v-progress-circular>
        </li>
        <li class="list-inline-item">
            <p>Loading</p>
        </li>
    </ul>

    <SongList ref="songlist" v-show="!loading" @selected="selected" />
</template>

<script lang="ts">
export default {
    data() {
        return {
            loading: true
        };
    },
    mounted() {
        get_library().then((tracks) => {
            (this.$refs.songlist as any).songs = tracks;
            this.loading = false;
        });
    },
    methods: {
        async selected(track: Song) {
            await play_track(track.id);
        }
    }
}
</script>