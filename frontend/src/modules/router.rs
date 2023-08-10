use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::not_found_404::PageNotFound;
use crate::pages::services::ServicePage;
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
        Route::Exhibitions => html! {<div>{"Exhibitions"}</div>},
        Route::Exhibition { id } => todo!(),
        Route::Cases => html! { <div>{"Exhibitions"}</div> },
        Route::Case { id } => todo!(),
        Route::Services => html! { <ServicePage /> },
        Route::Contact => html! { <Contact /> },
        Route::NotFound => html! { <PageNotFound /> },
    }
}
