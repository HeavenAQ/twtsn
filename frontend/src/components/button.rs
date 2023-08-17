use leptos::*;

#[component]
pub fn Button(cx: Scope, href: &'static str, content: &'static str) -> impl IntoView {
    view! { cx,
        <a href=href>
            <div class="flex justify-center items-center bg-white w-auto min-w-[128px] h-7 text-zinc-900 rounded-full p-5 hover:bg-gray-400 duration-200 font-semibold cursor-pointer hover:text-white">
                {content}
            </div>
        </a>
    }
}
