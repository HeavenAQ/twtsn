use crate::{
    app::{Lang, Store},
    pages::services::SERVICES,
};
use leptos::*;
use leptos_icons::*;

#[component]
fn ContactInfo(cx: Scope) -> impl IntoView {
    let icons = vec![
        (
            "072-284-9617",
            view! {cx, <Icon icon=Icon::from(FaIcon::FaSquarePhoneSolid) />},
        ),
        (
            "〒593-8316大阪府堺市西区山田4-2056-1",
            view! {cx, <Icon icon=Icon::from(FaIcon::FaLocationDotSolid) />},
        ),
        (
            "info@twtsn.co.jp",
            view! {cx, <Icon icon=Icon::from(FaIcon::FaEnvelopeSolid) />},
        ),
    ];

    view! {cx,
        <div class="flex flex-col gap-9">
            {icons.iter().enumerate().map(|(i, icon)| {
                view!{cx,
                    <div key={i} class="inline-grid grid-cols-6 gap-4 w-full h-auto">
                    <div class="bg-white text-zinc-800 h-10 w-10 rounded-full flex items-center justify-center col-span-1">{icon.1.clone()}</div>
                        <div class="leading-10 col-span-5">
                        <p class="text-md">{icon.0}</p>
                        </div>
                    </div>
                }
            }).collect_view(cx)}
        </div>
    }
}

#[component]
fn EmailForm(cx: Scope) -> impl IntoView {
    let store = use_context::<ReadSignal<Store>>(cx).unwrap();
    let input_fields = move || match store.with(|store| store.language) {
        Lang::JP => [
            "名前",
            "会社名",
            "電話番号",
            "メール",
            "信件內容",
            "件名",
            "メッセージ",
        ],
        Lang::CHN => [
            "姓名",
            "公司名",
            "電話號碼",
            "電子郵件",
            "信件內容",
            "主旨",
            "內文",
        ],
    };

    view! {cx,
        <>
            <form class="w-full">
                <div class="flex flex-wrap -mx-3 mb-6">
                    <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-first-name">
                            {move || input_fields()[0]}
                        </label>
                        <input class="appearance-none block w-full border rounded py-3 px-4 mb-3 leading-tight " id="grid-first-name" type="text" placeholder="Jane Chou" />
                    </div>
                    <div class="w-full md:w-1/2 px-3">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-last-name">
                            {move || input_fields()[1]}
                        </label>
                        <input class="appearance-none block w-full border border-gray-200 rounded py-3 px-4 leading-tight" id="grid-last-name" type="text" placeholder="Company"/>
                    </div>
                </div>
                <div class="flex flex-wrap -mx-3 mb-6">
                    <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="phone">
                            {move || input_fields()[2]}
                        </label>
                        <input class="appearance-none block w-full border rounded py-3 px-4 mb-3 leading-tight " id="phone" type="tel" name="phone" placeholder="xxx-xxx-xxx" />
                    </div>
                    <div class="w-full md:w-1/2 px-3">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-last-name">
                            {move || input_fields()[3]}
                        </label>
                        <input class="appearance-none block w-full border border-gray-200 rounded py-3 px-4 leading-tight" id="grid-last-name" type="email" placeholder="example@example.com"/>
                    </div>
                </div>
                <div class="flex flex-wrap items-end -mx-3 mb-2">
                    <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-state">
                            {move || input_fields()[4]}
                        </label>
                        <div class="relative">
                            <select
                                class="block appearance-none w-full border border-gray-200 py-3 px-4 pr-8 rounded leading-tight"
                                id="grid-state"
                            >
                                {move || SERVICES.get_service(store().language).iter().map(|service| {
                                    view!{cx,
                                        <option>{service}</option>
                                    }
                                }).collect_view(cx)}
                            </select>
                            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2">
                            </div>
                        </div>
                    </div>
                    <div class="w-full md:w-2/3 px-3">
                        <input class="appearance-none block w-full border border-gray-200 rounded py-3 px-4 leading-tight" id="grid-last-name" type="email" placeholder=move || input_fields()[5]/>
                    </div>
                </div>

                <div class="flex flex-wrap -mx-3 mb-6">
                    <div class="w-full px-3 mb-6 md:mb-0 mt-5">
                        <textarea class="appearance-none block w-full border rounded py-3 px-4 mb-3 leading-tight" id="email-content" type="textarea" name="email-content" placeholder=move || input_fields()[6] rows="10"/>
                    </div>
                </div>
            </form>
        </>
    }
}

#[component]
pub fn ContactForm(cx: Scope) -> impl IntoView {
    view! {cx,
    <div class="max-h-[700px] lg:h-[75vh] w-11/12 lg:grid lg:grid-rows-2 lg:grid-cols-2 mx-auto lg:rounded-xl lg:my-24 my-14 lg:gap-2 lg:shadow-lg bg-transparent">
        <div class="lg:row-span-2 lg:col-span-1 row-span-3 bg-zinc-800 flex p-12 justify-center rounded-xl mb-12 lg:mb-0">
            <EmailForm />
        </div>
        <div class="lg:row-span-1 lg:text-xl bg-gray-600 text-white p-12 md:text-lg text-md w-full rounded-xl row-span-1 lg:mb-0 mb-12 h-80 lg:h-auto">
            <ContactInfo />
        </div>
        <div class="lg:row-span-1 lg:rounded-none w-full rounded-xl row-span-2 lg:h-auto h-80">
            <iframe src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3287.691194299063!2d135.4673067765358!3d34.51071229338189!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x6000daeaeaaf7545%3A0x5f6e7cafccf701fc!2s4-ch%C5%8Dme-2056-1%20Yamada%2C%20Nishi%20Ward%2C%20Sakai%2C%20Osaka%20593-8316%2C%20Japan!5e0!3m2!1sen!2stw!4v1690584174173!5m2!1sen!2stw" class="w-full h-full lg:max-h-full max-h-96 rounded-xl" loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
        </div>
    </div>
    }
}
