<template>
  <div class="modal-backdrop" role="presentation" @click.self="$emit('close')">
    <section class="settings-modal" role="dialog" aria-modal="true">
      <header>
        <h2>{{ labels.settings }}</h2>
        <button class="icon-button icon-button--close" type="button" :aria-label="labels.close" @click="$emit('close')">
          <span aria-hidden="true"></span>
        </button>
      </header>

      <label class="setting-row">
        <span>{{ labels.launchOnStartup }}</span>
        <input v-model="draft.launchOnStartup" type="checkbox" />
      </label>

      <label class="setting-row">
        <span>{{ labels.minimizeToTray }}</span>
        <input v-model="draft.minimizeToTray" type="checkbox" />
      </label>

      <label class="setting-row">
        <span>{{ labels.autoConnect }}</span>
        <input v-model="draft.autoConnect" type="checkbox" />
      </label>

      <label class="setting-row setting-row--select">
        <span>{{ labels.language }}</span>
        <select v-model="draft.language">
          <option value="ru">Русский</option>
          <option value="en">English</option>
        </select>
      </label>

      <label class="setting-row">
        <span>{{ labels.telemetry }}</span>
        <input v-model="telemetryChecked" type="checkbox" />
      </label>

      <footer>
        <button class="secondary-button" type="button" @click="$emit('close')">{{ labels.close }}</button>
        <button class="primary-button" type="button" @click="save">OK</button>
      </footer>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, reactive, watch } from 'vue';
import type { AppSettings } from '../types/vpn';
import { t } from '../services/i18n';

const props = defineProps<{
  settings: AppSettings;
}>();

const emit = defineEmits<{
  close: [];
  save: [settings: AppSettings];
}>();

const draft = reactive<AppSettings>({ ...props.settings });
const labels = computed(() => t(draft.language));

// null отображаем как false в чекбоксе
const telemetryChecked = computed({
  get: () => draft.telemetryConsent === true,
  set: (v: boolean) => { draft.telemetryConsent = v; },
});

watch(
  () => props.settings,
  (settings) => {
    Object.assign(draft, settings);
  },
  { deep: true },
);

function save() {
  emit('save', { ...draft });
  emit('close');
}
</script>
