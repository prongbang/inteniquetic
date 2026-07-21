<script setup lang="ts">
import { cva, type VariantProps } from 'class-variance-authority'
import type { Component } from 'vue'
import { cn } from '@/lib/utils'

const buttonVariants = cva(
  'inline-flex items-center gap-2 rounded-xl px-4 py-2 text-sm font-semibold shadow-sm transition hover:-translate-y-0.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-500/60',
  {
    variants: {
      variant: {
        primary:
          'bg-slate-900 text-white hover:bg-slate-700 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200',
        secondary:
          'border border-slate-300/80 bg-white text-slate-700 hover:bg-slate-50 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800',
      },
    },
    defaultVariants: {
      variant: 'secondary',
    },
  },
)

withDefaults(
  defineProps<{
    label: string
    variant?: VariantProps<typeof buttonVariants>['variant']
    icon?: Component
  }>(),
  { variant: 'secondary' },
)

defineEmits<{ click: [] }>()
</script>

<template>
  <button type="button" :class="cn(buttonVariants({ variant }))" @click="$emit('click')">
    <component :is="icon" v-if="icon" :size="16" />
    <span>{{ label }}</span>
  </button>
</template>
