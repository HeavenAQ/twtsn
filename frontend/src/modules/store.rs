use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use yewdux::store::Store;

use crate::modules::lang::Lang;

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local")]
pub struct SharedData {
    pub language: Lang,
    pub loading: bool,
}

impl SharedData {
    pub fn set_loading(loading: bool, dispatch: &Dispatch<Self>) {
        dispatch.reduce_mut(move |store| {
            store.loading = loading;
        })
    }

    pub fn set_language(lang: Lang, dispatch: &Dispatch<Self>) {
        dispatch.reduce_mut(move |store| {
            store.language = lang;
        })
    }
}
