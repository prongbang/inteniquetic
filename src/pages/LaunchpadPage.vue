<script setup lang="ts">
import { computed } from 'vue'
import Section from '@/components/ui/Section.vue'
import FilterBar from '@/components/launchpad/FilterBar.vue'
import ProductSection from '@/components/launchpad/ProductSection.vue'
import HighlightSection from '@/components/launchpad/HighlightSection.vue'
import CategorySection from '@/components/launchpad/CategorySection.vue'
import { getCategories } from '@/data/categories'
import { filterLinks } from '@/lib/launchpad-utils'
import { useLaunchpadState } from '@/composables/useLaunchpadState'

const { query, activeFilter, isCollapsed, toggleCollapsed } = useLaunchpadState()

const categories = getCategories()
const filterNames = ['all', ...categories.map((c) => c.name)]

const visibleCategories = computed(() =>
  categories
    .map((category) => {
      if (activeFilter.value !== 'all' && activeFilter.value.toLowerCase() !== category.name.toLowerCase()) {
        return null
      }

      const links = filterLinks(category.name, category.links, query.value)
      if (links.length === 0) return null

      return { ...category, links }
    })
    .filter((category): category is NonNullable<typeof category> => category !== null),
)
</script>

<template>
  <main class="hide-scrollbar mx-auto flex w-full max-w-6xl flex-col gap-8 overflow-y-auto px-4 py-8 sm:px-6 lg:px-8">
    <header
      class="relative overflow-hidden rounded-[2rem] border border-white/20 bg-gradient-to-br from-slate-950 via-blue-950 to-cyan-900 p-8 text-white shadow-[0_24px_64px_-28px_rgba(2,6,23,0.85)]"
    >
      <div class="pointer-events-none absolute inset-0 opacity-20">
        <div class="absolute -top-16 left-1/3 h-40 w-40 rounded-full bg-cyan-300 blur-3xl"></div>
        <div class="absolute -bottom-20 right-0 h-48 w-48 rounded-full bg-blue-400 blur-3xl"></div>
      </div>
      <div class="relative">
        <p class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-200/80">Developer Hub</p>
        <h1 class="mt-2 text-3xl font-extrabold tracking-tight sm:text-4xl lg:text-5xl">Dev Launchpad</h1>
        <p class="mt-3 max-w-3xl text-sm text-slate-100/90 sm:text-base">
          A modern catalog of projects, tools, and libraries. Search fast, filter by stack, and open workflows in one
          click.
        </p>
      </div>
      <FilterBar v-model="query" v-model:active-filter="activeFilter" :filter-names="filterNames" />
    </header>

    <ProductSection />

    <HighlightSection />

    <section class="space-y-6">
      <Section
        v-if="visibleCategories.length === 0"
        title="No results"
        description="Try another keyword or reset the current filter."
      >
        <p class="text-sm font-medium text-slate-500 dark:text-slate-400">
          No projects matched your current search and category filter.
        </p>
      </Section>
      <CategorySection
        v-for="category in visibleCategories"
        :key="category.name"
        :category="category"
        :is-collapsed="isCollapsed(category.name)"
        @toggle="toggleCollapsed"
      />
    </section>
  </main>
</template>
