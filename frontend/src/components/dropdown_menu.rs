use gloo::utils::document;
use yew::prelude::*;
use yewdux::prelude::{use_store, Dispatch};

use crate::components::language_switcher::HorizontalLanguageSwitcher;
use crate::modules::store::SharedData;

pub const DROPDOWN_INACTIVE: &'static str = "before:md:-translate-y-[10px] after:md:translate-y-[10px] before:-translate-y-[8px] after:translate-y-[8px]";
pub const DROPDOWN_ACTIVE: &'static str =
    "before:translate-x-[50px] after:translate-x-[50px] before:translate-y-0 after:translate-y-0 before:-rotate-45 after:rotate-45 translate-x-[-50px]";

pub fn switch_dropdown(dispatch: &Dispatch<SharedData>, on: bool) -> Callback<MouseEvent> {
    let dispatch = dispatch.clone();
    Callback::from(move |_: MouseEvent| {
        SharedData::set_dropdown_active(on, &dispatch);
        let element = match document().query_selector(".menu-hamburger") {
            Ok(Some(element)) => element,
            _ => return,
        };
        let cur_classes = element.class_name();
        let (old, new) = match on {
            true => (DROPDOWN_INACTIVE, DROPDOWN_ACTIVE),
            false => (DROPDOWN_ACTIVE, DROPDOWN_INACTIVE),
        };
        element.set_class_name(cur_classes.replace(old, new).as_str());
    })
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub routes: Vec<(String, String)>,
}

#[function_component]
pub fn DropDownMenu(props: &Props) -> Html {
    let (store, _) = use_store::<SharedData>();
    html! {
        <div class="relative block lg:hidden">
            <MenuButton />
            <div class={format!("absolute md:w-[300px] w-[250px] h-auto top-14 left-0 -translate-x-[85%] bg-zinc-800 rounded-xl md:p-4 p-2 overflow-hidden duration-150 {}", if store.dropdown_active { "opacity-100" } else { "hidden opacity-0" })}>
                {for props.routes.iter().map(
                    |route| html! {
                        <DropDownItem item_name={route.0.to_owned()} item_route={route.1.to_owned()} />
                    }
                )}
                <HorizontalLanguageSwitcher/>
            </div>
        </div>
    }
}

#[function_component]
fn MenuButton() -> Html {
    let (store, dispatch) = use_store::<SharedData>();
    let onclick = switch_dropdown(&dispatch, !store.dropdown_active);

    html! {
        <div
            class="relative flex flex-col items-center justify-center w-10 h-10 cursor-pointer transition-all duration-150 ease-in-out overflow-hidden"
            onclick={onclick}
        >
            <div
                class={format!("md:w-8 md:h-[3px] w-6 h-[2px] bg-zinc-800 rounded-lg transition-all duration-150 ease-in-out before:content-[''] before:absolute before:md:w-8 before:md:h-[3px] before:w-6 before:h-[2px] before:bg-zinc-800 before:rounded-lg before:transition-all before:duration-150 before:ease-in-out after:content-[''] after:absolute after:md:w-8 after:md:h-[3px] after:w-6 after:h-[2px] after:bg-zinc-800 after:rounded-lg after:transition-all after:duration-150 after:ease-in-out menu-hamburger {}", DROPDOWN_INACTIVE)}
            >
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
struct DropDownItemProps {
    item_route: String,
    item_name: String,
}

#[function_component]
fn DropDownItem(props: &DropDownItemProps) -> Html {
    let (_, dispatch) = use_store::<SharedData>();
    let onclick = {
        let dispatch = dispatch.clone();
        Callback::from(move |_: MouseEvent| {
            SharedData::set_dropdown_active(false, &dispatch);
        })
    };
    html! {
        <a href={props.item_route.to_owned()} onclick={onclick}>
            <div class="h-12 w-full flex items-center rounded-xl p-2 transition-all duration-150 hover:bg-gray-400 text-slate-100 md:text-lg sm:text-sm">
                {&props.item_name}
            </div>
        </a>
    }
}
