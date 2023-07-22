use yew::prelude::*;

#[function_component]
pub fn PageNotFound() -> Html {
    html! {
        <main class="pt-[56px]">
            <section class="mt-20 text-red-500 text-center">
                <h1 class="text-4xl font-bold">{"404"}</h1>
                <p class="text-xl">{ "Page not found" }</p>
            </section>
        </main>
    }
}
