use leptos::{ev::MouseEvent, *};
use std::time::Duration;

use crate::app::HomePageState;

struct CarouselSlide<'a> {
    image: &'a str,
    title: &'a str,
    description: &'a str,
}

#[component]
fn CarouselCard<'a>(cx: Scope, slide: &'a CarouselSlide<'static>) -> impl IntoView {
    view! { cx,
        <div class="w-1/4 h-full relative overflow-hidden rounded-xl inline-block">
            <img class="h-full w-full object-cover" src=slide.image/>
            <div class="absolute text-center bottom-14 text-white w-1/2 left-1/2 -translate-x-1/2">
                <h3 class="font-semibold lg:text-xl md:text-lg text-md">{slide.title}</h3>
                <p class="lg:block hidden">{slide.description}</p>
            </div>
        </div>
    }
}

#[component]
fn CarouselDot(
    cx: Scope,
    id_prefix: &'static str,
    id: usize,
    set_active_idx: WriteSignal<usize>,
) -> impl IntoView {
    view! { cx,
        <input
            type="radio"
            name="dot"
            id=format!("{}_{}", id_prefix, id)
            class="hidden absolute"
            on:click=move |_: MouseEvent| set_active_idx.update(|idx| *idx = id)
            checked=move || id == 0
        />
    }
}

#[component]
fn CarouselLabel(
    cx: Scope,
    id_prefix: &'static str,
    id: usize,
    active_idx: ReadSignal<usize>,
) -> impl IntoView {
    view! { cx,
        <label
            for=format!("{}_{}", id_prefix, id)
            class="w-11 h-1 border-2 border-solid border-gray-400 hover:border-white m-2 rounded-full cursor-pointer transition-all duration-400"
            style:border=move || {
                format!("2px solid {}", if id == active_idx() { "white" } else { "gray" })
            }
        ></label>
    }
}

fn carousel_range(end: usize) -> impl Iterator<Item = usize> {
    (0..end).into_iter()
}

#[component]
fn CarouselIdxWrapper(
    cx: Scope,
    active_idx: ReadSignal<usize>,
    set_active_idx: WriteSignal<usize>,
    slides_len: usize,
    children: Children,
) -> impl IntoView {
    view! { cx,
        {carousel_range(slides_len)
            .map(|id| view! { cx, <CarouselDot id_prefix="exhibit" id=id set_active_idx/> })
            .collect_view(cx)}

        {children(cx)}

        <div class="absolute bottom-5 flex left-1/2 -translate-x-1/2">
            {carousel_range(slides_len)
                .map(|id| view! { cx, <CarouselLabel id_prefix="exhibit" id=id active_idx/> })
                .collect_view(cx)}
        </div>
    }
}

fn auto_slide(cx: Scope, slides_len: usize) {
    create_effect(cx, move |_| {
        let home_state = use_context::<ReadSignal<HomePageState>>(cx).unwrap();
        set_interval(
            move || {
                if (home_state().cur_carousel)() == slides_len - 1 {
                    home_state().set_cur_carousel.update(|idx| *idx = 0);
                    return;
                }
                home_state().set_cur_carousel.update(|idx| *idx = *idx + 1);
            },
            Duration::from_secs(17),
        )
    });
}

#[component]
pub fn ExhibitionCarousel(cx: Scope) -> impl IntoView {
    let (active_idx, set_active_idx) = create_signal(cx, 0_usize);
    let carousel_slides = vec![
        CarouselSlide {
            image: "/assets/images/hero.jpg",
            title: "TWTSN",
            description: "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.",
        },
        CarouselSlide {
            image: "/assets/images/hero.jpg",
            title: "TWTSN",
            description: "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.",
        },
        CarouselSlide {
            image: "/assets/images/hero.jpg",
            title: "TWTSN",
            description: "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.",
        },
        CarouselSlide {
            image: "/assets/images/hero.jpg",
            title: "TWTSN",
            description: "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.",
        },
    ];

    auto_slide(cx, carousel_slides.len());
    view! { cx,
        <div class="w-full h-full overflow-hidden relative shadow-lg rounded-xl">
            <div
                class="h-full overflow-hidden duration-700"
                style:margin-left=move || format!("-{}%", active_idx() * 100)
                style:width=format!("{}%", carousel_slides.len() * 100)
            >
                <CarouselIdxWrapper slides_len=carousel_slides.len() active_idx set_active_idx>
                    {carousel_slides
                        .iter()
                        .map(|slide| {
                            view! { cx, <CarouselCard slide/> }
                        })
                        .collect_view(cx)}
                </CarouselIdxWrapper>
            </div>
        </div>
    }
}
