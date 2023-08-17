use leptos::*;

use crate::app::{Lang, Store};

fn toggle_language(language: Lang) -> Lang {
    match language {
        Lang::CHN => Lang::JP,
        Lang::JP => Lang::CHN,
    }
}

fn get_current_lang(language: Lang) -> String {
    match language {
        Lang::CHN => "中".to_owned(),
        Lang::JP => "日".to_owned(),
    }
}

#[component]
pub fn LanguageSwitcher(cx: Scope, set_store: WriteSignal<Store>) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    view! {cx,
        <div
            class="bg-zinc-800 w-10 h-10 text-white rounded-lg hover:bg-gray-500 duration-300 cursor-pointer"
            on:click=move |_| set_store.update(|s| s.language = toggle_language(s.language))
        >
            <div class="w-full h-full hover:-translate-y-2 duration-300 flex items-center justify-center">
                {move || get_current_lang(store.with(|s| s.language))}
            </div>
        </div>
    }
}

#[component]
pub fn HorizontalLanguageSwitcher(cx: Scope, set_store: WriteSignal<Store>) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    view! { cx,
        <div class="inline-flex text-slate-100 md:text-lg text-sm h-12 w-full items-center p-2">
            <div
                class="inline-flex bg-zinc-700 md:w-20 md:h-10 w-16 h-8 rounded-lg transition-all duration-300 cursor-pointer"
                on:click=move |_| set_store.update(|s| s.language = toggle_language(s.language))
            >

            {["中", "日"].iter().map(|lang| {
                    let is_cur_lang = move || store.with(|s| get_current_lang(s.language) == lang.to_owned());
                     view! {cx,
                        <div
                            class="w-full h-full flex items-center rounded-lg justify-center"
                            class=("bg-white", is_cur_lang)
                            class=("text-black", is_cur_lang)
                        >
                            {lang.to_string()}
                        </div>
                    }
                }).collect_view(cx)
            }
            </div>
        </div>
    }
}
