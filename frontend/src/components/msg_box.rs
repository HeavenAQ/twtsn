use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub info: &'static str,
}

#[function_component]
pub fn MsgBox(props: &Props) -> Html {
    html! {
        <div class="relative msg_box">
            <div class="h-15 text-white bg-gray-500 p-3 pt-4 rounded-lg flex items-center text-center">
                {props.info}
            </div>
            <div class="w-3 h-10 bg-gray-500 absolute top-10"/>
            <div class="w-10 h-7 bg-zinc-900 absolute top-[52px] rounded-tl-full -left-3"/>
        </div>
    }
}
