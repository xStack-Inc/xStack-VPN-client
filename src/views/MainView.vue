<template>
  <main class="vpn-shell">
    <header class="app-header">
      <div>
        <p class="app-kicker">Desktop VPN Client</p>
        <h1>{{ labels.appName }}</h1>
      </div>
      <button class="settings-button" type="button" :aria-label="labels.settings" @click="settingsOpen = true">
        ⚙
      </button>
    </header>

    <section class="connection-card">
      <StatusIndicator :status="store.state.status" />
      <p class="status-text">{{ statusText(store.state.status, store.state.settings.language) }}</p>
      <p class="server-text">{{ labels.server }} · {{ labels.location }}</p>

      <PowerButton
        :status="store.state.status"
        :language="store.state.settings.language"
        @toggle="store.toggleVpn"
      />
    </section>

    <StatsPanel :stats="store.state.stats" :language="store.state.settings.language" />

    <p v-if="store.state.errorMessage" class="error-line">{{ store.state.errorMessage }}</p>

    <SettingsModal
      v-if="settingsOpen"
      :settings="store.state.settings"
      @save="store.updateSettings"
      @close="settingsOpen = false"
    />
  </main>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import PowerButton from '../components/PowerButton.vue';
import SettingsModal from '../components/SettingsModal.vue';
import StatsPanel from '../components/StatsPanel.vue';
import StatusIndicator from '../components/StatusIndicator.vue';
import { statusText, t } from '../services/i18n';
import { useVpnStore } from '../stores/vpnStore';

const store = useVpnStore();
const settingsOpen = ref(false);
const labels = computed(() => t(store.state.settings.language));

onMounted(() => {
  void store.initialize();
});

onBeforeUnmount(() => {
  store.dispose();
});
</script>

