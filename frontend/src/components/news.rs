use leptos::*;

#[component]
pub fn News(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="w-11/12 h-12 mx-auto rounded-md overflow-hidden">
            <div class="h-full w-full inline-flex justify-start items-center">
                <div class="w-16 h-full bg-zinc-900 p-2 relative">
                    <img class="w-full h-full" src="assets/images/bell.svg" alt="news icon" />
                </div>
            </div>
        </div>
    }
}
