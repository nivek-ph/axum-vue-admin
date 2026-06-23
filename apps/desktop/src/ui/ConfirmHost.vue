<template>
  <Teleport to="body">
    <Transition name="content-fade">
      <div v-if="confirmDialog" class="app-confirm-backdrop" @click="confirmDialogCancel">
        <section
          class="app-confirm"
          :class="`app-confirm--${confirmDialog.type}`"
          role="alertdialog"
          aria-modal="true"
          :aria-label="confirmDialog.title"
          @click.stop
        >
          <header class="app-confirm-header">
            <h2>{{ confirmDialog.title }}</h2>
          </header>
          <p class="app-confirm-message">{{ confirmDialog.message }}</p>
          <footer class="app-confirm-actions">
            <button class="app-confirm-button app-confirm-button--default" type="button" @click="confirmDialogCancel">
              {{ t('Cancel') }}
            </button>
            <button
              data-test="confirm-accept"
              class="app-confirm-button app-confirm-button--primary"
              type="button"
              @click="confirmDialogAccept"
            >
              {{ t('Confirm') }}
            </button>
          </footer>
        </section>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { t } from '@/i18n'
import { confirmDialog, confirmDialogAccept, confirmDialogCancel } from './feedback'
</script>
