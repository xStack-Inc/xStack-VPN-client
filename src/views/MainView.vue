<template>
  <main class="vpn-shell">
    <header class="app-header">
      <div>
        <p class="app-kicker">Secure Link Console</p>
        <h1>{{ labels.appName }}</h1>
      </div>
      <button class="settings-button" type="button" :aria-label="labels.settings" @click="settingsOpen = true">
        <span aria-hidden="true"></span>
      </button>
    </header>

    <section class="connection-console" :class="`connection-console--${store.state.status}`">
      <div class="status-strip">
        <StatusIndicator :status="store.state.status" />
        <div>
          <span>{{ labels.stateCode }}</span>
          <strong>{{ shortStatus }}</strong>
        </div>
      </div>

      <div class="switch-panel">
        <PowerButton
          :status="store.state.status"
          :language="store.state.settings.language"
          @toggle="store.toggleVpn"
        />
        <div class="status-copy">
          <p>{{ statusText(store.state.status, store.state.settings.language) }}</p>
          <span>{{ store.state.status === 'connected' ? labels.server : labels.noConnection }}</span>
        </div>
      </div>
    </section>

    <StatsPanel
      :stats="store.state.stats"
      :language="store.state.settings.language"
      :status="store.state.status"
    />

    <section v-if="store.state.status === 'error'" class="error-line" role="alert">
      <span>{{ store.state.errorMessage ?? statusText('error', store.state.settings.language) }}</span>
      <button type="button" @click="store.toggleVpn">{{ labels.retry }}</button>
    </section>

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
const shortStatus = computed(() => store.state.status.toUpperCase());

onMounted(() => {
  void store.initialize();
});

onBeforeUnmount(() => {
  store.dispose();
});
</script>
