<template>
  <div class="emulator-card">
    <div class="emulator-header">
      <h3>{{ emulator.name }}</h3>
      <div v-if="emulator.version" class="header-info">
        <span class="version">v{{ emulator.version }}</span>
      </div>
    </div>
    <div class="emulator-info">
      <p v-if="emulator.lastUsed">Last used: {{ formatDate(emulator.lastUsed) }}</p>
      <p v-if="emulator.description">{{ emulator.description }}</p>
      <p v-if="emulator.size">Size: {{ emulator.size }}</p>
    </div>
    <div class="button-group">
      <slot name="actions"></slot>
    </div>
    <div v-if="errors[emulator.name]" class="error-message">
      {{ errors[emulator.name] }}
    </div>
  </div>
</template>

<script setup lang="ts">
interface Emulator {
  name: string;
  installed: boolean;
  version?: string;
  lastUsed?: string;
  size?: string;
  description?: string;
  platforms: string[];
}

const props = defineProps<{
  emulator: Emulator;
  errors: { [key: string]: string };
}>();

function formatDate(dateString?: string) {
  if (!dateString) return '';
  return new Date(dateString).toLocaleDateString();
}
</script> 