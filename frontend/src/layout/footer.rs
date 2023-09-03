use crate::components::button::Button;
use crate::components::msg_box::MsgBox;
use crate::modules::utils::render_jp_or_chn;
use leptos::*;
use leptos_icons::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    let icons = vec![
        (
            "www.facebook.com",
            view! {cx, <Icon icon=Icon::from(SiIcon::SiFacebook) /> },
        ),
        (
            "072-284-9617",
            view! {cx, <Icon icon=Icon::from(FaIcon::FaSquarePhoneSolid) />},
        ),
        (
            "大阪府堺市西区山田4-2056-1",
            view! {cx, <Icon icon=Icon::from(FaIcon::FaLocationDotSolid) />},
        ),
        (
            "info@twtsn.co.jp",
            view! {cx, <Icon icon=Icon::from(FaIcon::FaEnvelopeSolid) />},
        ),
    ];

    view! { cx,
      <footer class="flex flex-col gap-6 items-center justify-center bg-zinc-900 text-white text-center mx-auto p-6 z-30 w-full h-72 left-1/2 overflow-hidden">
        <h1 class="text-3xl font-bold">{"TWTSN"}</h1>
        <div class="inline-flex w-1/2 justify-center items-center space-x-16">
          {move || render_jp_or_chn(cx,
              view! { cx,
                <Button content="お問い合わせ" href="/contact#contact_form"/>
                <Button content="サブスク" href="/#news-letter"/>
              },
                            view! { cx,
                <Button content="聯絡我們" href="/contact#contact_form"/>
                <Button content="訂閱電子報" href="/#news-letter"/>
              })
          }

        </div>
        <div class="inline-flex mx-auto w-full justify-center items-center space-x-10">
          {icons
              .into_iter()
              .map(|icon| {
                  view! { cx,
                    <div class="flex flex-col items-center justify-start">
                      <div class="relative w-auto h-auto group/info cursor-pointer">
                        <MsgBox info=icon.0/>
                        <div class="mt-16 text-2xl">{icon.1}</div>
                      </div>
                    </div>
                  }
              })
              .collect_view(cx)}
        </div>
      </footer>
    }
}
