<script setup lang="ts">
import { prettifyTime } from '../backend/helpers';
import { type Song } from '../backend/types';
import '../assets/songlist/styles.css';
import { update_history } from '../backend/backend';
import { useShortHistoryStore } from '../stores/short_history';
</script>

<template>
    <v-data-table id="songlist" :items="songs" :headers="(headers as any)" @dblclick:row="trackSelected"></v-data-table>
</template>

<script lang="ts">
export default {
    expose: ['songs'],
    emits: ['selected'],

    data() {
        const history = useShortHistoryStore();

        return {
            short_history: history,
            songs: [] as Array<Song>,
            headers: [
                { title: "Title", key: "title" },
                { title: "Album", key: "album" },
                { title: "Artist", key: "artist" },
                { title: "Format", key: "format" },
                { title: "Length", key: "length", value: (item: Song) => prettifyTime(item.length) },
            ]
        }
    },
    methods: {
        resized() {
            const table = document.getElementById("songlist")!;
            const winheight = window.innerHeight;

            table.style.height = `${winheight - 138}px`;
        },
        async trackSelected(_event: any, { item }: any) {
            this.$emit("selected", item);
            this.short_history.push(item);
            await update_history(item.id);
        }
    },
    mounted() {
        this.resized();
    },
    created() {
        window.addEventListener("resize", this.resized);
    },
    destroyed() {
        window.removeEventListener("resize", this.resized);
    },
}
</script>