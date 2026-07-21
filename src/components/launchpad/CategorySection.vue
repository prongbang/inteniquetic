<script setup lang="ts">
import { ChevronDown, ChevronUp, Globe } from '@lucide/vue'
import Badge from '@/components/ui/Badge.vue'
import Button from '@/components/ui/Button.vue'
import Section from '@/components/ui/Section.vue'
import ProjectCard from '@/components/launchpad/ProjectCard.vue'
import type { Category } from '@/types/launchpad'

const props = defineProps<{
  category: Category
  isCollapsed: boolean
}>()

const emit = defineEmits<{ toggle: [name: string] }>()

function openAll() {
  props.category.links.forEach((link) => window.open(link.url, '_blank'))
}
</script>

<template>
  <Section :title="category.name" :description="category.description">
    <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
      <div class="flex items-center gap-2">
        <Badge :label="`${category.links.length} links`" />
        <Button label="Open all" variant="primary" :icon="Globe" @click="openAll" />
      </div>
      <Button
        :label="isCollapsed ? 'Expand' : 'Collapse'"
        variant="secondary"
        :icon="isCollapsed ? ChevronDown : ChevronUp"
        @click="emit('toggle', category.name)"
      />
    </div>

    <p v-if="isCollapsed" class="text-sm font-medium text-slate-500 dark:text-slate-400">Category is collapsed</p>
    <div v-else class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      <ProjectCard v-for="link in category.links" :key="link.url" :url="link.url" />
    </div>
  </Section>
</template>
