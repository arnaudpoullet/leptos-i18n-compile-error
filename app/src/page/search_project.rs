use leptos::IntoView;
use leptos::prelude::*;
use crate::i18n::*;
use leptos_i18n::t;

/// Renders the home page of your application.
#[component]
pub fn SearchProject() -> impl IntoView {
    // Creates a reactive value to update the button
    let i18n = use_i18n();

    view! {
        <div class="m-2">
            <h1 class="text-2xl">{{t!(i18n,search_project.search_project_title)}}</h1>
        </div>
    }
}