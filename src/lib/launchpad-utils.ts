import type { ProjectLink } from '@/types/launchpad'

export function makeTitle(url: string): string {
  try {
    const parsed = new URL(url)
    const host = parsed.host
    const segments = parsed.pathname.split('/').filter(Boolean)

    if (host.includes('crates.io')) {
      return segments.at(-1) ?? url
    }

    if (host.includes('github.com') && segments.length >= 2) {
      return segments[1]
    }

    return host
  } catch {
    return url
  }
}

export function hostType(url: string): [host: string, kind: string] {
  try {
    const host = new URL(url).host
    if (host.includes('github.com')) return ['github.com', 'GitHub']
    if (host.includes('crates.io')) return ['crates.io', 'Crate']
    return ['external', 'Link']
  } catch {
    return ['unknown', 'Link']
  }
}

export function dedupeLinks(links: ProjectLink[]): ProjectLink[] {
  const seen = new Set<string>()
  return links.filter((link) => {
    if (seen.has(link.url)) return false
    seen.add(link.url)
    return true
  })
}

export function filterLinks(category: string, links: ProjectLink[], query: string): ProjectLink[] {
  const deduped = dedupeLinks(links)
  const q = query.trim().toLowerCase()
  if (!q) return deduped

  return deduped.filter((link) => {
    const title = makeTitle(link.url).toLowerCase()
    return (
      title.includes(q) ||
      link.url.toLowerCase().includes(q) ||
      category.toLowerCase().includes(q)
    )
  })
}
