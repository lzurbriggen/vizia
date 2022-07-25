use crate::prelude::*;

/// Modifiers which alter the styling of a view.
pub trait StyleModifiers: Sized {
    modifier!(
        /// Sets the display property of the view.
        ///
        /// Display determines whether an entity will be rendered and acted on by the layout system.
        /// To make an entity invisible to rendering but still visible to layout, see [Visibility].
        display,
        Display
    );

    modifier!(
        /// Sets the visibility of the view.
        ///
        /// Visibility determines whether an entity will be rendered.
        /// An invisible entity will still be acted upon by the layout system.
        /// Use [Display] to hide an entity from both rendering and layout.
        visibility,
        Visibility
    );

    modifier!(
        /// Sets the background color of the view.
        background_color,
        Color
    );

    modifier!(
        /// Sets the background image of the view.
        background_image,
        String
    );

    modifier!(
        /// Sets the border width of the view.
        border_width,
        Units
    );

    modifier!(
        /// Sets the border color of the view.
        border_color,
        Color
    );

    modifier!(
        /// Sets the z-order of the view.
        z_order,
        i32
    );

    modifier!(
        /// Sets the overflow of the contents of the view.
        overflow,
        Overflow
    );
}

#[doc(hidden)]
impl<V: View> StyleModifiers for Handle<'_, V> {
    modifier_impl!(display, Display);
    modifier_impl!(visibility, Visibility);
    modifier_impl!(background_color, Color);
    modifier_impl!(background_image, String);
    modifier_impl!(border_width, Units);
    modifier_impl!(border_color, Color);
    modifier_impl!(z_order, i32);
}
