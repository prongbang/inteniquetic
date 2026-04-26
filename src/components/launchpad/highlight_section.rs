use crate::components::ui::badge::UiBadge;
use crate::components::ui::button::{ButtonVariant, UiButton};
use leptos::prelude::*;

#[derive(Clone, Copy)]
struct HighlightProject {
    title: &'static str,
    platform: &'static str,
    description: &'static str,
    url: &'static str,
}

const HIGHLIGHT_PROJECTS: &[HighlightProject] = &[
    HighlightProject {
        title: "screen_protector",
        platform: "Flutter",
        description: "Flutter plugin for protecting sensitive app screens from screenshots and screen recording.",
        url: "https://github.com/prongbang/screen_protector",
    },
    HighlightProject {
        title: "ScreenProtectorKit",
        platform: "Swift",
        description: "Native iOS kit for adding screen protection behavior to privacy-sensitive screens.",
        url: "https://github.com/prongbang/ScreenProtectorKit",
    },
];

#[component]
pub fn HighlightSection(on_open: Callback<String>, on_copy: Callback<String>) -> impl IntoView {
    view! {
        <section class="relative overflow-hidden rounded-[2rem] border border-cyan-200/70 bg-gradient-to-br from-cyan-50 via-white to-slate-100 p-5 shadow-[0_20px_56px_-32px_rgba(8,145,178,0.55)] dark:border-cyan-900/70 dark:from-slate-950 dark:via-slate-900 dark:to-cyan-950/70">
            <div class="pointer-events-none absolute -right-14 -top-16 h-40 w-40 rounded-full bg-cyan-300/40 blur-3xl dark:bg-cyan-500/20"></div>
            <div class="pointer-events-none absolute -bottom-20 left-8 h-40 w-40 rounded-full bg-blue-400/30 blur-3xl dark:bg-blue-500/20"></div>

            <div class="relative space-y-5">
                <header class="flex flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
                    <div>
                        <p class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-700 dark:text-cyan-300">"Highlight"</p>
                        <h2 class="mt-1 text-2xl font-extrabold tracking-tight text-slate-950 dark:text-white">
                            "Featured Libraries"
                        </h2>
                        <p class="mt-2 max-w-2xl text-sm text-slate-600 dark:text-slate-300">
                            "A curated set of standout Flutter and iOS libraries across security, privacy, and mobile UX."
                        </p>
                    </div>
                    <UiBadge label=format!("{} projects", HIGHLIGHT_PROJECTS.len()) />
                </header>

                <div class="grid gap-4 md:grid-cols-2">
                    <For
                        each=move || HIGHLIGHT_PROJECTS.to_vec()
                        key=|project| project.url
                        children={
                            let on_open = on_open.clone();
                            let on_copy = on_copy.clone();
                            move |project| {
                                let open_url = project.url.to_string();
                                let copy_url = project.url.to_string();

                                view! {
                                    <article class="group relative overflow-hidden rounded-3xl border border-white/80 bg-white/90 p-5 shadow-[0_18px_44px_-28px_rgba(15,23,42,0.55)] backdrop-blur transition hover:-translate-y-0.5 hover:shadow-[0_26px_54px_-28px_rgba(8,145,178,0.55)] dark:border-slate-800/80 dark:bg-slate-900/80">
                                        <div class="absolute right-4 top-4 text-cyan-500/20 transition group-hover:scale-110 dark:text-cyan-300/20">
                                            <iconify-icon icon="material-icon-theme:shield" width="72" height="72"></iconify-icon>
                                        </div>

                                        <div class="relative space-y-4">
                                            <div class="flex items-center gap-2">
                                                <UiBadge label=project.platform.to_string() />
                                                <UiBadge label="GitHub".to_string() />
                                            </div>

                                            <div>
                                                <h3 class="text-lg font-extrabold tracking-tight text-slate-950 dark:text-white">
                                                    {project.title}
                                                </h3>
                                                <p class="mt-2 min-h-12 text-sm leading-6 text-slate-600 dark:text-slate-300">
                                                    {project.description}
                                                </p>
                                            </div>

                                            <p class="truncate rounded-2xl border border-slate-200/80 bg-slate-50 px-3 py-2 text-xs font-medium text-slate-500 dark:border-slate-800 dark:bg-slate-950/70 dark:text-slate-400">
                                                {project.url}
                                            </p>

                                            <div class="flex flex-wrap items-center gap-2">
                                                <UiButton
                                                    label="Open".to_string()
                                                    variant=ButtonVariant::Primary
                                                    icon="material-icon-theme:folder-docs-open"
                                                    on_click=Callback::new(move |_| on_open.run(open_url.clone()))
                                                />
                                                <UiButton
                                                    label="Copy".to_string()
                                                    variant=ButtonVariant::Secondary
                                                    icon="material-icon-theme:folder-docs"
                                                    on_click=Callback::new(move |_| on_copy.run(copy_url.clone()))
                                                />
                                            </div>
                                        </div>
                                    </article>
                                }
                            }
                        }
                    />
                </div>
            </div>
        </section>
    }
}
