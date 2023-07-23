use frontend::components::dropdown_menu::switch_dropdown;
use frontend::layout::footer::Footer;
use frontend::layout::navbar::Navbar;
use frontend::modules::router::*;
use frontend::modules::store::SharedData;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;

#[function_component]
fn App() -> Html {
    let (_, dispatch) = use_store::<SharedData>();
    html! {
        <>
            <Navbar />
            <div onclick={switch_dropdown(&dispatch, false)}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
                <Footer/>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
