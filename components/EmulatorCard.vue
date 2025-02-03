<template>
  <div class="card">
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
import type { Emulator } from '~/composables/useEmulators';

defineProps<{
  emulator: Emulator;
  errors: { [key: string]: string };
}>();

function formatDate(dateString?: string) {
  if (!dateString) return '';
  return new Date(dateString).toLocaleDateString();
}
</script>

<style scoped>
.card {
  background: white;
  border-radius: 16px;
  padding: 1.5rem;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  transition: all 0.2s;
  border: 1px solid rgba(52, 152, 219, 0.1);
  position: relative;
  overflow: hidden;
}

.card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, #3498db, #2563eb, #3498db);
  opacity: 0;
  transition: opacity 0.2s;
  background-size: 200% 100%;
  animation: moveGradient 2s linear infinite;
}

.card:hover::before {
  opacity: 1;
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
}

.emulator-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.emulator-header h3 {
  font-size: 1.25rem;
  font-weight: 600;
  color: #1e293b;
  margin: 0;
}

.header-info {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.version {
  background: linear-gradient(90deg, #3498db, #2563eb, #3498db);
  background-size: 200% 100%;
  animation: moveGradient 2s linear infinite;
  color: white;
  padding: 0.3rem 0.8rem;
  border-radius: 6px;
  font-size: 0.9rem;
  font-weight: 500;
}

.emulator-info {
  margin-bottom: 1.5rem;
}

.emulator-info p {
  margin: 0.5rem 0;
  color: #64748b;
  line-height: 1.6;
}

.button-group {
  display: flex;
  gap: 0.5rem;
}

.error-message {
  margin-top: 1rem;
  color: #dc2626;
  font-size: 0.9rem;
}

.loader {
  display: inline-block;
  width: 1rem;
  height: 1rem;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  border-top-color: white;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes moveGradient {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 200% 50%;
  }
}
</style> 