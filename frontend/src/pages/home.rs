use crate::components::bulletin::Bulletin;
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <main class="h-[45vh] w-full max-w-screen-2xl bg-sky-500 mt-[62px] mb-11 flex mx-auto relative">
            </main>
            <section>
                <Bulletin />
            </section>
        </>
    }
}
