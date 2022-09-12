use crate::prelude::*;

pub struct Menu2 {
    is_open: bool,
    open_submenu: usize,
}

impl Menu2 {
    pub fn new<L, T: 'static>(cx: &mut Context, lens: L) -> Handle<Self>
    where
        L: Lens<Target = Vec<T>>,
    {
        Self { is_open: false, open_submenu: 0 }.build(cx, |cx| {})
    }
}

impl View for Menu2 {}
