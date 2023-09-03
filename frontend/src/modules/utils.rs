use leptos::*;
use leptos_router::use_location;

use crate::app::{Lang, Store};

pub fn is_cur_route(cx: Scope, route: &str) -> bool {
    let cur_location = use_location(cx);
    cur_location.pathname.get() == route
}

pub fn render_jp_or_chn<T>(cx: Scope, jp: T, chn: T) -> T {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    match store.with(|s| s.language) {
        Lang::CHN => chn,
        Lang::JP => jp,
    }
}
