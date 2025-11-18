<script setup>
import IconButton from "./IconButton.vue";
import { SunIcon, MoonIcon } from "@heroicons/vue/24/outline";
import { useDark, useToggle } from "@vueuse/core";

const { t } = useI18n();

const isDark = useDark();
const toggleDark = useToggle(isDark);
</script>

<template>
  <IconButton
    :icon="isDark ? SunIcon : MoonIcon"
    :text="isDark ? t('Switch to light mode') : t('Switch to dark mode')"
    size="size-8"
    @click="toggleDark()"
  />
</template>
