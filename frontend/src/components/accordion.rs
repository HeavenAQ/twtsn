use gloo::{console::log, utils::document_element};
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub question: String,
    pub answer: String,
    pub nth: usize,
}

const LABEL_UNCHECKED: &'static str = "before:content-['+']";
const LABEL_CHECKED: &'static str = "before:content-['-']";
const CONTENT_UNCHECKED: &'static str = "max-h-0";
const CONTENT_CHECKED: &'static str = "max-h-[400px]";

fn toggle_accordion(id: usize) {
    let accordion_selector = format!("body > div > ul > li:nth-child({})", id);
    let accordion = match get_element_by_selector(&document_element(), accordion_selector.as_str())
    {
        Some(element) => element,
        None => return,
    };
    let mut label = match get_element_by_selector(&accordion, "label") {
        Some(element) => element,
        None => return,
    };
    let mut content = match get_element_by_selector(&accordion, "div") {
        Some(element) => element,
        None => return,
    };
    log!("label: {:?}", &label);
    log!("content: {:?}", &content);
    toggle_class(&mut label, LABEL_CHECKED, LABEL_UNCHECKED);
    toggle_class(&mut content, CONTENT_CHECKED, CONTENT_UNCHECKED);
}

fn get_element_by_selector(root: &Element, selector: &str) -> Option<Element> {
    match root.query_selector(selector) {
        Ok(element) => match element {
            Some(element) => Some(element),
            None => return None,
        },
        Err(_) => return None,
    }
}

fn toggle_class(e: &mut Element, checked: &str, unchecked: &str) {
    let classname = e.class_name();
    let new_classname = if classname.contains(checked) {
        classname.replace(checked, unchecked)
    } else {
        classname.replace(unchecked, checked)
    };
    e.set_class_name(new_classname.as_str());
}

#[function_component]
pub fn Accordion(props: &Props) -> Html {
    let id = props.nth;
    html! {
        <li class="list-none w-full md:p-8 rounded-2xl shadow-lg p-5">
            <label
                class={format!("flex items-center p-3 cursor-pointer text-lg font-medium before:mr-3 before:text-2xl before:font-semibold {}", LABEL_UNCHECKED)}
                onclick={Callback::from(move |_| toggle_accordion(id))}
            >
                {&props.question}
            </label>
            <div class={format!("px-[10px] leading-[26px] overflow-hidden {} transition-[max-height] duration-300", CONTENT_UNCHECKED)}>
                {&props.answer}
            </div>
        </li>
    }
}
