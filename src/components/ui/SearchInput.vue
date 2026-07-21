<script setup lang="ts">
import { Search } from '@lucide/vue'

defineProps<{ placeholder?: string }>()
const query = defineModel<string>({ required: true })
</script>

<template>
  <div class="relative">
    <Search
      :size="16"
      class="pointer-events-none absolute left-4 top-1/2 -translate-y-1/2 text-slate-400"
    />
    <input
      v-model="query"
      type="search"
      :placeholder="placeholder"
      class="w-full rounded-2xl border border-white/30 bg-white/85 py-3 pl-11 pr-4 text-sm text-slate-900 shadow-[inset_0_1px_0_rgba(255,255,255,0.6)] outline-none ring-0 transition placeholder:text-slate-500 focus:border-cyan-400 focus:ring-4 focus:ring-cyan-500/20 dark:border-slate-700 dark:bg-slate-900/80 dark:text-slate-100 dark:placeholder-slate-500 dark:focus:border-cyan-500 dark:focus:ring-cyan-500/20"
    />
  </div>
</template>
