use crate::components::ui::input::UiSearchInput;
use crate::state::launchpad_state::LaunchpadState;
use leptos::prelude::*;

#[component]
pub fn FilterBar(state: LaunchpadState, filter_names: Vec<String>) -> impl IntoView {
    view! {
        <div class="mt-6 grid gap-3 sm:grid-cols-3">
            <div class="sm:col-span-2">
                <UiSearchInput query=state.query placeholder="Search projects or categories" />
            </div>
            <div class="flex flex-wrap items-center gap-2">
                {filter_names
                    .into_iter()
                    .map({
                        let state = state.clone();
                        move |name| {
                            let filter_value = name.clone();
                            let filter_value_for_class = filter_value.clone();
                            let filter_value_for_click = filter_value.clone();
                            let label = if name == "all" {
                                "All".to_string()
                            } else {
                                name
                            };
                            view! {
                                <button
                                    class=move || {
                                        let is_active = state.active_filter.get() == filter_value_for_class;
                                        if is_active {
                                            "rounded-full border border-white/30 bg-white/20 px-4 py-2 text-xs font-semibold uppercase tracking-wide text-white shadow-sm backdrop-blur transition"
                                        } else {
                                            "rounded-full border border-transparent bg-slate-100/10 px-4 py-2 text-xs font-semibold uppercase tracking-wide text-white/90 backdrop-blur transition hover:bg-slate-100/20"
                                        }
                                    }
                                    on:click={
                                        let state = state.clone();
                                        let filter_value = filter_value_for_click.clone();
                                        move |_| state.active_filter.set(filter_value.clone())
                                    }
                                >
                                    {label}
                                </button>
                            }
                        }
                    })
                    .collect_view()}
            </div>
        </div>
    }
}
