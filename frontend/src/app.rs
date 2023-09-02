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

#[derive(Clone)]
pub struct HomePageState {
    pub headline_idx: ReadSignal<usize>,
    pub headline_num: ReadSignal<usize>,
    pub set_headline_num: WriteSignal<usize>,
    pub set_headline_idx: WriteSignal<usize>,
    pub cur_carousel: ReadSignal<usize>,
    pub set_cur_carousel: WriteSignal<usize>,
}

impl HomePageState {
    fn new(cx: Scope) -> Self {
        let (headline_num, set_headline_num) = create_signal(cx, 0_usize);
        let (headline_idx, set_headline_idx) = create_signal(cx, 0_usize);
        let (cur_carousel, set_cur_carousel) = create_signal(cx, 0_usize);
        Self {
            headline_idx,
            headline_num,
            set_headline_num,
            set_headline_idx,
            cur_carousel,
            set_cur_carousel,
        }
    }
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (store, set_store) = create_signal(cx, Store::default());
    let (home_state, set_home_state) = create_signal(cx, HomePageState::new(cx));
    provide_context(cx, store);
    provide_context(cx, home_state);
    provide_context(cx, set_home_state);

    view! { cx,
      <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
      <Title text="TWTSN"/>

      <Router>
        <Navbar set_store/>
          <Routes>
            <Route path="" view=HomePage/>
            <Route path="/contact" view=Contact/>
            <Route path="/services" view=ServicePage/>
            <Route path="/*any" view=NotFound/>
          </Routes>
          <Footer/>
      </Router>
    }
}
