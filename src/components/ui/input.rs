use leptos::prelude::*;

#[component]
pub fn UiSearchInput(
    query: RwSignal<String>,
    #[prop(into)] placeholder: String,
) -> impl IntoView {
    view! {
        <input
            type="search"
            placeholder=placeholder
            class="w-full rounded-2xl border border-white/30 bg-white/85 px-4 py-3 text-sm text-slate-900 shadow-[inset_0_1px_0_rgba(255,255,255,0.6)] outline-none ring-0 transition placeholder:text-slate-500 focus:border-cyan-400 focus:ring-4 focus:ring-cyan-500/20 dark:border-slate-700 dark:bg-slate-900/80 dark:text-slate-100 dark:placeholder-slate-500 dark:focus:border-cyan-500 dark:focus:ring-cyan-500/20"
            prop:value=move || query.get()
            on:input=move |ev| query.set(event_target_value(&ev))
        />
    }
}
