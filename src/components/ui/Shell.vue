<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { Moon, Rocket, Sun } from '@lucide/vue'

const theme = ref<'light' | 'dark'>('light')

onMounted(() => {
  theme.value = document.documentElement.classList.contains('dark') ? 'dark' : 'light'
})

watch(theme, (mode) => {
  document.documentElement.classList.toggle('dark', mode === 'dark')
  localStorage.setItem('theme', mode)
})

function toggleTheme() {
  theme.value = theme.value === 'dark' ? 'light' : 'dark'
}
</script>

<template>
  <div
    class="min-h-screen bg-gradient-to-b from-slate-50 via-white to-slate-100 text-slate-900 dark:from-slate-950 dark:via-slate-950 dark:to-slate-900 dark:text-slate-100"
  >
    <header
      class="sticky top-0 z-30 border-b border-slate-200/80 bg-white/80 shadow-sm backdrop-blur-xl dark:border-slate-800/80 dark:bg-slate-900/70"
    >
      <div class="mx-auto flex w-full max-w-6xl items-center justify-between gap-4 px-4 py-4 sm:px-6 lg:px-8">
        <div class="flex items-center gap-2">
          <span
            class="grid h-8 w-8 shrink-0 place-items-center rounded-xl bg-gradient-to-br from-cyan-500 to-blue-600 text-white shadow-sm"
          >
            <Rocket :size="16" />
          </span>
          <div>
            <p class="text-[10px] font-semibold uppercase tracking-[0.24em] text-slate-500 dark:text-slate-400">
              INTENIQUETIC
            </p>
            <h1 class="text-sm font-bold text-slate-900 dark:text-slate-100">Launchpad</h1>
          </div>
        </div>

        <button
          type="button"
          class="inline-flex h-9 w-9 items-center justify-center rounded-xl border border-slate-300/80 bg-white text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-500/60 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
          :aria-label="theme === 'dark' ? 'Light mode' : 'Dark mode'"
          :title="theme === 'dark' ? 'Light mode' : 'Dark mode'"
          @click="toggleTheme"
        >
          <Sun v-if="theme === 'dark'" :size="18" />
          <Moon v-else :size="18" />
        </button>
      </div>
    </header>

    <slot />

    <footer
      class="border-t border-slate-200/80 bg-white/70 backdrop-blur dark:border-slate-800/80 dark:bg-slate-900/70"
    >
      <div class="mx-auto w-full max-w-6xl px-4 py-4 text-xs text-slate-500 dark:text-slate-400 sm:px-6 lg:px-8">
        2026 inteniquetic.com
      </div>
    </footer>
  </div>
</template>
