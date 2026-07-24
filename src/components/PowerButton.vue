<template>
  <button
    class="power-button"
    :class="[`power-button--${status}`, { 'power-button--busy': busy }]"
    :disabled="busy"
    type="button"
    :aria-label="label"
    @click="$emit('toggle')"
  >
    <span class="power-button__icon"></span>
    <span class="power-button__label">{{ label }}</span>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Language, VpnStatus } from '../types/vpn';
import { isTransitioning } from '../services/vpnStateMachine';
import { t } from '../services/i18n';

const props = defineProps<{
  status: VpnStatus;
  language: Language;
}>();

defineEmits<{
  toggle: [];
}>();

const busy = computed(() => isTransitioning(props.status));
const label = computed(() =>
  props.status === 'connected' ? t(props.language).disconnect : t(props.language).connect,
);
</script>

