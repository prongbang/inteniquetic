use leptos::prelude::*;

#[component]
pub fn UiBadge(#[prop(into)] label: String) -> impl IntoView {
    view! {
        <span class="inline-flex items-center rounded-full border border-slate-200 bg-white px-2.5 py-1 text-xs font-semibold text-slate-700 dark:border-slate-700 dark:bg-slate-800 dark:text-slate-300">
            {label}
        </span>
    }
}
