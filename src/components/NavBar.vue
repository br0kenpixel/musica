<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
import '../assets/navbar/styles.css';
import { RouterLink } from 'vue-router';
import { build_type } from '../backend/backend';
</script>

<template>
    <v-navigation-drawer width="200" permanent>
        <v-list-item title="Musica" :subtitle="appVersion"></v-list-item>
        <v-divider></v-divider>

        <div>
            <RouterLink :to="item.link" class="router-link" v-for="item in UPPER_LINKS">
                <v-list-item link :title="item.display"></v-list-item>
            </RouterLink>
        </div>

        <div id="bottomSidebar">
            <RouterLink :to="item.link" class="router-link" v-for="item in LOWER_LINKS">
                <v-list-item link :title="item.display"></v-list-item>
            </RouterLink>
        </div>
    </v-navigation-drawer>
</template>

<script lang="ts">
const UPPER_LINKS = [
    { display: "📘 Recently played", link: "history" },
    { display: "💽 Library", link: "library" },
];
const LOWER_LINKS = [
    { display: "⚙️ Settings", link: "settings" },
    { display: "ℹ️ About", link: "about" },
];

export default {
    data() {
        return {
            appVersion: ''
        }
    },
    mounted() {
        this.displayAppVersion();
    },
    methods: {
        async displayAppVersion() {
            this.appVersion = await getVersion();

            if (await build_type() === "debug") {
                this.appVersion += "-debug";
            }
        }
    }
}
</script>