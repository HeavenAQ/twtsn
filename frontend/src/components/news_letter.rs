use crate::modules::utils::render_jp_or_chn;
use leptos::*;

#[component]
pub fn NewsLetter(cx: Scope) -> impl IntoView {
    view! { cx,
          <div class="pt-20 pb-8 px-4 mx-auto max-w-screen-xl lg:py-16 lg:px-6">
              <div class="mx-auto max-w-screen-md sm:text-center">
                  <h2 class="mb-10 text-3xl tracking-tight font-bold text-white">
                    {move || render_jp_or_chn(cx, "ニュースレター", "電子報")}
                  </h2>
                  <form action="#">
                      <div class="items-center mx-auto mb-3 space-y-4 max-w-screen-sm sm:flex sm:space-y-0">
                          <div class="relative w-full">
                              <label for="email" class="hidden mb-2 text-sm font-medium text-gray-900">Email address</label>
                              <div class="flex absolute inset-y-0 left-0 items-center pl-3 pointer-events-none">
                                  <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path><path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path></svg>
                              </div>
                              <input class="block p-3 pl-10 w-full text-sm rounded-lg border-[.5px] sm:rounded-none sm:rounded-l-lg focus:ring-zinc-500 focus:border-primary-500 bg-slate-200 border-gray-600 placeholder-gray-400 focus:ring-primary-500 focus:border-primary-500" placeholder="example@mail.com" type="email" id="email" required="" />
                          </div>
                          <div>
                                  <button type="submit" class="py-3 px-5 w-full text-sm font-medium text-center text-white rounded-lg border cursor-pointer bg-zinc-700 border-primary-600 sm:rounded-none sm:rounded-r-lg hover:bg-white hover:text-zinc-900 focus:ring-4 focus:ring-primary-300 bg-primary-600 hover:bg-primary-700 focus:ring-primary-800 duration-200 transition-all break-keep">
                                    {move || render_jp_or_chn(cx, "購読する", "訂閱")}
                                </button>
                          </div>
                      </div>
                  </form>
              </div>
          </div>
    }
}
