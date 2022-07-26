use crate::prelude::*;

define_modifiers!(
    /// Modifiers which alter the layout of a view.
    trait LayoutModifiers: Sized;

    basic {
        /// Sets the layout type of the view.
        layout_type: LayoutType,

        /// Sets the position type of the view.
        position_type: PositionType,

        /// Sets the spacing applied to the left of the view.
        left: Units,


        /// Sets the spacing applied to the right of the view.
        right: Units,


        /// Sets the spacing applied above the view.
        top: Units,


        /// Sets the spacing applied below the view.
        bottom: Units,
    };

    custom {
        /// Sets the space applied to all sides of the view.
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
    };

    basic {
        /// Sets the width of the view.
        width: Units,

        /// Sets the height of the view.
        height: Units,
    };

    custom {
        /// Sets the width and height of the view.
        fn size<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v|{
                cx.style().width.insert(entity, v.into());
                cx.style().height.insert(entity, v.into());

                cx.need_relayout();
                cx.need_redraw();
            });

            self
        }

        fn min_size<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v|{
                cx.style().min_width.insert(entity, v.into());
                cx.style().min_height.insert(entity, v.into());
        
                cx.need_relayout();
                cx.need_redraw();
            });
    
            self
        }
    
        fn max_size(self, value: Units) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v|{
                cx.style().max_width.insert(entity, v);
                cx.style().max_height.insert(entity, v);
        
                cx.need_relayout();
                cx.need_redraw();
            });
    
            self
        }
    };

    basic {
        /// Sets the spacing applied to the left of the children of the view.
        child_left: Units,


        /// Sets the spacing applied to the right of the children of the view.
        child_right: Units,


        /// Sets the spacing applied above the children of the view.
        child_top: Units,


        /// Sets the spacing applied below the children of the view.
        child_bottom: Units,
    };

    custom {
        /// Sets the space applied around the children of the view.
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
    };

    basic {
        /// Sets the spacing applied horizontally between the children of the view.
        col_between: Units,

        /// Sets the spacing applied vertically between the children of the view.
        row_between: Units,

        /// Sets the minimum constraint on the space to the left of the view.
        min_left: Units,

        /// Sets the maximum constraint on the space to the left of the view.
        max_left: Units,

        /// Sets the minimum constraint on the space to the right of the view.
        min_right: Units,

        /// Sets the maximum constraint on the space to the right of the view.
        max_right: Units,

        /// Sets the minimum constraint on the space above the view.
        min_top: Units,

        /// Sets the maximum constraint on the space above the view.
        max_top: Units,

        /// Sets the minimum constraint on the space below the view.
        min_bottom: Units,

        /// Sets the maximum constraint on the space below the view.
        max_bottom: Units,

        /// Sets the minimum constraint on the width of the view.
        min_width: Units,

        /// Sets the maximum constraint on the width of the view.
        max_width: Units,

        /// Sets the minimum constraint on the height of the view.
        min_height: Units,

        /// Sets the maximum constraint on the height of the view.
        max_height: Units,
    };

    custom {
        fn grid_rows(self, rows: Vec<Units>) -> Self {
            self.cx.style().grid_rows.insert(self.entity, rows);
    
            self
        }
    
        fn grid_cols(self, cols: Vec<Units>) -> Self {
            self.cx.style().grid_cols.insert(self.entity, cols);
    
            self
        }
    };

    basic {
        row_index: usize,
        row_span: usize,
        col_index: usize,
        col_span: usize,
    };

);
