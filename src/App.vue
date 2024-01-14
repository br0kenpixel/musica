<script setup lang="ts">
import { RouterView } from 'vue-router';
import NavBar from './components/NavBar.vue';
import Player from './components/Player.vue';
import { setTheme } from './theme';
import { useTheme } from 'vuetify';
import { get_settings } from './backend/backend';
import { Settings } from './backend/types';

const vuetifyTheme = useTheme();

get_settings().then((settings: Settings) => {
    setTheme(vuetifyTheme, settings.theme);
});
</script>

<template>
    <v-app>
        <v-layout>
            <NavBar />

            <Player ref="player" />

            <v-main style="min-height: 300px">
                <v-container>
                    <RouterView />
                </v-container>
            </v-main>
        </v-layout>
    </v-app>
</template>

<script lang="ts">
export default {}
</script>