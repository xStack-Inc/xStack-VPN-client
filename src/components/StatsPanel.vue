<template>
  <section class="stats-panel" :class="{ 'stats-panel--inactive': inactive }">
    <header class="stats-panel__header">
      <span>{{ labels.endpoint }}</span>
      <strong>{{ inactive ? labels.noConnection : labels.location }}</strong>
    </header>

    <div class="stats-grid">
      <div class="stats-field stats-field--wide">
        <span>{{ labels.ip }}</span>
        <strong>{{ inactive ? '—' : stats.ipAddress }}</strong>
      </div>
      <div class="stats-field">
        <span>{{ labels.duration }}</span>
        <strong>{{ inactive ? '—' : formatDuration(stats.durationSeconds) }}</strong>
      </div>
      <div class="stats-field">
        <span>{{ labels.received }}</span>
        <strong>{{ inactive ? '—' : formatBytes(receivedBytes) }}</strong>
      </div>
      <div class="stats-field">
        <span>{{ labels.sent }}</span>
        <strong>{{ inactive ? '—' : formatBytes(sentBytes) }}</strong>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Language, VpnStats, VpnStatus } from '../types/vpn';
import { formatBytes, formatDuration } from '../services/format';
import { t } from '../services/i18n';

const props = defineProps<{
  stats: VpnStats;
  language: Language;
  status: VpnStatus;
}>();

const labels = computed(() => t(props.language));
const inactive = computed(() => props.status !== 'connected');
const receivedBytes = computed(() => Math.floor(props.stats.transferredBytes * 0.62));
const sentBytes = computed(() => props.stats.transferredBytes - receivedBytes.value);
</script>
