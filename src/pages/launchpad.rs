use crate::components::launchpad::category_section::CategorySection;
use crate::components::launchpad::filter_bar::FilterBar;
use crate::components::ui::section::UiSection;
use crate::services::launchpad_service::get_categories;
use crate::state::launchpad_state::LaunchpadState;
use crate::types::launchpad::Category;
use crate::utils::launchpad_utils::filter_links;
use leptos::prelude::*;
use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
fn open_url(url: &str) {
    let _ =
        web_sys::window().and_then(|window| window.open_with_url_and_target(url, "_blank").ok());
}

#[cfg(not(target_arch = "wasm32"))]
fn open_url(_url: &str) {}

fn open_all(urls: &[String]) {
    for url in urls {
        open_url(url);
    }
}

#[cfg(target_arch = "wasm32")]
fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(text);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn copy_to_clipboard(_text: &str) {}

#[component]
pub fn LaunchpadPage() -> impl IntoView {
    let state = LaunchpadState::new();
    let categories = Arc::new(get_categories());

    let filter_names = {
        let mut names = vec!["all".to_string()];
        names.extend(categories.iter().map(|c| c.name.clone()));
        names
    };

    let visible_categories = {
        let categories = Arc::clone(&categories);
        let state = state.clone();
        move || {
            let active_filter = state.active_filter.get();
            let query = state.query.get();

            categories
                .iter()
                .filter_map(|category| {
                    if active_filter != "all"
                        && active_filter.to_lowercase() != category.name.to_lowercase()
                    {
                        return None;
                    }

                    let links = filter_links(&category.name, &category.links, &query);
                    if links.is_empty() {
                        return None;
                    }

                    Some(Category {
                        name: category.name.clone(),
                        description: category.description.clone(),
                        links,
                    })
                })
                .collect::<Vec<_>>()
        }
    };

    let open_all_cb = Callback::new(move |urls: Vec<String>| open_all(&urls));
    let open_one_cb = Callback::new(move |url: String| open_url(&url));
    let copy_one_cb = Callback::new(move |url: String| copy_to_clipboard(&url));

    view! {
        <main class="hide-scrollbar mx-auto flex w-full max-w-6xl flex-col gap-8 overflow-y-auto px-4 py-8 sm:px-6 lg:px-8">
            <header class="relative overflow-hidden rounded-[2rem] border border-white/20 bg-gradient-to-br from-slate-950 via-blue-950 to-cyan-900 p-8 text-white shadow-[0_24px_64px_-28px_rgba(2,6,23,0.85)]">
                <div class="pointer-events-none absolute inset-0 opacity-20">
                    <div class="absolute -top-16 left-1/3 h-40 w-40 rounded-full bg-cyan-300 blur-3xl"></div>
                    <div class="absolute -bottom-20 right-0 h-48 w-48 rounded-full bg-blue-400 blur-3xl"></div>
                </div>
                <div class="relative">
                    <p class="text-xs font-semibold uppercase tracking-[0.24em] text-cyan-200/80">"Developer Hub"</p>
                    <h1 class="mt-2 text-3xl font-extrabold tracking-tight sm:text-4xl lg:text-5xl">"Dev Launchpad"</h1>
                    <p class="mt-3 max-w-3xl text-sm text-slate-100/90 sm:text-base">
                        "A modern catalog of projects, tools, and libraries. Search fast, filter by stack, and open workflows in one click."
                    </p>
                </div>
                <FilterBar state=state.clone() filter_names=filter_names />
            </header>

            <section class="space-y-6">
                {move || {
                    let categories = visible_categories();
                    if categories.is_empty() {
                        return view! {
                            <UiSection title="No results" description="Try another keyword or reset the current filter.">
                                <p class="text-sm font-medium text-slate-500 dark:text-slate-400">
                                    "No projects matched your current search and category filter."
                                </p>
                            </UiSection>
                        }
                            .into_any();
                    }

                    categories
                        .into_iter()
                        .map({
                            let state = state.clone();
                            let on_open_all = open_all_cb;
                            let on_open_one = open_one_cb;
                            let on_copy_one = copy_one_cb;
                            move |category| {
                                view! {
                                    <CategorySection
                                        category=category
                                        state=state.clone()
                                        on_open_all=on_open_all
                                        on_open=on_open_one
                                        on_copy=on_copy_one
                                    />
                                }
                            }
                        })
                        .collect_view()
                        .into_any()
                }}
            </section>
        </main>
    }
}
