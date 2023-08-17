use leptos::*;

/// 404 - Not Found
#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {cx,
        <main class="w-full h-[100vh] flex items-center justify-center">
            <section class="text-red-500 text-center translate-y-4">
                <h1 class="text-4xl font-bold">{"404"}</h1>
                <p class="text-xl">{ "Page not found" }</p>
            </section>
        </main>
    }
}
