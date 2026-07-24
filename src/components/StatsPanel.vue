<template>
  <section class="stats-panel">
    <div>
      <span>{{ labels.ip }}</span>
      <strong>{{ stats.ipAddress }}</strong>
    </div>
    <div>
      <span>{{ labels.duration }}</span>
      <strong>{{ formatDuration(stats.durationSeconds) }}</strong>
    </div>
    <div>
      <span>{{ labels.traffic }}</span>
      <strong>{{ formatBytes(stats.transferredBytes) }}</strong>
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Language, VpnStats } from '../types/vpn';
import { formatBytes, formatDuration } from '../services/format';
import { t } from '../services/i18n';

const props = defineProps<{
  stats: VpnStats;
  language: Language;
}>();

const labels = computed(() => t(props.language));
</script>

