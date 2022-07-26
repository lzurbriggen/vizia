use crate::prelude::*;

define_modifiers!(
    /// Modifiers which alter the styling of a view.
    trait StyleModifiers: Sized;

    custom {
        /// Sets the style id of the view.
        ///
        /// The style id must be unique.
        ///
        /// # Example
        /// ```rust, ignore
        /// Application::new(|cx|{
        ///     Element::new(cx).id("foo");
        /// }).run();
        /// ```
        /// In a css file:
        /// ```css
        /// #foo {
        ///     background_color: red;
        /// }
        /// ```
        fn id(self, id: &str) -> Self {
            self.cx.style().ids.insert(self.entity, id.to_owned()).expect("Could not insert id");
            self.cx.need_restyle();

            self
        }

        /// Adds a class name to the view.
        ///
        /// A view can have multiple differing class names.
        ///
        /// # Example
        /// ```rust, ignore
        /// Application::new(|cx|{
        ///     Element::new(cx).class("foo");
        /// }).run();
        /// ```
        /// In a css file:
        /// ```css
        /// .foo {
        ///     background_color: red;
        /// }
        /// ```
        fn class(self, name: &str) -> Self {
            if let Some(class_list) = self.cx.style().classes.get_mut(self.entity) {
                class_list.insert(name.to_string());
            }

            self.cx.need_restyle();

            self
        }

        fn toggle_class(self, name: &str, applied: impl Res<bool>) -> Self {
            let name = name.to_owned();
            applied.set_or_bind(self.cx, self.entity, move |cx, entity, applied| {
                if let Some(class_list) = cx.style().classes.get_mut(entity) {
                    if applied {
                        class_list.insert(name.clone());
                    } else {
                        class_list.remove(&name);
                    }
                }
    
                cx.need_restyle();
            });
    
            self
        }
    };

    basic {
        /// Sets the display property of the view.
        ///
        /// Display determines whether an entity will be rendered and acted on by the layout system.
        /// To make an entity invisible to rendering but still visible to layout, see [Visibility].
        display: Display,

        /// Sets the visibility of the view.
        ///
        /// Visibility determines whether an entity will be rendered.
        /// An invisible entity will still be acted upon by the layout system.
        /// Use [Display] to hide an entity from both rendering and layout.
        visibility: Visibility,

        /// Sets the background color of the view.
        background_color: Color,

        /// Sets the background image of the view.
        background_image: String,

        /// Sets the border width of the view.
        border_width: Units,

        /// Sets the border color of the view.
        border_color: Color,

        border_shape_top_left: BorderCornerShape,
        border_shape_top_right: BorderCornerShape,
        border_shape_bottom_left: BorderCornerShape,
        border_shape_bottom_right: BorderCornerShape,
    
        border_radius_top_left: Units,
        border_radius_top_right: Units,
        border_radius_bottom_left: Units,
        border_radius_bottom_right: Units,

    };

    custom {
        fn border_radius<U: Copy + Into<Units>>(self, value: impl Res<U>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v|{
                cx.style().border_radius_top_left.insert(entity, v.into());
                cx.style().border_radius_top_right.insert(entity, v.into());
                cx.style().border_radius_bottom_left.insert(entity, v.into());
                cx.style().border_radius_bottom_right.insert(entity, v.into());
        
                cx.need_redraw();
            });
    
            self
        }
    };

    basic {

        /// Sets the z-order of the view.
        z_order: i32,

        /// Sets the overflow of the contents of the view.
        overflow: Overflow,

        /// 
        cursor: CursorIcon,
    };
);
