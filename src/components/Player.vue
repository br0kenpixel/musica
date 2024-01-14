<script setup lang="ts">
import '../assets/player/styles.css';
import { prettifyTime } from '../backend/helpers';
import { pause_playback, resume_playback, set_volume, stop_playback } from '../backend/backend';
import VolumeSlider from './VolumeSlider.vue';
import { Event, listen } from '@tauri-apps/api/event';
import { PlayerEvent, PlaybackStarted } from '../backend/types';
import { Counter } from '../counter';
import { PlayerStatus } from '../player_state';
</script>

<template>
    <v-app-bar color="grey" height="65" flat>
        <v-container fill-height fluid>
            <v-row no-gutters>
                <v-col>
                    <v-sheet id="song-title" class="transparent-color" align="center">
                        {{ realTitle }}
                    </v-sheet>
                </v-col>
            </v-row>
            <v-row no-gutters>
                <v-col>
                    <v-sheet id="song-description" class="transparent-color" align="center">
                        {{ description }}
                    </v-sheet>
                </v-col>
            </v-row>
            <v-row no-gutters align="center" justify="center">
                <v-col align="center" sm="1">
                    <v-sheet class="transparent-color">
                        <div class="time-text">{{ runtime }}</div>
                    </v-sheet>
                </v-col>
                <v-col align="center" sm="9">
                    <v-sheet>
                        <v-progress-linear :indeterminate="loading" color="blue-lighten-3"
                            :model-value="progress"></v-progress-linear>
                    </v-sheet>
                </v-col>
                <v-col align="center" sm="1">
                    <v-sheet class="transparent-color">
                        <div class="time-text">{{ length }}</div>
                    </v-sheet>
                </v-col>
            </v-row>
        </v-container>

        <v-container style="width: 300px !important;">
            <v-row no-gutters>
                <v-col>
                    <v-btn icon="mdi-skip-previous" density="compact"></v-btn>
                </v-col>
                <v-col>
                    <v-btn :icon="PLAY_ICONS[Number(state)]" density="compact" :disabled="state === PlayerStatus.None"
                        @click="toggle_playback"></v-btn>
                </v-col>
                <v-col>
                    <v-btn icon="mdi-skip-next" density="compact"></v-btn>
                </v-col>
                <v-col>
                    <v-btn icon="mdi-stop" density="compact" :disabled="state === PlayerStatus.None" @click="stop"></v-btn>
                </v-col>
                <v-col>
                    <v-tooltip text="Lyrics" location="bottom">
                        <template v-slot:activator="{ props }">
                            <v-btn v-bind="props" icon="mdi-card-text-outline" density="compact" disabled></v-btn>
                        </template>
                    </v-tooltip>
                </v-col>
            </v-row>

            <v-row no-gutters>
                <v-col style="height: 15px;">
                    <VolumeSlider @volume-change="change_volume" />
                </v-col>
            </v-row>
        </v-container>
    </v-app-bar>
</template>

<script lang="ts">
const MAX_TITLE_LEN = 64;
const MAX_DESC_LEN = 84;
const PLAY_ICONS = ["mdi-pause", "mdi-play", "mdi-play"];

export default {
    expose: ['displaySongInfo', 'reset'],

    data() {
        const counter = new Counter(0, () => { });

        return {
            loading: false,
            counter: counter as Counter,
            realTitle: "-",
            description: "-",
            runtime: "--:--",
            length: "--:--",
            progress: 0,
            state: PlayerStatus.None,
        }
    },
    methods: {
        displaySongInfo(title: string, album: string, artist: string, length: number) {
            var realTitle = title.length > MAX_TITLE_LEN ? (title.substring(0, MAX_TITLE_LEN - 3) + "...") : (title);
            var realDescription = `${artist} - ${album}`;

            if (realDescription.length > MAX_DESC_LEN) {
                realDescription = realDescription.substring(0, MAX_DESC_LEN - 3) + "...";
            }

            this.realTitle = realTitle;
            this.description = realDescription;
            this.length = prettifyTime(length);

            this.counter.cancel();
            this.counter = new Counter(length, this.displayRuntime);
        },
        displayRuntime(time: number) {
            this.runtime = prettifyTime(time);
            this.progress = (time / this.counter.end) * 100;
        },
        reset() {
            this.counter.cancel();
            this.state = PlayerStatus.None;
            this.realTitle = "-";
            this.description = "-";
            this.runtime = "--:--";
            this.length = "--:--";
            this.progress = 0;
        },
        async setupEventListener() {
            await listen("player", (event: Event<PlayerEvent>) => {
                if (event.payload === "Loading") {
                    this.stop();
                    this.loading = true;
                    this.realTitle = "Loading...";
                } else if (event.payload === "Stopped") {
                    this.reset();
                } else if (event.payload === "Paused") {
                    this.counter.pause();
                    this.state = PlayerStatus.Paused;
                } else if (event.payload === "Resumed") {
                    this.counter.resume();
                    this.state = PlayerStatus.Playing;
                } else {
                    const track = (event.payload as PlaybackStarted).Started;

                    this.displaySongInfo(track.title, track.album, track.artist, track.length);
                    this.loading = false;
                    console.log("not loading anymore");

                    this.counter.resume();
                    this.state = PlayerStatus.Playing;
                }
            });
        },
        async toggle_playback() {
            switch (this.state) {
                case PlayerStatus.Playing: {
                    await pause_playback();
                    this.state = PlayerStatus.Paused;
                    break;
                }
                case PlayerStatus.Paused: {
                    await resume_playback();
                    this.state = PlayerStatus.Playing;
                    break;
                }
            }
        },
        async stop() {
            await stop_playback();
            this.reset();
        },
        async change_volume(value: number) {
            await set_volume(value);
        }
    },
    mounted() {
        this.setupEventListener();
        stop_playback();
    },
}
</script>