use crate::modules::lang::Lang;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub content: String,
    pub lang: Lang,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    html! {
        <div class="flex justify-center items-center bg-white w-32 h-7 text-zinc-900 rounded-full p-5 hover:bg-gray-400 duration-200 font-semibold cursor-pointer hover:text-white">
            { &props.content }
        </div>
    }
}
