export interface ProjectLink {
  url: string
}

export interface Category {
  name: string
  description: string
  links: ProjectLink[]
}
