<template>
  <button
    class="power-button"
    :class="[
      `power-button--${status}`,
      { 'power-button--busy': busy, 'power-button--locked': locked },
    ]"
    :disabled="busy || pendingAccess"
    type="button"
    :aria-label="label"
    :aria-pressed="status === 'connected'"
    @click="$emit('toggle')"
  >
    <span class="power-button__press" aria-hidden="true">
      <span class="power-button__press-core">
        <span class="power-button__press-mark"></span>
      </span>
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
  locked?: boolean;
  pendingAccess?: boolean;
}>();

defineEmits<{
  toggle: [];
}>();

const busy = computed(() => isTransitioning(props.status));
const label = computed(() => {
  if (props.pendingAccess) {
    return t(props.language).accountRequesting;
  }

  if (props.locked) {
    return t(props.language).accountRequired;
  }

  return props.status === 'connected' ? t(props.language).disconnect : t(props.language).connect;
});

const stateLabel = computed(() => {
  if (props.pendingAccess) {
    return t(props.language).preparing;
  }

  if (props.locked) {
    return t(props.language).accountRequiredState;
  }

  return busy.value
    ? t(props.language).preparing
    : props.status === 'connected'
      ? t(props.language).online
      : t(props.language).offline;
});
</script>
