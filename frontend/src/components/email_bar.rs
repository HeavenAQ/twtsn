use leptos::*;

use crate::app::Lang;
use crate::app::Store;

#[component]
pub fn EmailBar(cx: Scope) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    view! { cx,
        <div class="w-[90%] h-16 border-none shadow-xl rounded-full mx-auto">
            <form class="w-full h-full rounded-full">
              <input class="w-full h-full rounded-full p-7" type="email" placeholder="subscribe@me.now" />
              <button class="" type="button">
                    <span
                        class="w-52 h-16 m-12 text-slate-100 rounded-full text-2xl font-bold flex justify-center items-center border-[#242424] transition-all duration-150 shadow-[4px_4px_10px_#000,_-4px_-4px_10px_#353535] hover:shadow-[4px_4px_10px_#000,_-4px_-4px_10px_#353535_inset_-2px_-2px_4px_#353535_inset_2px_2px_4px_#000] active:translate-y-1 active:shadow-[-4px_-4px_10px_#000,_4px_4px_10px_#353535]"
                    >
                        {move || match store.with(|store| store.language) {
                            Lang::CHN => "訂閱",
                            Lang::JP => "購読",
                        }}
                    </span>
               </button>
            </form>
        </div>
    }
}
