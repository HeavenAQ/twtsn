use frontend::layout::footer::Footer;
use frontend::layout::navbar::Navbar;
use frontend::modules::lang::Lang;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let lang = use_state(|| Lang::CHN);

    html! {
        <div>
            <Navbar />
            <main class="h-96 w-10/12 bg-sky-500 mt-10 flex mx-auto">
            </main>
            <Footer lang={*lang.clone()}/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
