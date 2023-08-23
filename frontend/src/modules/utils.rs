use leptos::*;
use leptos_router::use_location;

pub fn is_cur_route(cx: Scope, route: &str) -> bool {
    let cur_location = use_location(cx);
    cur_location.pathname.get() == route
}
