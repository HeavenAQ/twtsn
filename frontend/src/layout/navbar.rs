use web_sys::window;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::components::dropdown_menu::DropDownMenu;
use crate::components::language_switcher::LanguageSwitcher;
use crate::{modules::lang::Lang, modules::store::SharedData};

fn get_cur_route() -> String {
    window()
        .unwrap()
        .location()
        .pathname()
        .unwrap_or("/".to_string())
}

fn is_cur_route(route: &str) -> bool {
    get_cur_route() == route
}

#[function_component]
pub fn Navbar() -> Html {
    let (store, _) = use_store::<SharedData>();
    let routes = match store.language {
        Lang::CHN => ["首頁", "近期展覽", "案例分享", "服務", "聯絡我們"],
        Lang::JP => [
            "ホーム",
            "最近の展覧会",
            "ケースシェア",
            "サービス",
            "お問い合わせ",
        ],
    };
    let routes = routes
        .iter()
        .zip(["/", "/exhibitions", "/cases", "/services", "/contact"]);

    html! (
        <header class="fixed top-0 w-full h-16 z-20 backdrop-blur-md flex justify-center">
            <nav class="w-10/12 h-full flex justify-start items-center">
                <div class="flex-start flex-auto flex items-center gap-3 font-black tracking-widest text-3xl">
                    <h1>{ "TWTSN" }</h1>
                </div>
                <div class="items-center justify-end space-x-6 w-auto hidden lg:inline-flex">
                    {for routes.clone().map(
                        |route| html! {
                            <NavItem item_name={route.0} item_route={route.1} />
                        }
                    )}
                    <LanguageSwitcher />
                </div>
                <DropDownMenu
                    routes={routes.map(|(route, path)| (route.to_string(), path.to_string())).collect::<Vec<(String, String)>>()}
                />
            </nav>
        </header>
    )
}

#[derive(Properties, Clone, PartialEq)]
struct Props {
    item_name: &'static str,
    item_route: &'static str,
}

#[function_component]
fn NavItem(props: &Props) -> Html {
    html! {
        <div
            class={format!("cursor-pointer hover:border-b-2 hover:border-b-zinc-700 duration-150 {}", if is_cur_route(props.item_route) { "border-b-2 border-b-zinc-700" } else {""})}
        >
                <a href={props.item_route}>{props.item_name}</a>
        </div>
    }
}
