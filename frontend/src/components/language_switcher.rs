use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{modules::lang::Lang, modules::store::SharedData};

fn toggle_language(language: Lang) -> Lang {
    match language {
        Lang::CHN => Lang::JP,
        Lang::JP => Lang::CHN,
    }
}

#[function_component]
pub fn LanguageSwitcher() -> Html {
    let (store, dispatch) = use_store::<SharedData>();
    let lang = match store.language {
        Lang::CHN => "中",
        Lang::JP => "日",
    };

    let onclick = {
        let dispatch = dispatch.clone();
        let current_lang = store.language.clone();
        Callback::from(move |_: MouseEvent| {
            SharedData::set_language(toggle_language(current_lang), &dispatch)
        })
    };

    html! {
        <div
            class="bg-zinc-800 w-10 h-10 text-white rounded-lg hover:bg-gray-500 duration-300 cursor-pointer"
            onclick={onclick}
        >
            <div class="w-full h-full hover:-translate-y-2 duration-300 flex items-center justify-center">
                {lang}
            </div>
        </div>
    }
}
