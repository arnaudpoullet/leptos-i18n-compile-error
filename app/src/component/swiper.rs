use leptos::prelude::*;
use crate::component::svg::{ChevronLeft,ChevronRight};

#[derive(Clone)]
pub struct CardData {
    pub idx: u8,
    pub title: &'static str,
    pub description: &'static str,
}

#[component]
pub fn Swiper(cards: Vec<CardData>) -> impl IntoView {
    // Card data struct

    // Create reactive state for the current index
    let (current_index, set_current_index) = signal(0u8);
    let total_slides = cards.len() as u8;

    // Helper function to go to the next slide
    let next_slide = move |_| {
        set_current_index.update(|index| *index = (*index + 1) % total_slides);
    };

    // Helper function to go to the previous slide
    let previous_slide = move |_| {
        set_current_index.update(|index| {
            *index = if *index == 0 {
                total_slides - 1
            } else {
                *index - 1
            }
        });
    };

    view! {
        <div class="relative w-full overflow-hidden">
            <div class="flex transition-transform duration-300 ease-in-out"
                 style=move || format!("transform: translateX(-{}%)", current_index.get() * 100)>
                <For each=move || cards.clone() key=|card| card.idx
                    children=move |card: CardData| {
                        view! {
                            <div class="flex-none w-full p-4">
                                <div class="max-w-md sm:max-w-3xl mx-auto bg-white shadow-lg rounded-lg p-6">
                                    <h2 class="text-xl font-semibold text-gray-800">{ card.title }</h2>
                                    <p class="mt-2 text-gray-600 h-72">{ card.description }</p>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
            // Navigation buttons
            <div on:click=previous_slide class="absolute flex items-center justify-center top-1/2 left-0 h-full transform -translate-y-1/2 hover:cursor-pointer text-center text-gray-400 hover:text-gray-700 rounded py-2 pl-2 sm:w-24 w-[45%]"><ChevronLeft {..} style="width:48px; height:48px"/></div>
            <div on:click=next_slide class="absolute flex items-center justify-center top-1/2 right-0 h-full transform -translate-y-1/2 hover:cursor-pointer text-center text-gray-400 hover:text-gray-700 rounded py-2 pl-2 sm:w-24 w-[45%]"><ChevronRight {..} style="width:48px; height:48px"/></div>
            // Dots at the bottom for navigation
            <div class="flex justify-center mt-4 space-x-2">
                <For each=move || {(0..total_slides).collect::<Vec<_>>()} key=|&i| i
                    children=move |i: u8| {
                        view! {
                            <div class=move || if current_index.get() == i { "w-3 h-3 rounded-full bg-main-color" } else { "w-3 h-3 rounded-full border-2 border-main-color" }>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}