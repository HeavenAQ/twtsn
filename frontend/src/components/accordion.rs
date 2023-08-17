use leptos::*;

const LABEL_UNCHECKED: &'static str = "before:content-['+']";
const LABEL_CHECKED: &'static str = "before:content-['-']";
const CONTENT_UNCHECKED: &'static str = "max-h-0";
const CONTENT_CHECKED: &'static str = "max-h-[400px]";

fn toggle_accordion(id: usize) {
    let selector = format!("body > main > ul > li:nth-child({})", id);
    let accordion = match document().query_selector(selector.as_str()) {
        Ok(Some(accordion)) => accordion,
        _ => return,
    };
    let label = match accordion.query_selector("label") {
        Ok(Some(label)) => label,
        _ => return,
    };
    let content = match accordion.query_selector("div") {
        Ok(Some(label)) => label,
        _ => return,
    };
    let label_classname = toggle_class(label.class_name(), LABEL_CHECKED, LABEL_UNCHECKED);
    let content_classname = toggle_class(content.class_name(), CONTENT_CHECKED, CONTENT_UNCHECKED);
    label.set_class_name(label_classname.as_str());
    content.set_class_name(content_classname.as_str());
}

fn toggle_class(old_classname: String, checked: &str, unchecked: &str) -> String {
    let new_classname = if old_classname.contains(checked) {
        old_classname.replace(checked, unchecked)
    } else {
        old_classname.replace(unchecked, checked)
    };
    new_classname
}

#[component]
pub fn Accordion(
    cx: Scope,
    question: &'static str,
    answer: &'static str,
    nth: usize,
) -> impl IntoView {
    view! {cx,
        <li class="list-none w-full md:p-8 rounded-2xl shadow-lg p-5">
            <label
                class="flex items-center p-3 cursor-pointer text-lg font-medium before:mr-3 before:text-2xl before:font-semibold before:content-['+']"
                on:click=move |_| toggle_accordion(nth)
            >
                {question}
            </label>
            <div
                class="px-[10px] leading-[26px] overflow-hidden transition-[max-height] duration-300 max-h-0"
            >
                {answer}
            </div>
        </li>
    }
}
