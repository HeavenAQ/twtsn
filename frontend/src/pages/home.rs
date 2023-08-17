use crate::components::news::News;
use leptos::*;

#[component]
fn Hero(cx: Scope) -> impl IntoView {
    view! {cx,
        <header class="max-h-[85vh] h-auto w-full max-w-screen-2xl mx-auto relative overflow-hidden">
            <img class="w-full h-auto" src="assets/images/hero.jpg" alt="hero image" />
        </header>

    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! {cx,
        <>
            <Hero />
            <section class="mt-5 mb-40">
                <News />
            </section>
        </>
    }
}
