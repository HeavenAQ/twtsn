use yew::prelude::*;
use yewdux::prelude::{use_store, Dispatch};

use crate::{modules::lang::Lang, modules::store::SharedData};

fn toggle_language(language: Lang) -> Lang {
    match language {
        Lang::CHN => Lang::JP,
        Lang::JP => Lang::CHN,
    }
}

fn get_current_lang(store: &SharedData) -> String {
    match store.language {
        Lang::CHN => "中".to_owned(),
        Lang::JP => "日".to_owned(),
    }
}

fn switch_lang_onclick(
    store: &SharedData,
    dispatch: &Dispatch<SharedData>,
) -> Callback<MouseEvent> {
    let dispatch = dispatch.clone();
    let current_lang = store.language.clone();
    Callback::from(move |_: MouseEvent| {
        SharedData::set_language(toggle_language(current_lang), &dispatch)
    })
}

#[function_component]
pub fn LanguageSwitcher() -> Html {
    let (store, dispatch) = use_store::<SharedData>();
    let cur_lang = get_current_lang(&store);

    html! {
        <div
            class="bg-zinc-800 w-10 h-10 text-white rounded-lg hover:bg-gray-500 duration-300 cursor-pointer"
            onclick={switch_lang_onclick(&store, &dispatch)}
        >
            <div class="w-full h-full hover:-translate-y-2 duration-300 flex items-center justify-center">
                {cur_lang}
            </div>
        </div>
    }
}

#[function_component]
pub fn HorizontalLanguageSwitcher() -> Html {
    let (store, dispatch) = use_store::<SharedData>();
    let cur_lang = get_current_lang(&store);

    html! {
        <div class="inline-flex text-slate-100 text-lg h-12 w-full items-center p-2">
            <div
                class="inline-flex bg-zinc-700 w-20 h-10 rounded-lg transition-all duration-300 cursor-pointer"
                onclick={switch_lang_onclick(&store, &dispatch)}
            >
                {for ["中", "日"].iter().map(|lang| html! {
                    <div class={format!("w-full h-full flex items-center rounded-lg justify-center  {}", if lang.to_string() == cur_lang { "bg-white text-black" } else {""})}>
                        {&lang}
                    </div>
                })}
            </div>
        </div>
    }
}
