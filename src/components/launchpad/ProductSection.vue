<script setup lang="ts">
import { ref } from 'vue'
import { Check, Copy, ExternalLink, Image, Smartphone } from '@lucide/vue'
import type { Component } from 'vue'
import Badge from '@/components/ui/Badge.vue'
import Button from '@/components/ui/Button.vue'

interface Product {
  name: string
  category: string
  summary: string
  url: string
  icon: Component
  highlights: string[]
}

const products: Product[] = [
  {
    name: 'AI Upscaler',
    category: 'Media Tool',
    summary:
      'AI-powered image and video upscaling with clean enhancement workflows and 2x, 3x, and 4x output options.',
    url: 'https://prongbang.github.io/ai-upscaler/',
    icon: Image,
    highlights: ['Image & video', 'AI enhancement', '2x / 3x / 4x'],
  },
  {
    name: 'Tools',
    category: 'Android App',
    summary:
      'A local-first utility app for everyday file work, covering PDF, QR, image, and Zip tools without account or cloud dependency.',
    url: 'https://play.google.com/store/apps/details?id=com.inteniquetic.tools&hl=en',
    icon: Smartphone,
    highlights: ['PDF / QR / Image / Zip', 'Works offline', 'No account'],
  },
]

const copiedUrl = ref<string | null>(null)

function open(url: string) {
  window.open(url, '_blank')
}

async function copy(url: string) {
  try {
    await navigator.clipboard.writeText(url)
  } catch {
    return
  }
  copiedUrl.value = url
  setTimeout(() => {
    if (copiedUrl.value === url) copiedUrl.value = null
  }, 1500)
}
</script>

<template>
  <section
    class="animate-in fade-in slide-in-from-bottom-3 relative overflow-hidden rounded-[2rem] border border-emerald-200/70 bg-gradient-to-br from-white via-emerald-50 to-sky-50 p-5 shadow-[0_22px_58px_-34px_rgba(16,185,129,0.5)] duration-500 dark:border-emerald-900/70 dark:from-slate-950 dark:via-emerald-950/40 dark:to-slate-900"
  >
    <div
      class="pointer-events-none absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-emerald-400/70 to-transparent"
    ></div>
    <div
      class="pointer-events-none absolute bottom-0 left-0 h-28 w-full bg-[radial-gradient(circle_at_20%_100%,rgba(16,185,129,0.18),transparent_32%),radial-gradient(circle_at_85%_20%,rgba(14,165,233,0.18),transparent_28%)]"
    ></div>

    <div class="relative space-y-5">
      <header class="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
        <div>
          <p class="text-xs font-semibold uppercase tracking-[0.24em] text-emerald-700 dark:text-emerald-300">
            Product
          </p>
          <h2 class="mt-1 text-2xl font-extrabold tracking-tight text-slate-950 dark:text-white">Built Products</h2>
          <p class="mt-2 max-w-2xl text-sm text-slate-600 dark:text-slate-300">
            Finished tools users can open today, from AI media enhancement to practical mobile utilities.
          </p>
        </div>
        <Badge :label="`${products.length} products`" />
      </header>

      <div class="grid gap-4 lg:grid-cols-2">
        <article
          v-for="product in products"
          :key="product.url"
          class="group overflow-hidden rounded-3xl border border-white/80 bg-white/90 shadow-[0_18px_46px_-30px_rgba(15,23,42,0.55)] transition hover:-translate-y-0.5 hover:shadow-[0_28px_58px_-32px_rgba(16,185,129,0.52)] dark:border-slate-800/80 dark:bg-slate-900/80"
        >
          <div class="flex h-full flex-col gap-5 p-5">
            <div class="flex items-start justify-between gap-4">
              <div class="flex min-w-0 items-center gap-3">
                <div
                  class="grid h-12 w-12 shrink-0 place-items-center rounded-2xl border border-emerald-200 bg-emerald-50 text-emerald-700 shadow-inner dark:border-emerald-900/80 dark:bg-emerald-950/70 dark:text-emerald-300"
                >
                  <component :is="product.icon" :size="26" />
                </div>
                <div class="min-w-0">
                  <h3 class="truncate text-lg font-extrabold tracking-tight text-slate-950 dark:text-white">
                    {{ product.name }}
                  </h3>
                  <p class="mt-1 text-xs font-semibold uppercase tracking-[0.16em] text-emerald-700 dark:text-emerald-300">
                    {{ product.category }}
                  </p>
                </div>
              </div>
              <Badge label="Product" />
            </div>

            <p class="min-h-16 text-sm leading-6 text-slate-600 dark:text-slate-300">{{ product.summary }}</p>

            <div class="flex flex-wrap gap-2">
              <span
                v-for="highlight in product.highlights"
                :key="highlight"
                class="rounded-full border border-emerald-200/80 bg-emerald-50 px-3 py-1 text-xs font-semibold text-emerald-800 dark:border-emerald-900/80 dark:bg-emerald-950/70 dark:text-emerald-200"
              >
                {{ highlight }}
              </span>
            </div>

            <p
              class="truncate rounded-2xl border border-slate-200/80 bg-slate-50 px-3 py-2 text-xs font-medium text-slate-500 dark:border-slate-800 dark:bg-slate-950/70 dark:text-slate-400"
            >
              {{ product.url }}
            </p>

            <div class="mt-auto flex flex-wrap items-center gap-2">
              <Button label="Open product" variant="primary" :icon="ExternalLink" @click="open(product.url)" />
              <Button
                :label="copiedUrl === product.url ? 'Copied' : 'Copy link'"
                variant="secondary"
                :icon="copiedUrl === product.url ? Check : Copy"
                @click="copy(product.url)"
              />
            </div>
          </div>
        </article>
      </div>
    </div>
  </section>
</template>
