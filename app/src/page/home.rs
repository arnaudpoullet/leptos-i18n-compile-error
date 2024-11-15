use leptos::IntoView;
use leptos::prelude::*;
use crate::i18n::*;
use crate::component::svg::{Handshake,PlantInHand,Target,Fist, Camera, Lightbulb, BankNotes,Magnifier,PaperPlane};
use crate::component::swiper::{Swiper,CardData};
use leptos_i18n::t;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {

    let i18n = use_i18n();

    let cards = vec![
        CardData {
            idx: 0,
            title: "Card 1",
            description: "This is the first card.",
        },
        CardData {
            idx: 1,
            title: "Card 2",
            description: "This is the second card.",
        },
        CardData {
            idx: 2,
            title: "Card 3",
            description: "This is the third card.",
        },
    ];

    view! {
        <div class="">
            <div class="relative isolate overflow-hidden bg-gray-900 py-24 sm:py-32">
              <img src="https://images.unsplash.com/photo-1521737604893-d14cc237f11d?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&crop=focalpoint&fp-y=.8&w=2830&h=1500&q=80&blend=111827&sat=-100&exp=15&blend-mode=multiply" alt="" class="absolute inset-0 -z-10 h-full w-full object-cover"/>
              <div class="hidden sm:absolute sm:-top-10 sm:right-1/2 sm:-z-10 sm:mr-10 sm:block sm:transform-gpu sm:blur-3xl" aria-hidden="true">
                <div class="aspect-[1097/845] w-[68.5625rem] bg-gradient-to-tr from-[#ff4694] to-[#776fff] opacity-20" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
              </div>
              <div class="absolute -top-52 left-1/2 -z-10 -translate-x-1/2 transform-gpu blur-3xl sm:top-[-28rem] sm:ml-16 sm:translate-x-0 sm:transform-gpu" aria-hidden="true">
                <div class="aspect-[1097/845] w-[68.5625rem] bg-gradient-to-tr from-[#ff4694] to-[#776fff] opacity-20" style="clip-path: polygon(74.1% 44.1%, 100% 61.6%, 97.5% 26.9%, 85.5% 0.1%, 80.7% 2%, 72.5% 32.5%, 60.2% 62.4%, 52.4% 68.1%, 47.5% 58.3%, 45.2% 34.5%, 27.5% 76.7%, 0.1% 64.9%, 17.9% 100%, 27.6% 76.8%, 76.1% 97.7%, 74.1% 44.1%)"></div>
              </div>
              <div class="mx-auto max-w-7xl px-6 lg:px-8">
                <div class="mx-auto max-w-7xl lg:mx-0 text-right">
                  <h2 class="text-5xl font-semibold tracking-tight text-white sm:text-7xl">{ t!(i18n,home.banner.title) }</h2>
                </div>
              </div>
            </div>
            <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 pb-16 pt-20 text-center lg:pt-28">
                <h2 class="mx-auto max-w-4xl sm:max-w-6xl font-display text-4xl font-medium tracking-tight text-slate-900 sm:text-6xl">{t!(i18n,home.sub_header_paragraph, <b> = <span class="text-accent-color"/ > )}</h2>
            </div>
            <div class="relative">
                <div class="flex sm:flex-row flex-col justify-center items-center space-x-4 p-4">
                    <div class="overflow-hidden rounded-lg bg-main-color shadow">
                      <div class="px-4 py-5 sm:p-6 text-white text-2xl sm:text-4xl text-center">
                         {t!(i18n,home.handshake_textbox_1) }
                      </div>
                    </div>
                    <Handshake {..} style="width:96px; height:96px" class="text-main-color"/>
                    <div class="overflow-hidden rounded-lg bg-main-color shadow">
                      <div class="px-4 py-5 sm:p-6 text-white text-2xl sm:text-4xl text-center">
                         {t!(i18n,home.handshake_textbox_2) }
                      </div>
                    </div>
                </div>
            </div>
            <div class="p-6 mt-6">
                <h1 class="text-2xl text-gray-800 font-bold mb-4">{t!(i18n,home.cards.title) }</h1>
                <Swiper cards=cards />
            </div>
            <section class="px-6 py-12 sm:py-20 bg-light-color">
                <div>
                    <h2 class="mx-auto max-w-4xl sm:max-w-6xl font-display text-4xl font-medium tracking-tight text-main-color sm:text-6xl">{t!(i18n,home.company.title, <b> = <span class="text-accent-color"/ > )}</h2>
                    <h3 class="mx-auto max-w-4xl sm:max-w-6xl font-display text-center text-2xl tracking-tight text-main-color sm:text-4xl py-6 sm:py-12">{t!(i18n,home.company.sub_title, <b> = <span class="font-bold"/> )}</h3>
                    <div class="grid grid-cols-1 xs:grid-cols-2 md:grid-cols-3 gap-4 sm:gap-8 max-w-xs xs:max-w-md sm:max-w-4xl mx-auto">
                      <div class="flex flex-row justify-start items-center space-x-4">
                            <PlantInHand {..} style="width:72px; height:72px" class="sm:w-24 text-main-color"/>
                            <p class="text-gray-700">{t!(i18n,home.company.icon1_label) }</p>
                        </div>
                      <div class="flex flex-row justify-start items-center space-x-4">
                            <Target {..} style="width:72px; height:72px" class="sm:w-24 text-main-color"/>
                            <p class="text-gray-700">{t!(i18n,home.company.icon2_label) }</p>
                        </div>
                      <div class="flex flex-row justify-start items-center space-x-4">
                            <Fist {..} style="width:72px; height:72px" class="sm:w-24 text-main-color"/>
                            <p class="text-gray-700">{t!(i18n,home.company.icon3_label) }</p>
                        </div>
                      <div class="flex flex-row justify-start items-center space-x-4">
                            <Lightbulb {..} style="width:72px; height:72px" class="sm:w-24 text-main-color"/>
                            <p class="text-gray-700">{t!(i18n,home.company.icon4_label) }</p>
                        </div>
                      <div class="flex flex-row justify-start items-center space-x-4">
                            <BankNotes {..} style="width:72px; height:72px" class="sm:w-24 text-main-color"/>
                            <p class="text-gray-700">{t!(i18n,home.company.icon5_label) }</p>
                        </div>
                      <div class="flex flex-row justify-start items-center space-x-4">
                            <Camera {..} style="width:72px; height:72px" class="sm:w-24 text-main-color"/>
                            <p class="text-gray-700">{t!(i18n,home.company.icon6_label) }</p>
                        </div>
                      <div class="grid grid-cols-subgrid gap-4 md:col-span-3 text-center">
                        <div class="md:col-start-2 text-gray-700 py-3">{t!(i18n,home.company.and_more) }</div>
                      </div>
                    </div>
                    <p class="italic text-4xl font-display text-main-color text-center p-4">{t!(i18n,home.company.quote, <b> = <span class="text-7xl text-accent-color"/ > )}</p>
                    // Removing this avoid the linking error.
                    <div class="flex sm:flex-row flex-col justify-center items-center space-x-4 p-4">
                        <div class="overflow-hidden flex flex-row rounded-lg bg-main-color shadow">
                            <div class="px-4 py-5 sm:p-6 text-white text-2xl sm:text-4xl text-center">
                              <p>{t!(i18n,home.company.search_cta) }</p>
                              <Magnifier {..} style="width:64px; height:64px" class="text-accent-color"/>
                            </div>
                        </div>
                        <div class="overflow-hidden flex flex-row rounded-lg bg-main-color shadow">
                          <div class="px-4 py-5 sm:p-6 text-white text-2xl sm:text-4xl text-center">
                              <p>{t!(i18n,home.company.submit_cta) }</p>
                              <PaperPlane {..} style="width:64px; height:64px" class="text-accent-color"/>
                          </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}