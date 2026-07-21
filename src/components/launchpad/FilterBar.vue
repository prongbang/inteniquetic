<script setup lang="ts">
import SearchInput from '@/components/ui/SearchInput.vue'

defineProps<{ filterNames: string[] }>()
const query = defineModel<string>({ required: true })
const activeFilter = defineModel<string>('activeFilter', { required: true })
</script>

<template>
  <div class="mt-6 grid gap-3 sm:grid-cols-3">
    <div class="sm:col-span-2">
      <SearchInput v-model="query" placeholder="Search projects or categories" />
    </div>
    <div class="flex flex-wrap items-center gap-2">
      <button
        v-for="name in filterNames"
        :key="name"
        type="button"
        :class="[
          'rounded-full border px-4 py-2 text-xs font-semibold uppercase tracking-wide backdrop-blur transition',
          activeFilter === name
            ? 'border-white/30 bg-white/20 text-white shadow-sm'
            : 'border-transparent bg-slate-100/10 text-white/90 hover:bg-slate-100/20',
        ]"
        @click="activeFilter = name"
      >
        {{ name === 'all' ? 'All' : name }}
      </button>
    </div>
  </div>
</template>
