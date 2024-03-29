<script setup lang="ts">
import { useTheme } from 'vuetify';
import { clear_history, get_config_path, get_data_dir, get_library_path, get_settings, reload_library, save_settings, update_settings } from '../backend/backend';
import { Settings } from '../backend/types';
import { setTheme } from '../theme';

const vuetifyTheme = useTheme();

function updateTheme(theme: string) {
    setTheme(vuetifyTheme, theme);
}
</script>

<template>
    <h1>Settings</h1>
    <div style="height: 30px;"></div>

    <div v-if="locked">
        <v-progress-circular indeterminate size="24" class="ms-2"></v-progress-circular>
        {{ wait_text }}
    </div>
    <div v-if="save_error">
        <v-alert type="error" title="Error" text="Settings could not be saved."></v-alert>
    </div>

    <div style="height: 10px;"></div>

    <div>
        <div class="row mb-3">
            <label for="theme" class="col-sm-2 col-form-label">Data directory</label>
            <div class="col">
                <input type="text" class="form-control" disabled v-model="datadir">
            </div>
        </div>

        <div class="row mb-3">
            <label for="theme" class="col-sm-2 col-form-label">Library</label>
            <div class="col">
                <input type="text" class="form-control" disabled v-model="library_path">
            </div>
        </div>

        <div class="row mb-3">
            <label for="theme" class="col-sm-2 col-form-label">Settings file</label>
            <div class="col">
                <input type="text" class="form-control" disabled v-model="settings_path">
            </div>
        </div>

        <div class="row mb-3">
            <label for="theme" class="col-sm-2 col-form-label">Theme</label>
            <div class="col-sm-2">
                <select class="form-select form-control" :disabled="locked" v-model="theme"
                    @change="() => updateTheme(theme)">
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                </select>
            </div>
        </div>

        <div class="row mb-3">
            <label for="theme" class="col-sm-2 col-form-label">Home Page</label>
            <div class="col-sm-3">
                <select class="form-select form-control" :disabled="locked" v-model="home_page">
                    <option value="history" selected>Recently played</option>
                    <option value="library">Library</option>
                </select>
            </div>
        </div>

        <div style="height: 30px;"></div>

        <form class="row row-cols-lg-auto g-3 align-items-center">
            <div class="col-12">
                <v-btn prepend-icon="mdi-reload" color="blue" :disabled="locked" @click="reload_lib">
                    Rescan library
                </v-btn>
            </div>
            <div class="col-12">
                <v-btn prepend-icon="mdi-trash-can" color="red" :disabled="locked" @click="erase_history">
                    Clear history
                </v-btn>
            </div>
        </form>
    </div>
</template>

<script lang="ts">
export default {
    data() {
        return {
            locked: false,
            wait_text: "Loading...",
            save_error: false,
            vuetifyTheme: null,
            datadir: "Loading...",
            library_path: "Loading...",
            settings_path: "Loading...",
            theme: "light",
            home_page: "history",
        }
    },
    mounted() {
        get_settings().then((settings) => {
            this.theme = settings.theme;
            this.home_page = settings.home;
            this.locked = false;
        });
        get_data_dir().then((path) => this.datadir = path);
        get_library_path().then((path) => this.library_path = path);
        get_config_path().then((path) => this.settings_path = path);
    },
    methods: {
        into_settings(): Settings {
            return {
                theme: this.theme,
                home: this.home_page
            };
        },
        lock(message: string) {
            this.locked = true;
            this.wait_text = message;
        },
        release() {
            this.locked = false;
        },
        async save() {
            this.lock("Loading...");

            const settings = this.into_settings();
            await update_settings(settings);

            if (!await save_settings()) {
                this.save_error = true;
            }

            this.release();
        },
        async reload_lib() {
            this.lock("Reloading library...");

            await reload_library();

            this.release();
        },
        async erase_history() {
            this.lock("Clearing history...");

            await clear_history();

            this.release();
        }
    },
    watch: {
        theme(_old, _new) {
            this.save();
        },
        home_page(_old, _new) {
            this.save();
        },
        library_scan(_old, _new) {
            this.save();
        }
    }
}
</script>