use crate::modules::{lang::Lang, store::SharedData};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use yew::prelude::*;
use yewdux::prelude::use_store;

pub struct Services {
    jp: [&'static str; 5],
    ch: [&'static str; 5],
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

    fn get_service_info_list(&self, lang: Lang) -> Vec<ServiceInfo> {
        let service_titles = self.get_service(lang);
        let service_descriptions = match lang {
            Lang::CHN => vec![
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
                "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
            ],
            Lang::JP => vec![
                "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.",
                "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.",
                "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.",
                "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.",
                "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.",
            ],
        };
        let service_images = vec![
            vec!["images/default.jpg".to_string()],
            vec!["images/default.jpg".to_string()],
            vec!["images/default.jpg".to_string()],
            vec!["images/default.jpg".to_string()],
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

#[derive(Clone, PartialEq)]
struct ServiceInfo {
    title: String,
    description: String,
    images: Vec<String>,
}

#[derive(Properties, Clone, PartialEq)]
struct ServiceCardProps {
    id: String,
    service: ServiceInfo,
}

#[function_component]
fn ServiceCard(props: &ServiceCardProps) -> Html {
    // get window size
    let window = window().unwrap();
    let window_size = use_state(|| window.inner_width().unwrap().as_f64().unwrap());
    let is_img_left = props.id.parse::<usize>().unwrap() % 2 == 0;
    let description = html! { <div><p >{&props.service.description}</p></div> };
    let empty = html! { <></> };

    let resize_callback = {
        let window = window.clone();
        let window_size = window_size.clone();
        Closure::wrap(Box::new(move || {
            window_size.set(window.inner_width().unwrap().as_f64().unwrap());
            if let 1000..=1024 = *window_size as i64 {
                window.location().reload().unwrap();
            }
        }) as Box<dyn FnMut()>)
    };

    window.set_onresize(Some(resize_callback.as_ref().unchecked_ref()));
    resize_callback.forget();
    html! {
        <div id={props.id.clone()}>
            <div class="inline-grid lg:grid-cols-2 gap-10 grid-cols-1">
                {if is_img_left || *window_size <= 1024_f64 { empty.clone() } else { description.clone() }}
                <div class="text-center w-auto relative">
                    <h1 class="text-3xl font-bold absolute left-1/2 -translate-x-1/2 -top-14">{&props.service.title}</h1>
                    <img class="w-full h-full rounded-lg" src="images/default.jpg" alt="service image" />
                </div>
                {if is_img_left || *window_size <= 1024_f64 { description } else { empty }}
            </div>
        </div>
    }
}

#[function_component]
pub fn ServicePage() -> Html {
    let (store, _) = use_store::<SharedData>();
    let services = SERVICES.get_service_info_list(store.language);

    html! {
        <div class="flex flex-col space-y-28 mt-40 mb-24 w-4/5 mx-auto">
            {for services.iter().take(4).enumerate().map(|(i, service)| {
                html! {
                    <ServiceCard id={i.to_string()} service={service.clone()}/>
                }
            })}
        </div>
    }
}
