<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/api/shell';
import { build_type } from '../backend/backend';
</script>

<template>
    <h1>About</h1>

    <v-alert v-if="debug_warn" density="compact" type="warning" title="Debug build"
        text="You are using a debug build. Such builds are only for testing and have significantly worse performance."></v-alert>

    <p>Musica version {{ appVersion }}</p>

    <p>Used technologies</p>
    <ul>
        <li>Rust</li>
        <li>VueJS</li>
        <li>Vuetify</li>
        <li>Tauri</li>
        <li>Vite</li>
        <li>Bun</li>
    </ul>

    <p>Made by <a href="#" @click="() => open('https://github.com/br0kenpixel')">br0kenpixel</a>.</p>
</template>

<script lang="ts">
export default {
    data() {
        return {
            appVersion: '',
            debug_warn: false
        }
    },
    mounted() {
        this.displayAppVersion();
    },
    methods: {
        async displayAppVersion() {
            this.appVersion = await getVersion();

            if (await build_type() === "debug") {
                this.debug_warn = true;
            }
        }
    }
}
</script>