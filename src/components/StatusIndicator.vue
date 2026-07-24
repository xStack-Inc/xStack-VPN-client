<template>
  <div class="status-indicator" :class="`status-indicator--${status}`" aria-hidden="true">
    <span class="status-indicator__mark"></span>
    <span class="status-indicator__line"></span>
  </div>
</template>

<script setup lang="ts">
import type { VpnStatus } from '../types/vpn';

defineProps<{
  status: VpnStatus;
}>();
</script>
