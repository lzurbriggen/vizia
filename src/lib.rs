#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/vizia/vizia/main/assets/branding/vizia-logo-01.png"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/vizia/vizia/main/assets/branding/vizia-logo-01.png"
)]

#[cfg(all(not(feature = "baseview"), feature = "winit"))]
pub use vizia_winit::application::Application;

#[cfg(all(not(feature = "winit"), feature = "baseview"))]
pub use vizia_baseview::{Application, ParentWindow, WindowScalePolicy};

pub use vizia_core::*;

pub mod prelude {
    pub use vizia_core::prelude::*;

    #[cfg(all(not(feature = "baseview"), feature = "winit"))]
    pub use vizia_winit::application::Application;

    #[cfg(all(not(feature = "winit"), feature = "baseview"))]
    pub use vizia_baseview::Application;
}
