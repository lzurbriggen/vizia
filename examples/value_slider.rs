use vizia::*;

#[derive(Lens)]
pub struct AppData {
    val: f32,
}

pub enum AppEvent {
    SetValue(f32),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetValue(val) => {
                    self.val = *val;
                }
            }
        }
    }
}

fn main() {
    let window_description = WindowDescription::new().with_title("Value Slider");
    Application::new(window_description, |cx| {
        AppData { val: 0.5 }.build(cx);

        ValueSlider::new(cx, AppData::val)
            .on_changing(|cx, val| cx.emit(AppEvent::SetValue(val)))
            .width(Pixels(300.0))
            .height(Pixels(50.0))
            .space(Stretch(1.0));
    })
    .run();
}
