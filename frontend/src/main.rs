use frontend::layout::footer::Footer;
use frontend::layout::navbar::Navbar;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Navbar />
            <main class="h-96 w-11/12 bg-sky-500 mt-10 flex mx-auto">
            </main>
            <Footer/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
