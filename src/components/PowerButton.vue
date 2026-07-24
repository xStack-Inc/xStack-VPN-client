<template>
  <button
    class="power-button"
    :class="[`power-button--${status}`, { 'power-button--busy': busy }]"
    :disabled="busy"
    type="button"
    :aria-label="label"
    :aria-pressed="status === 'connected'"
    @click="$emit('toggle')"
  >
    <span class="power-button__rail" aria-hidden="true">
      <span class="power-button__tick power-button__tick--off">0</span>
      <span class="power-button__track">
        <span class="power-button__lever"></span>
      </span>
      <span class="power-button__tick power-button__tick--on">1</span>
    </span>
    <span class="power-button__body">
      <span class="power-button__label">{{ label }}</span>
      <span class="power-button__state">{{ stateLabel }}</span>
    </span>
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
const stateLabel = computed(() =>
  busy.value ? t(props.language).preparing : props.status === 'connected' ? t(props.language).online : t(props.language).offline,
);
</script>
