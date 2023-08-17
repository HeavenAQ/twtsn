use crate::app::{Lang, Store};
use crate::components::dropdown_menu::{switch_dropdown, DropDownMenu};
use crate::components::language_switcher::LanguageSwitcher;
use leptos::*;
use leptos_router::use_location;

fn is_cur_route(cx: Scope, route: &str) -> bool {
    let cur_location = use_location(cx);
    cur_location.pathname.get() == route
}

#[component]
pub fn Navbar(cx: Scope, set_store: WriteSignal<Store>) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    let paths = ["/", "/exhibitions", "/cases", "/services", "/contact"];
    let chn_routes = ["首頁", "近期展覽", "案例分享", "服務", "聯絡我們"];
    let jp_routes = [
        "ホーム",
        "最近の展覧会",
        "ケースシェア",
        "サービス",
        "お問い合わせ",
    ];
    let (routes, set_routes) = create_signal(cx, chn_routes);

    create_effect(cx, move |prev_path| {
        let cur_location = use_location(cx).pathname.get();
        if Some(cur_location.clone()) != prev_path {
            switch_dropdown(set_store, false);
        }
        cur_location
    });

    view! { cx,
      <header class="fixed top-0 w-full md:h-16 h-12 z-20 backdrop-blur-md flex justify-center">
        <nav
          class="w-11/12 h-full flex justify-start items-center"
          class=("text-slate-100", move || is_cur_route(cx, "/"))
        >
          <div class="flex-start flex-auto flex items-center gap-3 font-semibold tracking-widest lg:text-3xl md:text-2xl text-xl">
            <h1>
              <a href="/">
                TWTSN
              </a>
            </h1>
          </div>
          <div class="items-center justify-end space-x-6 w-auto hidden lg:inline-flex">
            {move || {
                set_routes.update(|routes| *routes = match store.with(|s| s.language) {
                    Lang::CHN => chn_routes,
                    Lang::JP => jp_routes,
                });
                routes().iter().zip(paths).map(|(name, path)| {
                    view! { cx, <NavItem route_name=name route_path=path/> }
                }).collect_view(cx)
            }}
            <LanguageSwitcher set_store/>
          </div>
          <DropDownMenu
            routes=routes
            paths=paths.to_vec().iter().map(|path| path.to_string())
            set_store=set_store
          />
        </nav>
      </header>
    }
}

#[component]
fn NavItem(cx: Scope, route_name: &'static str, route_path: &'static str) -> impl IntoView {
    view! { cx,
      <div
        class="cursor-pointer hover:border-b-2 hover:border-b-zinc-700 duration-150"
        class=("border-b-2", move || is_cur_route(cx, route_path))
        class=("border-b-zinc-700", move || is_cur_route(cx, route_path))
      >
        <a href=route_path>{route_name}</a>
      </div>
    }
}
