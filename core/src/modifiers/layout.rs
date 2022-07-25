use crate::prelude::*;

/// Modifiers which alter the layout of a view.
pub trait LayoutModifiers: Sized {
    modifier!(
        /// Sets the layout type of the view.
        layout_type,
        LayoutType
    );

    modifier!(
        /// Sets the position type of the view.
        position_type,
        PositionType
    );

    modifier!(
        /// Sets the spacing applied to the left of the view.
        left,
        Units
    );

    modifier!(
        /// Sets the spacing applied to the right of the view.
        right,
        Units
    );

    modifier!(
        /// Sets the spacing applied above the view.
        top,
        Units
    );

    modifier!(
        /// Sets the spacing applied below the view.
        bottom,
        Units
    );

    /// Sets the space applied to all sides of the view.
    fn space<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self;

    modifier!(
        /// Sets the width of the view.
        width,
        Units
    );

    modifier!(
        /// Sets the height of the view.
        height,
        Units
    );

    /// Sets the width and height of the view.
    fn size<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self;

    modifier!(
        /// Sets the spacing applied to the left of the children of the view.
        child_left,
        Units
    );

    modifier!(
        /// Sets the spacing applied to the right of the children of the view.
        child_right,
        Units
    );

    modifier!(
        /// Sets the spacing applied above the children of the view.
        child_top,
        Units
    );

    modifier!(
        /// Sets the spacing applied below the children of the view.
        child_bottom,
        Units
    );

    /// Sets the space applied around the children of the view.
    fn child_space<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self;

    modifier!(
        /// Sets the spacing applied horizontally between the children of the view.
        col_between,
        Units
    );

    modifier!(
        /// Sets the spacing applied vertically between the children of the view.
        row_between,
        Units
    );

    modifier!(
        /// Sets the minimum constraint on the space to the left of the view.
        min_left,
        Units
    );

    modifier!(
        /// Sets the maximum constraint on the space to the left of the view.
        max_left,
        Units
    );

    modifier!(
        /// Sets the minimum constraint on the space to the right of the view.
        min_right,
        Units
    );

    modifier!(
        /// Sets the maximum constraint on the space to the right of the view.
        max_right,
        Units
    );

    modifier!(
        /// Sets the minimum constraint on the space above the view.
        min_top,
        Units
    );

    modifier!(
        /// Sets the maximum constraint on the space above the view.
        max_top,
        Units
    );

    modifier!(
        /// Sets the minimum constraint on the space below the view.
        min_bottom,
        Units
    );

    modifier!(
        /// Sets the maximum constraint on the space below the view.
        max_bottom,
        Units
    );

    modifier!(
        /// Sets the minimum constraint on the width of the view.
        min_width,
        Units
    );

    modifier!(
        /// Sets the maximum constraint on the width of the view.
        max_width,
        Units
    );

    modifier!(
        /// Sets the minimum constraint on the height of the view.
        min_height,
        Units
    );

    modifier!(
        /// Sets the maximum constraint on the height of the view.
        max_height,
        Units
    );
}

#[doc(hidden)]
impl<V: View> LayoutModifiers for Handle<'_, V> {
    modifier_impl!(left, Units);
    modifier_impl!(right, Units);
    modifier_impl!(top, Units);
    modifier_impl!(bottom, Units);

    fn space<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
            cx.style().left.insert(entity, v.into());
            cx.style().right.insert(entity, v.into());
            cx.style().top.insert(entity, v.into());
            cx.style().bottom.insert(entity, v.into());

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier_impl!(width, Units);
    modifier_impl!(height, Units);

    fn size<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
            cx.style().width.insert(entity, v.into());
            cx.style().height.insert(entity, v.into());

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier_impl!(child_left, Units);
    modifier_impl!(child_right, Units);
    modifier_impl!(child_top, Units);
    modifier_impl!(child_bottom, Units);

    fn child_space<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self {
        value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
            cx.style().child_left.insert(entity, v.into());
            cx.style().child_right.insert(entity, v.into());
            cx.style().child_top.insert(entity, v.into());
            cx.style().child_bottom.insert(entity, v.into());

            cx.need_relayout();
            cx.need_redraw();
        });

        self
    }

    modifier_impl!(col_between, Units);
    modifier_impl!(row_between, Units);

    modifier_impl!(min_left, Units);
    modifier_impl!(max_left, Units);
    modifier_impl!(min_right, Units);
    modifier_impl!(max_right, Units);
    modifier_impl!(min_top, Units);
    modifier_impl!(max_top, Units);
    modifier_impl!(min_bottom, Units);
    modifier_impl!(max_bottom, Units);

    modifier_impl!(min_width, Units);
    modifier_impl!(max_width, Units);
    modifier_impl!(min_height, Units);
    modifier_impl!(max_height, Units);
}
