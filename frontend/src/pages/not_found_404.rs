use yew::prelude::*;

#[function_component]
pub fn PageNotFound() -> Html {
    html! {
        <main class="w-full h-[67vh] flex items-center justify-center">
            <section class="text-red-500 text-center translate-y-4">
                <h1 class="text-4xl font-bold">{"404"}</h1>
                <p class="text-xl">{ "Page not found" }</p>
            </section>
        </main>
    }
}
