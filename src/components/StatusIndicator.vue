<template>
  <div class="status-indicator" :class="`status-indicator--${status}`" aria-hidden="true">
    <span></span>
  </div>
</template>

<script setup lang="ts">
import type { VpnStatus } from '../types/vpn';

defineProps<{
  status: VpnStatus;
}>();
</script>

