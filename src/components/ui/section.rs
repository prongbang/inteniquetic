use leptos::prelude::*;

#[component]
pub fn UiSection(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="space-y-4 rounded-3xl border border-slate-200/80 bg-white/85 p-5 shadow-[0_12px_40px_-20px_rgba(15,23,42,0.35)] backdrop-blur dark:border-slate-800/80 dark:bg-slate-900/75">
            <header>
                <h2 class="text-xl font-extrabold tracking-tight text-slate-900 dark:text-slate-100">{title}</h2>
                <p class="mt-1 text-sm text-slate-600 dark:text-slate-400">{description}</p>
            </header>
            {children()}
        </section>
    }
}
