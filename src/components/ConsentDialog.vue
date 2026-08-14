<template>
  <div class="modal-backdrop" role="presentation">
    <section class="settings-modal" role="dialog" aria-modal="true" aria-labelledby="consent-title">
      <header>
        <h2 id="consent-title">{{ labels.telemetryConsentTitle }}</h2>
      </header>

      <p class="consent-body">{{ labels.telemetryConsentBody }}</p>

      <footer>
        <button class="secondary-button" type="button" @click="$emit('respond', false)">
          {{ labels.telemetryConsentDecline }}
        </button>
        <button class="primary-button" type="button" @click="$emit('respond', true)">
          {{ labels.telemetryConsentAccept }}
        </button>
      </footer>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Language } from '../types/vpn';
import { t } from '../services/i18n';

const props = defineProps<{ language: Language }>();
const emit = defineEmits<{ respond: [consent: boolean] }>();

const labels = computed(() => t(props.language));
</script>

<style scoped>
.consent-body {
  padding: 0 0 1rem;
  line-height: 1.5;
  font-size: 0.875rem;
  opacity: 0.85;
}
</style>
