use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub info: &'static str,
}

#[function_component]
pub fn MsgBox(props: &Props) -> Html {
    html! {
        <span
            class="opacity-0 group-hover/info:opacity-100 duration-300 absolute left-1/2 top-0 -translate-x-1/2 bg-gray-500 whitespace-nowrap py-[10px] px-[15px] rounded-lg before:content-[''] before:absolute before:left-1/2 before:top-full before:-translate-x-1/2 before:border-[15px] before:border-t-gray-500 before:border-x-[#0000] before:border-b-[#0000]"
        >
            {props.info}
        </span>
    }
}
