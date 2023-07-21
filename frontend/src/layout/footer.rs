use std::rc::Rc;

use crate::components::button::Button;
use crate::modules::lang::Lang;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub lang: Lang,
}

fn icon_html(icon_id: IconId) -> Html {
    html! {
        <Icon icon_id={icon_id} class="hover:text-gray-400 duration-100 cursor-pointer" />
    }
}

#[function_component]
pub fn Footer(props: &Props) -> Html {
    let icons = Rc::from([
        icon_html(IconId::BootstrapFacebook),
        icon_html(IconId::FontAwesomeSolidSquarePhone),
        icon_html(IconId::FontAwesomeSolidLocationDot),
        icon_html(IconId::FontAwesomeSolidEnvelope),
    ]);

    html! {
        <>
            <footer
                class="flex flex-col gap-6 items-center justify-center bg-zinc-900 text-white text-center mx-auto p-6 fixed bottom-0 z-30 w-full h-56 left-1/2 -translate-x-1/2"
            >
                <h1 class="text-3xl font-bold">{ "TWTSN" }</h1>
                <div class="inline-flex w-1/2 justify-center items-center space-x-16 mb-7">
                    <Button content={"聯絡我們"} lang={props.lang.clone()}/>
                    <Button content={"訂閱電子報"} lang={props.lang.clone()}/>
                </div>
                <div class="inline-flex mx-auto w-full justify-center items-center space-x-10">
                    {for icons.iter().enumerate().map(|(i, icon)| {
                        html!{
                            <div
                                key={i}
                                class="flex flex-col items-center justify-start"
                            >
                                <div class="relative w-auto h-auto">
                                    {icon.clone()}
                                </div>
                            </div>
                        }
                    })}
                </div>
            </footer>
        </>
    }
}
