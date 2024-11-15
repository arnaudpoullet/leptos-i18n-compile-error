pub mod page;
pub mod component;

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::*;
use leptos_router::components::{Router,Routes,Route,A};
use leptos_router::nested_router::Outlet;

use crate::page::home::HomePage;
use crate::page::search_project::SearchProject;
use crate::page::submit_project::SubmitProject;
use crate::component::svg::Logo;
use crate::i18n::*;
use leptos_i18n::{t,t_string,td_string,i18n_path};

leptos_i18n::load_locales!();

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <I18nContextProvider>
        <Stylesheet id="leptos" href="/pkg/leptos-i18n-compile-error.css"/>
        // sets the document title
        <Title text="Welcome to my website"/>

        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <I18nRoute view=|| view! {
                            <main class="mt-20">
                                <Header />
                                <Outlet />
                            </main>
                        } >
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=i18n_path!(Locale, |locale| td_string!(locale, common.menu.search_project_path)) view=SearchProject/>
                    <Route path=i18n_path!(Locale, |locale| td_string!(locale, common.menu.submit_project_path)) view=SubmitProject/>
                </I18nRoute>
            </Routes>
        </Router>
        </I18nContextProvider>
    }
}

#[component]
fn Header() -> impl IntoView {
    let show = RwSignal::new(false);

    let i18n = use_i18n();

    let locale = move || i18n.get_locale();

    let other_languages = move || Locale::get_all().iter().filter(|l| *l != &locale()).collect::<Vec<_>>();

    view!{
        <header class="absolute inset-x-0 top-0 z-50 pt-2">
                <nav class="flex items-center justify-between px-6 h-[72px] lg:px-8" aria-label="Global">
                  <div class="flex lg:flex-1">
                    <A href="/">
                      <span class="sr-only">"Linking Website"</span>
                        <div style="height:64px; width:112px;">
                            <Logo />
                        </div>
                    </A>
                  </div>
                  <div class="flex lg:hidden">
                    <button type="button" on:click=move |_| show.set(true) class="-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-700">
                      <span class="sr-only">Open main menu</span>
                      <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
                      </svg>
                    </button>
                  </div>
                  <div class="hidden lg:flex lg:gap-x-12 lg:px-8">
                    <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.search_project_path)) exact=true {..} class="text-sm/6 font-semibold text-gray-900">{t!(i18n,common.menu.search_project_label)}</A>
                    <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.submit_project_path)) exact=true {..} class="text-sm/6 font-semibold text-gray-900">{t!(i18n,common.menu.submit_project_label)}</A>
                    <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.featured_projects_path)) exact=true {..} class="text-sm/6 font-semibold text-gray-900">{t!(i18n,common.menu.featured_projects_label)}</A>
                    <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.contact_path)) exact=true {..} class="text-sm/6 font-semibold text-gray-900">{t!(i18n,common.menu.contact_label)}</A>
                  </div>
                </nav>
                // Mobile menu, show/hide based on menu open state.
                <div class:hidden=move || !show.get() class="lg:hidden" role="dialog" aria-modal="true">
                  // Background backdrop, show/hide based on slide-over state.
                  <div class="fixed inset-0 z-50 bg-light-color opacity-50"></div>
                  <div class="fixed inset-y-0 right-0 z-50 w-full overflow-y-auto bg-white px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10">
                    <div class="flex items-center justify-between">
                      <A href="/">
                        <span class="sr-only">"Linking Website"</span>
                        <div style="height:64px; width:112px;">
                            <Logo />
                        </div>
                      </A>
                      <button type="button" on:click=move |_| show.set(false) class="-m-2.5 rounded-md p-2.5 text-gray-700">
                        <span class="sr-only">Close menu</span>
                        <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" data-slot="icon">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M6 18 18 6M6 6l12 12" />
                        </svg>
                      </button>
                    </div>
                    <div class="mt-6 flow-root">
                      <div class="-my-6 divide-y divide-gray-500/10">
                        <div class="space-y-2 py-6">
                          <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.search_project_path)) exact=true on:click=move |_| show.set(false) {..} class="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">{t!(i18n,common.menu.search_project_label)}</A>
                          <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.submit_project_path)) exact=true on:click=move |_| show.set(false) {..} class="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">{t!(i18n,common.menu.submit_project_label)}</A>
                          <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.featured_projects_path)) exact=true on:click=move |_| show.set(false) {..} class="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">{t!(i18n,common.menu.featured_projects_label)}</A>
                          <A href=move || format!("/{}/{}",locale(),t_string!(i18n,common.menu.contact_path)) exact=true on:click=move |_| show.set(false) {..} class="-mx-3 block rounded-lg px-3 py-2 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">{t!(i18n,common.menu.contact_label)}</A>
                        </div>
                        <div class="py-6">
                            <For
                                  each = other_languages
                                  key = |l| **l
                                  let:locale
                                >
                                    <A href=format!("/{}",locale.as_str()) {..} class="-mx-3 block rounded-lg px-3 py-2.5 text-base/7 font-semibold text-gray-900 hover:bg-gray-50">{locale.as_str()}</A>
                                </For>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </header>
    }
}