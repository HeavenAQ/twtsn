use leptos::*;

#[component]
pub fn GoBack(cx: Scope, content: String, href: String) -> impl IntoView {
    view! { cx,
        <footer class="mt-auto w-full max-w-[85rem] px-4 sm:px-6 lg:px-8 mx-auto">
            <div class="text-center">
            <a class="text-base w-full sm:w-auto inline-flex justify-center items-center gap-2 rounded-md border border-transparent font-semibold text-zinc-500 hover:text-zinc-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition-all  px-4 ring-offset-slate-900 mb-24" href=href>
            <svg class="w-2.5 h-2.5" width="16" height="16" viewBox="0 0 16 16" fill="none">
              <path d="M11.2792 1.64001L5.63273 7.28646C5.43747 7.48172 5.43747 7.79831 5.63273 7.99357L11.2792 13.64" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
            {content}
          </a>
            </div>
        </footer>
    }
}
