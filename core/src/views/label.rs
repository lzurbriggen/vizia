use crate::{Context, Handle, Res, View};

pub struct Label;

impl Label {
    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Self>
    where
        T: ToString,
    {
        Self {}.build2(cx, |_| {}).text(text).focusable(false)
    }
}

impl View for Label {
    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }
}
