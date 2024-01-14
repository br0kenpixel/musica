<script setup lang="ts">
import { getVersion } from '@tauri-apps/api/app';
import { open } from '@tauri-apps/api/shell';
</script>

<template>
    <h1>About</h1>

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
            appVersion: ''
        }
    },
    mounted() {
        this.displayAppVersion();
    },
    methods: {
        displayAppVersion() {
            getVersion().then((version) => {
                this.appVersion = version;
            });
        }
    }
}
</script>