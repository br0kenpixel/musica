<script setup lang="ts">
import { get_history } from '../backend/backend';
import SongList from '../components/SongList.vue';
</script>

<template>
    <p class="h3">Recently Played</p>

    <SongList ref="songlist" />
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
        }
    }
}
</script>