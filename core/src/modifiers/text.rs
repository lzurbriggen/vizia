use crate::prelude::*;

define_modifiers!(
    trait TextModifiers: Sized;

    basic {
        /// Sets the font used by the view.
        ///
        /// # Example
        /// ```rust, ignore
        /// Application::new(|cx|{
        ///
        ///     context.add_font_mem("custom", include_bytes!("path/to/font"));
        ///
        ///     Label::new(cx, "Hello World")
        ///         .font("custom");
        /// })
        /// .run();
        /// ```
        font: String,
    };

    custom {
        /// Sets the font size used by the view.
        ///
        /// # Example
        /// ```rust, ignore
        /// Application::new(|cx|{
        ///
        ///     Label::new(cx, "Hello World")
        ///         .font_size(24.0);
        /// })
        /// .run();
        /// ```
        fn font_size<U: Copy + Into<f64>>(self, value: impl Res<U>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
                cx.style().font_size.insert(entity, v.into() as f32);

                cx.need_redraw();
            });

            self
        }
    };

    custom {
        /// Sets the font color used by the view.
        ///
        /// # Example
        /// ```rust, ignore
        /// Application::new(|cx|{
        ///
        ///     Label::new(cx, "Hello World")
        ///         .color(Color::red());
        /// })
        /// .run();
        /// ```
        fn color<U: Copy + Into<Color>>(self, value: impl Res<U>) -> Self {
            value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
                cx.style().font_color.insert(entity, v.into());

                cx.need_redraw();
            });

            self
        }
    };
);
