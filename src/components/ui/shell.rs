use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
fn initial_theme_mode() -> String {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(saved_mode)) = storage.get_item("theme") {
                if saved_mode == "dark" {
                    return "dark".to_string();
                }
            }
        }
    }
    "light".to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn initial_theme_mode() -> String {
    "light".to_string()
}

#[cfg(target_arch = "wasm32")]
fn apply_theme_mode(mode: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(root) = document.document_element() {
                let classes = root.class_list();
                if mode == "dark" {
                    let _ = classes.add_1("dark");
                } else {
                    let _ = classes.remove_1("dark");
                }
            }
        }
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item("theme", mode);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn apply_theme_mode(_mode: &str) {}

#[component]
pub fn UiShell(children: Children) -> impl IntoView {
    let theme_mode = RwSignal::new(initial_theme_mode());

    Effect::new(move |_| {
        apply_theme_mode(&theme_mode.get());
    });

    view! {
        <div class="min-h-screen bg-gradient-to-b from-slate-50 via-white to-slate-100 text-slate-900 dark:from-slate-950 dark:via-slate-950 dark:to-slate-900 dark:text-slate-100">
            <header class="sticky top-0 z-30 border-b border-slate-200/80 bg-white/80 shadow-sm backdrop-blur-xl dark:border-slate-800/80 dark:bg-slate-900/70">
                <div class="mx-auto flex w-full max-w-6xl items-center justify-between gap-4 px-4 py-4 sm:px-6 lg:px-8">
                    <div class="flex items-center gap-2">
                        <iconify-icon icon="material-icon-theme:folder-ui" width="18" height="18"></iconify-icon>
                        <div>
                        <p class="text-[10px] font-semibold uppercase tracking-[0.24em] text-slate-500 dark:text-slate-400">"INTENIQUETIC"</p>
                        <h1 class="text-sm font-bold text-slate-900 dark:text-slate-100">"Launchpad"</h1>
                        </div>
                    </div>
                    <div class="flex items-center gap-2">
                        <Show
                            when=move || theme_mode.get() == "dark"
                            fallback=move || {
                                view! {
                                    <button
                                        class="inline-flex h-9 w-9 items-center justify-center rounded-xl border border-slate-300/80 bg-white text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-500/60 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
                                        aria-label="Dark mode"
                                        title="Dark mode"
                                        on:click=move |_| {
                                            theme_mode.update(|mode| {
                                                if mode == "dark" {
                                                    *mode = "light".to_string();
                                                } else {
                                                    *mode = "dark".to_string();
                                                }
                                            })
                                        }
                                    >
                                        <iconify-icon icon="material-icon-theme:moon" width="18" height="18"></iconify-icon>
                                    </button>
                                }
                            }
                        >
                            <button
                                class="inline-flex h-9 w-9 items-center justify-center rounded-xl border border-slate-300/80 bg-white text-slate-700 shadow-sm transition hover:-translate-y-0.5 hover:bg-slate-50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-cyan-500/60 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-200 dark:hover:bg-slate-800"
                                aria-label="Light mode"
                                title="Light mode"
                                on:click=move |_| {
                                    theme_mode.update(|mode| {
                                        if mode == "dark" {
                                            *mode = "light".to_string();
                                        } else {
                                            *mode = "dark".to_string();
                                        }
                                    })
                                }
                            >
                                <iconify-icon icon="material-icon-theme:svg" width="18" height="18"></iconify-icon>
                            </button>
                        </Show>
                    </div>
                </div>
            </header>

            {children()}

            <footer class="border-t border-slate-200/80 bg-white/70 backdrop-blur dark:border-slate-800/80 dark:bg-slate-900/70">
                <div class="mx-auto w-full max-w-6xl px-4 py-4 text-xs text-slate-500 dark:text-slate-400 sm:px-6 lg:px-8">
                    "2026 inteniquetic.com"
                </div>
            </footer>
        </div>
    }
}
