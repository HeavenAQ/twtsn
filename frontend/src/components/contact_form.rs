use std::rc::Rc;

use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::use_store;

use crate::{
    modules::{lang::Lang, store::SharedData},
    pages::services::SERVICES,
};

fn icon_html(icon_id: IconId) -> Html {
    html! {
        <Icon icon_id={icon_id} />
    }
}

#[function_component]
fn ContactInfo() -> Html {
    let icons = Rc::from([
        (
            "072-284-9617",
            icon_html(IconId::FontAwesomeSolidSquarePhone),
        ),
        (
            "info@twtsn.co.jp",
            icon_html(IconId::FontAwesomeSolidEnvelope),
        ),
        (
            "〒593-8316大阪府堺市西区山田4-2056-1",
            icon_html(IconId::FontAwesomeSolidLocationDot),
        ),
    ]);
    html! {
    <>
        <div class="flex flex-col gap-9">
            {for icons.iter().enumerate().map(|(i, icon)| {
            html!{
                <div key={i} class="inline-grid grid-cols-6 gap-4 w-full h-auto">
                <div class="bg-white text-zinc-800 h-10 w-10 rounded-full flex items-center justify-center col-span-1">{icon.1.clone()}</div>
                    <div class="leading-10 col-span-5">
                    <p class="text-md">{&icon.0}</p>
                    </div>
                </div>
            }
            })}
        </div>
    </>
    }
}

#[function_component]
fn EmailForm() -> Html {
    let (store, _) = use_store::<SharedData>();
    let input_fields = match store.language {
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

    html! {
        <>
            <form class="w-full max-w-lg">
                <div class="flex flex-wrap -mx-3 mb-6">
                    <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-first-name">
                            {input_fields[0]}
                        </label>
                        <input class="appearance-none block w-full border rounded py-3 px-4 mb-3 leading-tight " id="grid-first-name" type="text" placeholder="Jane Chou" />
                    </div>
                    <div class="w-full md:w-1/2 px-3">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-last-name">
                            {input_fields[1]}
                        </label>
                        <input class="appearance-none block w-full border border-gray-200 rounded py-3 px-4 leading-tight" id="grid-last-name" type="text" placeholder="Company"/>
                    </div>
                </div>
                <div class="flex flex-wrap -mx-3 mb-6">
                    <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="phone">
                            {input_fields[2]}
                        </label>
                        <input class="appearance-none block w-full border rounded py-3 px-4 mb-3 leading-tight " id="phone" type="tel" name="phone" placeholder="xxx-xxx-xxx" />
                    </div>
                    <div class="w-full md:w-1/2 px-3">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-last-name">
                            {input_fields[3]}
                        </label>
                        <input class="appearance-none block w-full border border-gray-200 rounded py-3 px-4 leading-tight" id="grid-last-name" type="email" placeholder="example@example.com"/>
                    </div>
                </div>
                <div class="flex flex-wrap items-end -mx-3 mb-2">
                    <div class="w-full md:w-1/3 px-3 mb-6 md:mb-0">
                        <label class="block tracking-wide text-slate-100 text-sm font-bold mb-2" for="grid-state">
                            {input_fields[4]}
                        </label>
                        <div class="relative">
                            <select
                                class="block appearance-none w-full border border-gray-200 py-3 px-4 pr-8 rounded leading-tight"
                                id="grid-state"
                            >
                                {for SERVICES.get_service(store.language).iter().map(|service| {
                                html!{
                                    <option>{service}</option>
                                }
                                })}
                            </select>
                            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2">
                                <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
                            </div>
                        </div>
                    </div>
                    <div class="w-full md:w-2/3 px-3">
                        <input class="appearance-none block w-full border border-gray-200 rounded py-3 px-4 leading-tight" id="grid-last-name" type="email" placeholder={input_fields[5]}/>
                    </div>
                </div>

                <div class="flex flex-wrap -mx-3 mb-6">
                    <div class="w-full px-3 mb-6 md:mb-0 mt-5">
                        <textarea class="appearance-none block w-full border rounded py-3 px-4 mb-3 leading-tight" id="email-content" type="textarea" name="email-content" placeholder={input_fields[6]} rows="10"/>
                    </div>
                </div>
            </form>
        </>
    }
}

#[function_component]
pub fn ContactForm() -> Html {
    html! {
    <div class="lg:h-[75vh] w-4/5 lg:grid lg:grid-rows-2 lg:grid-cols-2 mx-auto lg:rounded-xl lg:my-24 my-14 lg:gap-2 lg:shadow-lg bg-transparent">
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
