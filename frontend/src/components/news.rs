use std::time::Duration;

use common::ResponseHeadline;

use leptos::*;
use leptos_icons::*;
use reqwest::{Response, StatusCode};

fn auto_scroll(
    cx: Scope,
    headline_num: ReadSignal<usize>,
    pos: ReadSignal<usize>,
    set_pos: WriteSignal<usize>,
) {
    create_effect(cx, move |_| {
        set_interval(
            move || {
                if pos() / 100 == headline_num() - 1 {
                    set_pos.update(|pos| *pos = 0);
                    return;
                }
                set_pos.update(|pos| *pos = *pos + 100)
            },
            Duration::from_secs(5),
        );
    });
}

type HeadlineInfo = (String, String);

#[component]
pub fn News(cx: Scope) -> impl IntoView {
    let headline_infos = create_resource(cx, || (), |_| get_headlines());
    let (headline_num, set_headline_num) = create_signal(cx, 0_usize);
    let (pos, set_pos) = create_signal(cx, 0_usize);

    auto_scroll(cx, headline_num, pos, set_pos);
    view! {cx,
        <div class="w-11/12 lg:h-12 md:h-10 h-8 mx-auto rounded-md relative overflow-hidden">
            <div class="h-full w-full">
                <div class="lg:w-16 md:w-14 w-12 h-full bg-zinc-900 p-2 relative inline-block">
                    <div class="bg-gray-400 lg:w-8 lg:h-8 w-6 h-6 animate-ping rounded-full absolute left-1/4 translate-x-1 lg:translate-x-0 -translate-y-1 md:translate-y-0"></div>
                    <div class="w-full h-full text-white lg:text-xl md:text-lg flex items-center justify-center">
                        <Icon icon=Icon::from(BiIcon::BiBellRingSolid) />
                    </div>
                </div>
                <div
                    class="w-10/12 h-full absolute lg:left-24 md:left-20 left-16 duration-300 transition-all"
                    style:top={move || format!("-{}%", pos())}
                >
                    <Suspense
                        fallback=move || view! { cx, <span class="loading loading-lg"></span> }
                    >
                        {move || headline_infos.read(cx).iter().flatten().map(|headlines| {
                            set_headline_num.update(|headline_num| *headline_num = headlines.len());
                            headlines.iter().map(|(headline, url)| {
                                view! {cx,
                                    <div class="h-full w-full relative">
                                        <h3 class="font-semibold lg:text-xl md:text-lg text-md absolute top-1/2 -translate-y-1/2 hover:text-gray-400 duration-150 truncate overflow-ellipsis">
                                            <a href=url>{headline}...</a>
                                        </h3>
                                    </div>
                                }
                            }).collect_view(cx)
                        }).collect_view(cx)}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

async fn get_headlines() -> Result<Vec<HeadlineInfo>, Vec<()>> {
    let url = "http://localhost:8080/api/headlines";
    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("Content-Type", "application/json")
        .send()
        .await;

    match res {
        Ok(res) => retrieve_headlines(res).await,
        Err(err) => {
            log!("{}", err);
            Err(vec![])
        }
    }
}

async fn retrieve_headlines(res: Response) -> Result<Vec<HeadlineInfo>, Vec<()>> {
    match res.status() {
        StatusCode::OK => {
            let body: ResponseHeadline = res.json().await.unwrap();
            Ok(body.headlines)
        }
        _ => Ok(vec![]),
    }
}
