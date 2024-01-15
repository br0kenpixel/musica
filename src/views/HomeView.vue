<script setup lang="ts">
import { get_settings } from '../backend/backend';
</script>

<template>
    <div></div>
</template>

<script lang="ts">
export default {
    created() {
        get_settings().then((settings) => {
            this.$router.push(settings.home);
        })
    }
}
</script>