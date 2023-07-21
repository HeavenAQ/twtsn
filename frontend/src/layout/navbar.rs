use yew::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! (
        <header class="fixed top-0 w-full h-16 z-20 backdrop-blur-md flex justify-center">
            <nav class="w-10/12 h-full flex justify-start items-center">
                <div class="flex-start flex-auto flex items-center gap-3 font-black tracking-widest p-2 text-3xl">
                    <h1>{ "TWTSN" }</h1>
                </div>
                <div class="inline-flex justify-evenly items-center w-1/2">
                    <div>{"首頁"}</div>
                    <div>{"近期展覽"}</div>
                    <div>{"案例分享"}</div>
                    <div>{"服務"}</div>
                    <div>{"聯絡我們"}</div>
                </div>
            </nav>
        </header>
    )
}
