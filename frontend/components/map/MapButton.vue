<template>
  <div class="map-button" @click="$emit('click')">
    <div class="map-icon">
      <i>🗺️</i>
    </div>
    <div class="button-pulse"></div>
  </div>
</template>

<script setup lang="ts">
defineEmits(['click'])
</script>

<style lang="scss" scoped>
.map-button {
  position: fixed;
  right: 20px;
  bottom: 20px;
  width: 60px;
  height: 60px;
  border-radius: 50%;
  background-color: var(--zelda-primary);
  border: 3px solid var(--zelda-border-light);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 10px rgba(47, 79, 45, 0.3);
  cursor: pointer;
  transition: all 0.3s ease;
  z-index: 100;
  
  &:hover {
    transform: translateY(-5px);
    box-shadow: 0 8px 15px rgba(47, 79, 45, 0.4);
    background-color: var(--zelda-primary-dark);
  }
  
  &:active {
    transform: translateY(0);
  }
}

.map-icon {
  font-size: 28px;
  color: white;
  position: relative;
  z-index: 2;
}

.button-pulse {
  position: absolute;
  inset: -3px;
  border-radius: 50%;
  border: 3px solid var(--zelda-primary-light);
  animation: pulse-anim 2s infinite;
  opacity: 0;
}

@keyframes pulse-anim {
  0% {
    transform: scale(1);
    opacity: 0.7;
  }
  70% {
    transform: scale(1.2);
    opacity: 0;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

@media (max-width: 768px) {
  .map-button {
    right: 15px;
    bottom: 15px;
    width: 50px;
    height: 50px;
  }
  
  .map-icon {
    font-size: 24px;
  }
}
</style> 