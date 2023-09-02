use crate::app::HomePageState;
use crate::app::{Lang, Store};
use crate::components::button::Button;
use crate::components::carousel::ExhibitionCarousel;
use crate::components::news::News;
use crate::components::news_letter::NewsLetter;
use crate::pages::services::{ServiceInfo, SERVICES};
use leptos::*;

#[component]
fn Hero(cx: Scope) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    view! { cx,
        <div
            class="hero min-h-screen bg-base-200"
            style="background-image: url(https://daisyui.com/images/stock/photo-1507358522600-9f71e620c44e.jpg);"
        >
            <div class="hero-content flex-col lg:flex-row-reverse">
                <div class="w-1/2"></div>
                <div class="w-1/2 text-white">
                    <h1 class="text-5xl font-bold uppercase">
                        TWTSN
                    </h1>
                    <p class="py-6">
                        Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi.
                    </p>

                    <div class="inline-flex w-1/2">
                        {move || match store.with(|store| store.language) {
                            Lang::JP => {
                                view! { cx,
                                    <Button
                                        content="お問い合わせ"
                                        href="/contact#contact_form"
                                    />
                                }
                            }
                            Lang::CHN => {

                                view! { cx,
                                    <Button content="聯絡我們" href="/contact#contact_form"/>
                                }
                            }
                        }}

                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ServiceCard(cx: Scope, service: ServiceInfo) -> impl IntoView {
    view! {cx,
        <div class="grid flex-grow card bg-base-300 rounded-box place-items-center hover:opacity-60 duration-300 cursor-pointer overflow-hidden relative">
            <a href=format!("/services#{}", service.title) class="w-full h-full">
                <h3 class="absolute font-bold text-xl left-4 top-4 z-10 text-white">{service.title}</h3>
                <img src=&service.images[0] alt="service image" class="w-full h-full object-cover"/>
            </a>
        </div>
    }
}

#[component]
fn ServiceRowCards(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <div class="flex flex-col w-full lg:flex-row gap-5 justify-evenly">
            {children(cx)}
        </div>
    }
}

#[component]
fn ServiceSection(cx: Scope) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    view! { cx,
        <div class="w-full flex flex-col gap-5">
            {move || {
                let mut first_two_services = SERVICES.get_service_info_list(store().language);
                let last_two_services = first_two_services.drain(2..).collect::<Vec<_>>();

                view! { cx,
                    <ServiceRowCards>
                        {first_two_services.iter().map(|service| {
                            view! { cx, <ServiceCard service={service.clone()}/> }
                        }).collect_view(cx)}
                    </ServiceRowCards>
                    <ServiceRowCards>
                        {last_two_services.iter().map(|service| {
                            view! { cx, <ServiceCard service={service.clone()}/> }
                        }).collect_view(cx)}
                    </ServiceRowCards>
                }
            }}
        </div>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    view! { cx,
        <div class="w-full">
            <Hero/>
            <section class="h-[10vh] flex items-center justify-center">
                <News/>
            </section>
            <section class="w-11/12 h-[40vh] justify-center items-center mx-auto lg:mb-40 md:mb-32 mb-20">
                <ExhibitionCarousel/>
            </section>
            <section class="lg:mb-40 md:mb-32 mb-20 w-10/12 mx-auto text-center">
                <h1 class="lg:text-4xl text-3xl font-bold uppercase lg:mb-20 md:mb-14 mb-10 tracking-widest">{move || match store.with(|store| store.language) {
                    Lang::JP => "サービス",
                    Lang::CHN => "服務",
                }}</h1>
                <ServiceSection/>
            </section>
            <section class="bg-zinc-800 h-80">
                <NewsLetter />
            </section>
        </div>
    }
}
