use leptos::*;

use crate::{app::Store, components::language_switcher::HorizontalLanguageSwitcher};

pub const DROPDOWN_INACTIVE: &'static str = "before:md:-translate-y-[10px] after:md:translate-y-[10px] before:-translate-y-[8px] after:translate-y-[8px]";
pub const DROPDOWN_ACTIVE: &'static str =
    "before:translate-x-[50px] after:translate-x-[50px] before:translate-y-0 after:translate-y-0 before:-rotate-45 after:rotate-45 translate-x-[-50px]";

pub fn switch_dropdown(set_store: WriteSignal<Store>, on: bool) {
    set_store.update(|store| store.dropdown_active = on);
    let element = match document().query_selector("#menu-hamburger") {
        Ok(Some(element)) => element,
        _ => return,
    };
    let cur_classes = element.class_name();
    let (old, new) = match on {
        true => (DROPDOWN_INACTIVE, DROPDOWN_ACTIVE),
        false => (DROPDOWN_ACTIVE, DROPDOWN_INACTIVE),
    };
    element.set_class_name(cur_classes.replace(old, new).as_str());
}

#[component]
pub fn DropDownMenu<I>(
    cx: Scope,
    routes: ReadSignal<[&'static str; 4]>,
    paths: I,
    set_store: WriteSignal<Store>,
) -> impl IntoView
where
    I: Iterator<Item = String>,
{
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    let is_dropdown_active = move || !store.with(|store| store.dropdown_active);
    let paths: Vec<String> = paths.collect();
    view! {cx,
        <div class="relative block lg:hidden">
            <MenuButton set_store=set_store/>
            <div
                class="absolute md:w-[300px] w-[250px] h-auto top-14 left-0 -translate-x-[85%] bg-zinc-800 rounded-xl md:p-4 p-2 overflow-hidden duration-150 shadow-xl transition-all"
                class=("opacity-100", is_dropdown_active)
                class=("hidden", is_dropdown_active)
            >

            {move || {
                routes()
                    .iter()
                    .zip(paths.clone())
                    .map(|(route_name, route_path)| view! {cx,
                        <DropDownItem
                        route_name=route_name.to_string()
                            route_path=route_path.to_string()
                            set_store=set_store
                        />
                    }).collect_view(cx)
            }}
            <HorizontalLanguageSwitcher set_store/>
            </div>
        </div>
    }
}

#[component]
fn MenuButton(cx: Scope, set_store: WriteSignal<Store>) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    view! {cx,
        <div
            class="relative flex flex-col items-center justify-center w-10 h-10 cursor-pointer transition-all duration-150 ease-in-out overflow-hidden"
            on:click=move |_| switch_dropdown(set_store, store.with(|store| !store.dropdown_active))
        >
            <div
                id="menu-hamburger"
                class="md:w-8 md:h-[3px] w-6 h-[2px] bg-zinc-800 rounded-lg transition-all duration-150 ease-in-out before:content-[''] before:absolute before:md:w-8 before:md:h-[3px] before:w-6 before:h-[2px] before:bg-zinc-800 before:rounded-lg before:transition-all before:duration-150 before:ease-in-out after:content-[''] after:absolute after:md:w-8 after:md:h-[3px] after:w-6 after:h-[2px] after:bg-zinc-800 after:rounded-lg after:transition-all after:duration-150 after:ease-in-out"
                class=DROPDOWN_INACTIVE
            >
            </div>
        </div>
    }
}

#[component]
fn DropDownItem(
    cx: Scope,
    route_name: String,
    route_path: String,
    set_store: WriteSignal<Store>,
) -> impl IntoView {
    view! {cx,
        <a
            href=route_path
            on:click=move |_| {
                set_store.update(|store| store.dropdown_active = false);
                switch_dropdown(set_store, false);
            }
        >
            <div
                class="h-12 w-full flex items-center rounded-xl p-2 transition-all duration-150 hover:bg-gray-400 text-slate-100 md:text-lg sm:text-sm"
            >
                {route_name}
            </div>
        </a>
    }
}
