import { reactive, ref } from 'vue'

export function useLaunchpadState() {
  const query = ref('')
  const activeFilter = ref('all')
  const collapsed = reactive(new Set<string>())

  function toggleCollapsed(name: string) {
    if (collapsed.has(name)) {
      collapsed.delete(name)
    } else {
      collapsed.add(name)
    }
  }

  function isCollapsed(name: string) {
    return collapsed.has(name)
  }

  return { query, activeFilter, isCollapsed, toggleCollapsed }
}
