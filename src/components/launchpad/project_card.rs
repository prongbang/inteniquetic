use crate::components::ui::badge::UiBadge;
use crate::components::ui::button::{ButtonVariant, UiButton};
use crate::utils::launchpad_utils::{host_type, make_title};
use leptos::prelude::*;

#[component]
pub fn ProjectCard(
    #[prop(into)] url: String,
    on_open: Callback<String>,
    on_copy: Callback<String>,
) -> impl IntoView {
    let title = make_title(&url);
    let (host, kind) = host_type(&url);

    let open_url = url.clone();
    let copy_url = url.clone();

    view! {
        <article class="space-y-3 rounded-2xl border border-slate-200/80 bg-gradient-to-b from-white to-slate-50 p-4 shadow-[0_14px_34px_-24px_rgba(15,23,42,0.45)] transition hover:-translate-y-0.5 hover:shadow-[0_20px_40px_-24px_rgba(15,23,42,0.6)] dark:border-slate-800/80 dark:from-slate-900 dark:to-slate-950/60">
            <div class="flex items-center gap-2 text-xs text-slate-500 dark:text-slate-400">
                <UiBadge label=kind.to_string() />
                <span>{host}</span>
            </div>
            <h3 class="truncate text-sm font-bold text-slate-900 dark:text-slate-100">{title}</h3>
            <p class="truncate text-xs text-slate-500 dark:text-slate-400">{url}</p>
            <div class="flex items-center gap-2">
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
        </article>
    }
}
