<script setup lang="ts">
import { ref } from 'vue'
import { Check, Copy, ExternalLink } from '@lucide/vue'
import Badge from '@/components/ui/Badge.vue'
import Button from '@/components/ui/Button.vue'
import { hostType, makeTitle } from '@/lib/launchpad-utils'

const props = defineProps<{ url: string }>()

const title = makeTitle(props.url)
const [host, kind] = hostType(props.url)

const copied = ref(false)

function open() {
  window.open(props.url, '_blank')
}

async function copy() {
  try {
    await navigator.clipboard.writeText(props.url)
  } catch {
    return
  }
  copied.value = true
  setTimeout(() => {
    copied.value = false
  }, 1500)
}
</script>

<template>
  <article
    class="space-y-3 rounded-2xl border border-slate-200/80 bg-gradient-to-b from-white to-slate-50 p-4 shadow-[0_14px_34px_-24px_rgba(15,23,42,0.45)] transition hover:-translate-y-0.5 hover:shadow-[0_20px_40px_-24px_rgba(15,23,42,0.6)] dark:border-slate-800/80 dark:from-slate-900 dark:to-slate-950/60"
  >
    <div class="flex items-center gap-2 text-xs text-slate-500 dark:text-slate-400">
      <Badge :label="kind" />
      <span>{{ host }}</span>
    </div>
    <h3 class="truncate text-sm font-bold text-slate-900 dark:text-slate-100">{{ title }}</h3>
    <p class="truncate text-xs text-slate-500 dark:text-slate-400">{{ url }}</p>
    <div class="flex items-center gap-2">
      <Button label="Open" variant="primary" :icon="ExternalLink" @click="open" />
      <Button :label="copied ? 'Copied' : 'Copy'" variant="secondary" :icon="copied ? Check : Copy" @click="copy" />
    </div>
  </article>
</template>
