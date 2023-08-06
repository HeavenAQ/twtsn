use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlElement};

use super::{lang::Lang, store::SharedData};

pub fn get_child_element(target: &EventTarget, selector: &str) -> Option<Element> {
    let tmp_target = target.clone();
    match tmp_target.dyn_into::<HtmlElement>() {
        Ok(element) => match element.query_selector(selector) {
            Ok(element) => element,
            Err(_) => return None,
        },
        Err(_) => return None,
    }
}
