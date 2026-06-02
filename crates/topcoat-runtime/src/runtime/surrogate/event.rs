//! Façade types for DOM events. These exist purely so handler bodies inside
//! `expr!` can be type-checked by rustc — they are never constructed
//! server-side. The browser resolves field accesses against the real DOM
//! `Event` at runtime.

use std::marker::PhantomData;

pub struct Event {
    pub alt_key: bool,
    pub bubbles: bool,
    pub button: crate::runtime::F64,
    pub buttons: crate::runtime::F64,
    pub cancelable: bool,
    pub client_x: crate::runtime::F64,
    pub client_y: crate::runtime::F64,
    pub code: crate::runtime::String,
    pub ctrl_key: bool,
    pub current_target: EventTarget,
    pub data: crate::runtime::String,
    pub default_prevented: bool,
    pub delta_x: crate::runtime::F64,
    pub delta_y: crate::runtime::F64,
    pub delta_z: crate::runtime::F64,
    pub event_type: crate::runtime::String,
    pub input_type: crate::runtime::String,
    pub is_composing: bool,
    pub key: crate::runtime::String,
    pub meta_key: bool,
    pub movement_x: crate::runtime::F64,
    pub movement_y: crate::runtime::F64,
    pub offset_x: crate::runtime::F64,
    pub offset_y: crate::runtime::F64,
    pub page_x: crate::runtime::F64,
    pub page_y: crate::runtime::F64,
    pub pointer_id: crate::runtime::F64,
    pub pointer_type: crate::runtime::String,
    pub repeat: bool,
    pub screen_x: crate::runtime::F64,
    pub screen_y: crate::runtime::F64,
    pub shift_key: bool,
    pub target: EventTarget,
    pub time_stamp: crate::runtime::F64,
    _priv: PhantomData<()>,
}

impl Event {
    pub fn prevent_default(&self) {
        unreachable!();
    }

    pub fn stop_propagation(&self) {
        unreachable!();
    }

    pub fn stop_immediate_propagation(&self) {
        unreachable!();
    }
}

pub struct EventTarget {
    pub checked: bool,
    pub id: crate::runtime::String,
    pub name: crate::runtime::String,
    pub text_content: crate::runtime::String,
    pub value: crate::runtime::String,
}
