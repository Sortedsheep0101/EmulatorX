<template>
  <div class="app-container">
    <header>
      <h1>EmulatorX</h1>
    </header>

    <nav>
      <div 
        class="nav-background" 
        :style="{ 
          transform: `translateX(${
            currentRoute === '/' ? '0' : 
            currentRoute === '/available' ? '100' : 
            currentRoute === '/settings' ? '200' : '0'}%)`
        }"
      ></div>
      <NuxtLink
        v-for="tab in tabs"
        :key="tab.id"
        :to="tab.path"
        :class="[
          'transition-all duration-200 hover:scale-110 active:scale-95',
          { active: (tab.id === 'installed' && currentRoute === '/') || 
                   currentRoute === tab.path }
        ]"
      >
        {{ tab.label }}
      </NuxtLink>
    </nav>

    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'nuxt/app';

const tabs = [
  { id: 'installed', label: 'Installed', path: '/' },
  { id: 'available', label: 'Available', path: '/available' },
  { id: 'settings', label: 'Settings', path: '/settings' }
];

const route = useRoute();
const currentRoute = computed(() => route.path);
</script>

<style scoped>
/* Move all the styles from App.vue except emulator-specific ones */
.app-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
  font-family: system-ui, -apple-system, sans-serif;
  min-height: 100vh;
}

header {
  text-align: center;
  margin-bottom: 3rem;
}

h1 {
  font-size: 2.8rem;
  background: linear-gradient(45deg, #3498db, #2563eb);
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  margin-bottom: 1rem;
}

nav {
  display: flex;
  justify-content: center;
  gap: 1rem;
  background: white;
  padding: 0.75rem;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  max-width: fit-content;
  border: 1px solid rgba(52, 152, 219, 0.1);
  position: relative;
  margin: 0 auto 2rem;
}

.nav-background {
  position: absolute;
  left: 0.75rem;
  top: 0.75rem;
  width: calc(33.333% - 0.667rem);
  height: calc(100% - 1.5rem);
  background: linear-gradient(45deg, #3498db, #2563eb);
  border-radius: 8px;
  transition: transform 0.3s ease;
  z-index: 0;
}

nav a {
  position: relative;
  padding: 0.75rem 2rem;
  border: none;
  border-radius: 8px;
  background: transparent;
  color: #64748b;
  cursor: pointer;
  transition: all 0.2s ease;
  font-weight: 500;
  min-width: 120px;
  font-size: 0.95rem;
  z-index: 1;
  transform-origin: center;
  text-decoration: none;
  text-align: center;
}

nav a:hover {
  transform: scale(1.1);
}

nav a:active {
  transform: scale(0.95);
}

nav a.active {
  color: white;
}

nav a:hover:not(.active) {
  color: #334155;
}
</style> 