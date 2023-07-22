use crate::pages::home::Home;
use crate::pages::not_found_404::PageNotFound;
use gloo::utils::window;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/exhibitions")]
    Exhibitions,
    #[at("/exhibitions/:id")]
    Exhibition { id: String },
    #[at("/cases")]
    Cases,
    #[at("/cases/:id")]
    Case { id: String },
    #[at("/services")]
    Services,
    #[at("/contact")]
    Contact,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Exhibitions => todo!(),
        Route::Exhibition { id } => todo!(),
        Route::Cases => todo!(),
        Route::Case { id } => todo!(),
        Route::Services => todo!(),
        Route::Contact => todo!(),
        Route::NotFound => html! { <PageNotFound /> },
    }
}
