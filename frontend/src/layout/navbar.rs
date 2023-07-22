use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::components::language_switcher::LanguageSwitcher;
use crate::{modules::lang::Lang, modules::store::SharedData};

#[function_component(Navbar)]
pub fn navbar() -> Html {
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
    html! (
        <header class="fixed top-0 w-full h-16 z-20 backdrop-blur-md flex justify-center">
            <nav class="w-10/12 h-full flex justify-start items-center">
                <div class="flex-start flex-auto flex items-center gap-3 font-black tracking-widest text-3xl">
                    <h1>{ "TWTSN" }</h1>
                </div>
                <div class="inline-flex items-center justify-end space-x-6 w-auto">
                    {for routes.map(
                        |route| html! {
                            <div class="cursor-pointer hover:border-b-2 hover:border-b-zinc-700 duration-150"> {route} </div>
                        }
                    )}
                    <LanguageSwitcher />
                </div>
            </nav>
        </header>
    )
}
