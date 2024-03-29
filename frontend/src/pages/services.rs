use leptos::*;

use crate::app::{Lang, Store};

pub struct Services {
    jp: [&'static str; 5],
    ch: [&'static str; 5],
}

#[derive(Clone, PartialEq)]
pub struct ServiceInfo {
    pub title: String,
    pub description: String,
    pub images: Vec<String>,
}

impl Services {
    pub const fn new() -> Self {
        Self {
            ch: ["展覽規劃", "場地租借", "演講翻譯", "樂器租借", "其他"],
            jp: [
                "展覧会の計画",
                "会場の貸し出し",
                "講演の翻訳",
                "楽器の貸し出し",
                "その他",
            ],
        }
    }

    pub fn get_service(&self, lang: Lang) -> Vec<String> {
        let target = match lang {
            Lang::CHN => self.ch,
            Lang::JP => self.jp,
        };
        target.iter().map(|s| s.to_string()).collect()
    }

    pub fn get_service_info_list(&self, lang: Lang) -> Vec<ServiceInfo> {
        let service_titles = self.get_service(lang);
        let service_descriptions = match lang {
            Lang::CHN => vec![
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
            ],
            Lang::JP => vec![
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat. Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat. Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat. Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat. Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
            ],
        };
        let service_images = vec![
            vec!["/assets/images/default.jpg".to_string()],
            vec!["/assets/images/default.jpg".to_string()],
            vec!["/assets/images/default.jpg".to_string()],
            vec!["/assets/images/default.jpg".to_string()],
        ];
        service_titles
            .iter()
            .take(4)
            .enumerate()
            .map(|(idx, title)| ServiceInfo {
                title: title.to_owned(),
                description: service_descriptions[idx].to_owned(),
                images: service_images[idx].to_owned(),
            })
            .collect()
    }
}

pub const SERVICES: Services = Services::new();

#[component]
fn ServiceCard(cx: Scope, id: usize, service: ServiceInfo) -> impl IntoView {
    // get window size
    let is_img_left = move || id % 2 == 0;
    let (description, _) = create_signal(
        cx,
        view! {cx,<div class="lg:grid hidden"><p >{service.description.clone()}</p></div> },
    );
    let (empty, _) = create_signal(cx, view! {cx, <div class="hidden"></div>});

    view! {cx,
        <div id=id>
            <div class="inline-grid lg:grid-cols-2 gap-10 grid-cols-1">
                <Show when=is_img_left fallback=move |_| empty() >
                    {description()}
                </Show>
                <div class="text-center w-auto relative">
                    <h1 class="text-3xl font-bold absolute left-1/2 -translate-x-1/2 -top-14 break-keep">{service.title}</h1>
                    <img class="w-full h-full rounded-lg" src="assets/images/default.jpg" alt="service image" />
                </div>
                <Show when=move || !is_img_left() fallback=move |_| empty.clone() >
                    {description()}
                </Show>
                <div class="lg:hidden grid"><p >{service.description}</p></div>
            </div>
        </div>
    }
}

#[component]
pub fn ServicePage(cx: Scope) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();

    view! {cx,
        <div class="relative mb-24 w-4/5 mx-auto snap-y snap-mandatory scroll-smooth">
            { move || {
                let services = SERVICES.get_service_info_list(store().language);
                services.iter().take(4).enumerate().map(|(i, service)| {
                    view! {cx,
                        <div id=&service.title class="h-[100vh] flex items-center justify-center snap-start">
                            <ServiceCard id=i service={service.clone()}/>
                        </div>
                    }
                }).collect_view(cx)
            }}
        </div>
    }
}
