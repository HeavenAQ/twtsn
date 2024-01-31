use crate::components::go_back::GoBack;
use crate::modules::utils::{is_cur_route, render_jp_or_chn};

use crate::api::cases::CaseType;
use leptos::Scope;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn CaseTabs(cx: Scope) -> impl IntoView {
    let tab = CaseType::new();
    view! {cx,
        <div class="tabs mx-auto mt-10 inline-flex justify-center">
            {move || {
                tab.into_iter().map(|case_type| {
                    view! {cx,
                        <a
                            class="tab tab-bordered duration-300 transition-all ease-in-out"
                            class=("tab-active", move || is_cur_route(cx, &format!("/cases/{}", case_type.to_string())))
                            href=case_type.to_string()
                        >
                            {move || case_type.to_tag_name(cx)}
                        </a>
                    }
                }).collect_view(cx)
            }}

            <a
                class="tab tab-bordered duration-300 transition-all ease-in-out"
                class=("tab-active", move || is_cur_route(cx, "/cases/others"))
                href=CaseType::Other.to_string()
            >
                {move || CaseType::Other.to_tag_name(cx)}
            </a>
        </div>

    }
}

#[component]
pub fn CasePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Title text="TWTSN Case History"/>
        <Meta name="description" content="TWTSN Case History"/>

        <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto min-h-screen">
            <div class="max-w-2xl mx-auto text-center mb-10 lg:mb-14 flex-col flex items-center">
                <h2 class="text-2xl font-bold md:text-4xl md:leading-tight text-zinc-900 lg:mt-0 mt-10">
                    {move || render_jp_or_chn(cx, "ケース", "案例")}
                </h2>
                <CaseTabs />
            </div>
            <Outlet />
        </div>
        <GoBack content="Back to Home".to_string() href="/".to_owned()/>
    }
}
