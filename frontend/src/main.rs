use frontend::components::bulletin::Bulletin;
use frontend::layout::footer::Footer;
use frontend::layout::navbar::Navbar;
use frontend::modules::router::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Navbar />
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            <Footer/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
