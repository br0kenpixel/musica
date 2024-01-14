<script setup lang="ts">
import '../assets/player/slider.css';
</script>

<template>
    <div class="container text-center">
        <div class="row">
            <div class="col-sm-1">
                <v-icon :icon="icon"></v-icon>
            </div>
            <div class="col">
                <input type="range" class="form-range" v-model.number="volume" min="0" max="100" step="1">
            </div>
        </div>
    </div>
</template>

<script lang="ts">
export default {
    emits: ['volumeChange'],

    data() {
        return { volume: 100, icon: "mdi-play" }
    },
    mounted() {
        this.updateIcon();
    },
    watch: {
        volume(_old, volume) {
            this.updateIcon();
            this.$emit("volumeChange", volume);
        }
    },
    methods: {
        updateIcon() {
            if (this.volume === 0) {
                this.icon = "mdi-volume-off";
            } else if (this.volume <= 50) {
                this.icon = "mdi-volume-medium";
            } else {
                this.icon = "mdi-volume-high";
            }
        }
    }
}
</script>