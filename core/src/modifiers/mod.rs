macro_rules! modifier {

    (
        $(#[$meta:meta])*
        $name:ident, $t:ty
    ) => {
        $(#[$meta])*
        #[allow(unused_variables)]
        fn $name<U: Into<$t>>(self, value: impl Res<U>) -> Self {
            self
        }
    };
}

macro_rules! modifier_impl {
    ($name:ident, $t:ty) => {
        fn $name<U: Into<$t>>(self, value: impl Res<U>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
                cx.style().$name.insert(entity, v.into());

                // TODO - Split this out
                cx.need_relayout();
                cx.need_redraw();
            });

            self
        }
    };
}

mod actions;
pub use actions::*;

mod style;
pub use style::*;

mod layout;
pub use layout::*;

mod text;
pub use text::*;
