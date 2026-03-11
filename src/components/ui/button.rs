use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
}

#[component]
pub fn UiButton(
    #[prop(into)] label: Signal<String>,
    #[prop(default = ButtonVariant::Secondary)] variant: ButtonVariant,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional)] on_click: Option<Callback<()>>,
) -> impl IntoView {
    let class = match variant {
        ButtonVariant::Primary => {
            "rounded-xl bg-slate-900 px-4 py-2 text-sm font-semibold text-white shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-700 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-500/60 dark:bg-slate-100 dark:text-slate-900 dark:hover:bg-slate-200"
        }
        ButtonVariant::Secondary => {
            "rounded-xl border border-slate-300/80 bg-white px-4 py-2 text-sm font-semibold text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-500/60 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
        }
    };

    let click = move |_| {
        if let Some(cb) = on_click.as_ref() {
            cb.run(());
        }
    };
    let icon_name = icon.unwrap_or_default();
    let has_icon = !icon_name.is_empty();

    view! {
        <button class=class on:click=click>
            <span class="inline-flex items-center gap-2">
                {if has_icon {
                    view! { <iconify-icon icon=icon_name width="16" height="16"></iconify-icon> }.into_any()
                } else {
                    ().into_any()
                }}
                <span>{move || label.get()}</span>
            </span>
        </button>
    }
}
