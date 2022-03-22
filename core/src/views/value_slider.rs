use std::marker::PhantomData;

use crate::{
    Binding, Color, Context, Element, Event, Handle, Label, Lens, LensExt, Model, Modifiers,
    MouseButton, Textbox, Units::*, View, WindowEvent, ZStack,
};

use crate::TextEvent;

#[derive(Lens)]
pub struct VSDataInternal {
    edit: bool,
}

pub enum ValueSliderEvent {
    SetEdit(bool),
    OnSubmit(f32),
}

impl Model for VSDataInternal {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(value_slider_event) = event.message.downcast() {
            match value_slider_event {
                ValueSliderEvent::SetEdit(flag) => {
                    self.edit = *flag;
                }

                _ => {}
            }
        }
    }
}

pub struct ValueSlider<L> {
    p: PhantomData<L>,
    is_dragging: bool,
    on_changing: Option<Box<dyn Fn(&mut Context, f32)>>,
}

impl<L> ValueSlider<L>
where
    L: Lens<Target = f32>,
{
    pub fn new(cx: &mut Context, lens: L) -> Handle<Self> {
        Self { p: PhantomData::default(), is_dragging: false, on_changing: None }.build2(cx, |cx| {
            VSDataInternal { edit: false }.build(cx);

            ZStack::new(cx, |cx| {
                Element::new(cx)
                    .height(Stretch(1.0))
                    .width(Stretch(1.0))
                    .background_color(Color::rgb(200, 200, 200))
                    .bind(lens.clone(), move |handle, l| {
                        let val = *l.get(handle.cx);
                        handle.width(Percentage(val * 100.0));
                    });
                Binding::new(cx, VSDataInternal::edit, move |cx, edit| {
                    if *edit.get(cx) {
                        Textbox::new(cx, lens.clone().map(|val| format!("{:.2}", val)))
                            .on_build(|cx| {
                                cx.emit(TextEvent::StartEdit);
                                cx.emit(TextEvent::SelectAll);
                            })
                            .on_submit(|cx, txt| {
                                if let Ok(val) = txt.parse::<f32>() {
                                    let val = val.clamp(0.0, 1.0);
                                    cx.emit(ValueSliderEvent::OnSubmit(val));
                                }
                            })
                            .on_edit_end(|cx| {
                                cx.emit(ValueSliderEvent::SetEdit(false));
                            })
                            .background_color(Color::transparent())
                            .child_space(Stretch(1.0))
                            .height(Stretch(1.0))
                            .width(Stretch(1.0));
                    } else {
                        Label::new(cx, lens.clone().map(|val| format!("{:.2}", val)))
                            .child_space(Stretch(1.0))
                            .height(Stretch(1.0))
                            .width(Stretch(1.0))
                            .hoverable(false);
                    };
                });
            });
        })
    }
}

impl<L> View for ValueSlider<L>
where
    L: Lens<Target = f32>,
{
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(value_slider_event) = event.message.downcast() {
            match value_slider_event {
                ValueSliderEvent::OnSubmit(val) => {
                    if let Some(callback) = &self.on_changing {
                        (callback)(cx, *val);
                    }
                }

                _ => {}
            }
        }

        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if cx.modifiers.contains(Modifiers::ALT) {
                        cx.emit(ValueSliderEvent::SetEdit(true));
                    } else {
                        if let Some(vs_data) = cx.data::<VSDataInternal>() {
                            if !vs_data.edit {
                                self.is_dragging = true;
                                cx.capture();

                                let mut dx = (cx.mouse.left.pos_down.0
                                    - cx.cache.get_posx(cx.current))
                                    / cx.cache.get_width(cx.current);

                                dx = dx.clamp(0.0, 1.0);

                                if let Some(callback) = self.on_changing.take() {
                                    (callback)(cx, dx);

                                    self.on_changing = Some(callback);
                                }
                            }
                        }
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.is_dragging = false;
                    cx.release();
                }

                WindowEvent::MouseMove(x, _) => {
                    if self.is_dragging {
                        let mut dx =
                            (*x - cx.cache.get_posx(cx.current)) / cx.cache.get_width(cx.current);

                        dx = dx.clamp(0.0, 1.0);

                        if let Some(callback) = &self.on_changing {
                            (callback)(cx, dx);
                        }
                    }
                }

                _ => {}
            }
        }
    }
}

impl<'a, L> Handle<'a, ValueSlider<L>>
where
    L: Lens<Target = f32>,
{
    pub fn on_changing<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut Context, f32),
    {
        if let Some(slider) =
            self.cx.views.get_mut(&self.entity).and_then(|f| f.downcast_mut::<ValueSlider<L>>())
        {
            slider.on_changing = Some(Box::new(callback));
        }

        self
    }
}
