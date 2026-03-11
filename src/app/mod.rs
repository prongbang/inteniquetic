use crate::components::ui::shell::UiShell;
use crate::pages::launchpad::LaunchpadPage;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <UiShell>
                <Routes fallback=|| "Not found".into_view()>
                    <Route path=path!("/") view=LaunchpadPage />
                    <Route path=path!("/launchpad") view=LaunchpadPage />
                </Routes>
            </UiShell>
        </Router>
    }
}
