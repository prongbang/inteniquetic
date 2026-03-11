use crate::components::launchpad::project_card::ProjectCard;
use crate::components::ui::badge::UiBadge;
use crate::components::ui::button::{ButtonVariant, UiButton};
use crate::components::ui::section::UiSection;
use crate::state::launchpad_state::LaunchpadState;
use crate::types::launchpad::Category;
use leptos::prelude::*;

#[component]
pub fn CategorySection(
    category: Category,
    state: LaunchpadState,
    on_open_all: Callback<Vec<String>>,
    on_open: Callback<String>,
    on_copy: Callback<String>,
) -> impl IntoView {
    let category_name = category.name.clone();
    let category_name_for_toggle = category.name.clone();
    let state_for_toggle = state.clone();
    let state_for_label = state.clone();
    let state_for_show = state.clone();
    let links = category.links.clone();
    let links_for_cards = links.clone();
    let links_for_open_all = links.iter().map(|link| link.url.clone()).collect::<Vec<_>>();

    view! {
        <UiSection title=category.name.clone() description=category.description.clone()>
            <div class="mb-4 flex flex-wrap items-center justify-between gap-3">
                <div class="flex items-center gap-2">
                    <UiBadge label=format!("{} links", links.len()) />
                    <UiButton
                        label="Open all".to_string()
                        variant=ButtonVariant::Primary
                        icon="material-icon-theme:folder-global"
                        on_click=Callback::new({
                            let urls = links_for_open_all.clone();
                            move |_| on_open_all.run(urls.clone())
                        })
                    />
                </div>
                <UiButton
                    label=move || {
                        if state_for_label.is_collapsed(&category_name) {
                            "Expand".to_string()
                        } else {
                            "Collapse".to_string()
                        }
                    }
                    variant=ButtonVariant::Secondary
                    icon="material-icon-theme:histoire"
                    on_click=Callback::new(move |_| {
                        state_for_toggle.toggle_collapsed(&category_name_for_toggle)
                    })
                />
            </div>

            <Show
                when={
                    let category_name = category.name.clone();
                    move || !state_for_show.is_collapsed(&category_name)
                }
                fallback=|| view! { <p class="text-sm font-medium text-slate-500 dark:text-slate-400">"Category is collapsed"</p> }
            >
                <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                    <For
                        each={
                            let links_for_cards = links_for_cards.clone();
                            move || links_for_cards.clone()
                        }
                        key=|link| link.url.clone()
                        children={
                            let on_open = on_open.clone();
                            let on_copy = on_copy.clone();
                            move |link| {
                                view! {
                                    <ProjectCard
                                        url=link.url
                                        on_open=on_open
                                        on_copy=on_copy
                                    />
                                }
                            }
                        }
                    />
                </div>
            </Show>
        </UiSection>
    }
}
