macro_rules! define_modifiers {
    (
        $(#[$trait_meta:meta])*
        trait $trait_name:ident: $trait_bound:ident;

        $(
            $(
                basic {
                    $(
                        $(#[$modifier_meta:meta])*
                        $modifier_name:ident: $modifier_type:ty,
                    )+
                }
            )?

            $(
                custom {
                    $(
                        $(#[$custom_function_meta:meta])*
                        fn $custom_function_name:ident$(<$generic_name:tt: Copy +  $generic_bound:tt<$generic_inner:tt>>)? (
                            $custom_function_self_ident:ident
                            $(, $custom_function_arg_name:ident: $custom_function_arg_type:ty)* $(,)?
                        ) $( -> $custom_function_return_type:ty )?
                        {
                            $($custom_function_body:tt)*
                        }
                    )+
                }
            )?
        );+
    ) => {
        $(#[$trait_meta])*
        pub trait $trait_name: $trait_bound {
            $(
                $(
                    $(
                        $(#[$modifier_meta])*
                        fn $modifier_name<U: Into<$modifier_type>>(self, value: impl Res<U>) -> Self;
                    )+
                )?
                $(
                    $(
                        $(#[$custom_function_meta])*
                        fn $custom_function_name$(<$generic_name: Copy + $generic_bound<$generic_inner>>)? (
                            $custom_function_self_ident,
                            $($custom_function_arg_name : $custom_function_arg_type),*
                        ) $( -> $custom_function_return_type)?;
                    )+
                )?
            )+
        }

        #[doc(hidden)]
        impl<V: View> $trait_name for Handle<'_, V> {
            $(
                $(
                    $(
                        fn $modifier_name<U: Into<$modifier_type>>(self, value: impl Res<U>) -> Self {
                            value.set_or_bind(self.cx, self.entity, |cx, entity, v| {
                                cx.style().$modifier_name.insert(entity, v.into());

                                // TODO - Split this out
                                cx.need_relayout();
                                cx.need_redraw();
                            });

                            self
                        }
                    )+
                )?
                $(
                    $(
                        fn $custom_function_name$(<$generic_name: Copy + $generic_bound<$generic_inner>>)? (
                            $custom_function_self_ident,
                            $($custom_function_arg_name : $custom_function_arg_type),*
                        ) $( -> $custom_function_return_type)? {
                            $($custom_function_body)*
                        }
                    )+
                )?
            )+
        }
    }
}

mod actions;
pub use actions::*;

mod style;
pub use style::*;

mod layout;
pub use layout::*;

mod text;
pub use text::*;
