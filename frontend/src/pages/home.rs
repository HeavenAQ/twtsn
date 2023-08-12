use crate::components::news::News;
use yew::prelude::*;

#[function_component]
fn Hero() -> Html {
    html! {
        <header class="max-h-[85vh] h-auto w-full max-w-screen-2xl mx-auto relative overflow-hidden">
            <img class="w-full h-auto" src="images/hero.jpg" alt="hero image" />
        </header>

    }
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <Hero />
            <section class="mt-5 mb-40">
                <News />
            </section>
        </>
    }
}
