use std::rc::Rc;

use crate::modules::lang::Lang;
use crate::{components::button::Button, components::msg_box::MsgBox, modules::store::SharedData};
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

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
pub fn Footer() -> Html {
    let (store, _) = use_store::<SharedData>();
    let icons = Rc::from([
        ("www.facebook.com", icon_html(IconId::BootstrapFacebook)),
        (
            "072-284-9617",
            icon_html(IconId::FontAwesomeSolidSquarePhone),
        ),
        (
            "〒593-8316大阪府堺市西区山田4-2056-1",
            icon_html(IconId::FontAwesomeSolidLocationDot),
        ),
        (
            "info@twtsn.co.jp",
            icon_html(IconId::FontAwesomeSolidEnvelope),
        ),
    ]);

    let (contact, subscribe) = match store.language {
        Lang::JP => ("お問い合わせ", "サブスク"),
        Lang::CHN => ("聯絡我們", "訂閱電子報"),
    };

    html! {
        <>
            <footer
                class="flex flex-col gap-6 items-center justify-center bg-zinc-900 text-white text-center mx-auto p-6 z-30 w-full h-72 left-1/2"
            >
                <h1 class="text-3xl font-bold">{ "TWTSN" }</h1>
                <div class="inline-flex w-1/2 justify-center items-center space-x-16">
                    <Button content={contact} href="/contact#contact_form"/>
                    <Button content={subscribe} href="some"/>
                </div>
                <div class="inline-flex mx-auto w-full justify-center items-center space-x-10">
                    {for icons.iter().enumerate().map(|(i, icon)| {
                        html!{
                            <div
                                key={i}
                                class="flex flex-col items-center justify-start"
                            >
                                <div class="relative w-auto h-auto group/info cursor-pointer">
                                    <MsgBox info={icon.0}/>
                                    <div class="mt-16">{icon.1.clone()}</div>
                                </div>
                            </div>
                        }
                    })}
                </div>
            </footer>
        </>
    }
}
