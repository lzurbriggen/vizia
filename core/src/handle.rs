use std::marker::PhantomData;

use crate::prelude::*;
use crate::text::Selection;

macro_rules! set_style {
    ($name:ident, $t:ty) => {
        pub fn $name(self, value: impl Res<$t>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
                cx.style().$name.insert(entity, v);

                // TODO - Split this out
                cx.need_relayout();
                cx.need_redraw();
            });

            // self.cx.style().$name.insert(self.entity, value.get_val(self.cx).into());

            // // TODO - Split this out
            // self.cx.need_relayout();
            // self.cx.need_redraw();

            self
        }
    };
}

/// A handle to a view which has been already built into the tree.
///
/// This type is part of the prelude.
pub struct Handle<'a, V: View> {
    pub entity: Entity,
    pub p: PhantomData<V>,
    pub cx: &'a mut Context,
}

impl<'a, V: View> Handle<'a, V> {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn ignore(self) -> Self {
        self.cx.tree().set_ignored(self.entity, true);
        self.focusable(false)
    }

    pub fn modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
        V: 'static,
    {
        if let Some(view) = self
            .cx
            .views
            .get_mut(&self.entity)
            .and_then(|view_handler| view_handler.downcast_mut::<V>())
        {
            (f)(view);
        }

        self
    }

    /// Callback which is run when the view is built/rebuilt
    pub fn on_build<F>(self, callback: F) -> Self
    where
        F: Fn(&mut Context),
    {
        self.cx.with_current(self.entity(), |cx| {
            (callback)(cx);
        });

        self
    }

    pub fn bind<L, F>(self, lens: L, closure: F) -> Self
    where
        L: Lens,
        <L as Lens>::Target: Data,
        F: 'static + Fn(Handle<'_, V>, L),
    {
        let entity = self.entity();
        Binding::new(self.cx, lens, move |cx, data| {
            let new_handle = Handle { entity, p: Default::default(), cx };

            new_handle.cx.set_current(new_handle.entity);
            (closure)(new_handle, data);
        });
        self
    }

    pub fn checked(self, state: impl Res<bool>) -> Self {
        state.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            if let Some(pseudo_classes) = cx.style().pseudo_classes.get_mut(entity) {
                pseudo_classes.set(PseudoClass::CHECKED, val);
            } else {
                let mut pseudoclass = PseudoClass::empty();
                pseudoclass.set(PseudoClass::CHECKED, val);
                cx.style().pseudo_classes.insert(entity, pseudoclass).unwrap();
            }

            cx.need_restyle();
        });

        self
    }

    pub fn disabled(self, state: impl Res<bool>) -> Self {
        state.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            cx.style().disabled.insert(entity, val);
            cx.need_restyle();
        });

        self
    }

    pub fn text<U: ToString>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            if let Some(prev_data) = cx.style().text.get(entity) {
                if prev_data != &val.to_string() {
                    cx.style().text.insert(entity, val.to_string());

                    cx.need_relayout();
                    cx.need_redraw();
                }
            } else {
                cx.style().text.insert(entity, val.to_string());

                cx.need_relayout();
                cx.need_redraw();
            }
        });

        self
    }

    pub fn image<U: ToString>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, val| {
            let val = val.to_string();
            if let Some(prev_data) = cx.style().image.get(entity) {
                if prev_data != &val {
                    cx.style().image.insert(entity, val);

                    cx.need_redraw();
                }
            } else {
                cx.style().image.insert(entity, val);

                cx.need_redraw();
            }
        });

        self
    }

    // Abilities
    pub fn hoverable(self, state: bool) -> Self {
        if let Some(abilities) = self.cx.style().abilities.get_mut(self.entity) {
            abilities.set(Abilities::HOVERABLE, state);
        }

        self.cx.need_restyle();

        self
    }

    pub fn focusable(self, state: bool) -> Self {
        if let Some(abilities) = self.cx.style().abilities.get_mut(self.entity) {
            abilities.set(Abilities::FOCUSABLE, state);
        }

        self.cx.need_restyle();

        self
    }

    set_style!(text_selection, Selection);
    set_style!(caret_color, Color);
    set_style!(selection_color, Color);
    set_style!(text_wrap, bool);

    set_style!(rotate, f32);
    set_style!(translate, (f32, f32));
    set_style!(scale, (f32, f32));


}
