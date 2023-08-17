use crate::layout::footer::Footer;
use crate::layout::navbar::Navbar;
use crate::pages::contact::Contact;
use crate::pages::home::HomePage;
use crate::pages::not_found::NotFound;
use crate::pages::services::ServicePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use serde::{Deserialize, Serialize};
#[derive(Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Lang {
    JP,
    #[default]
    CHN,
}

#[derive(Clone)]
pub struct Store {
    pub language: Lang,
    pub dropdown_active: bool,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            language: Lang::default(),
            dropdown_active: false,
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (store, set_store) = create_signal(cx, Store::default());
    provide_context(cx, store);

    view! { cx,
      <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
      <Title text="TWTSN"/>

      <Router>
        <Navbar set_store/>
        <main >
          <Routes>
            <Route path="" view=HomePage/>
            <Route path="/contact" view=Contact/>
            <Route path="/services" view=ServicePage/>
            <Route path="/*any" view=NotFound/>
          </Routes>
          <Footer/>
        </main>
      </Router>
    }
}
