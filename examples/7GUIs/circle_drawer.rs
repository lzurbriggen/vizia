use std::collections::VecDeque;

use vizia::*;
use vizia::draw::*;

const STYLE: &str = r#"
    /* * {
        border-width: 1px;
        border-color: red;
    } */

    .dialog {
        background-color: white;
        border-color: black;
        border-width: 1px;
        border-radius: 5px;
    }
"#;

fn main() {
    let window_description = WindowDescription::new();
    let app = Application::new(window_description, |cx|{

        cx.add_theme(STYLE);

        CircleDrawerData {
            circles: vec![CircleData{
                x: 50.0,
                y: 50.0,
                r: 20.0,
            },
            CircleData{
                x: 100.0,
                y: 100.0,
                r: 10.0,
            }],
            menu_open: false,
            dialog_open: true,
            selected: 0,
            undo_list: VecDeque::new(),
            redo_list: VecDeque::new(),
        }.build(cx);

        VStack::new(cx, |cx|{
            HStack::new(cx, |cx|{
                Button::new(cx, |cx| cx.emit(CircleDrawerEvent::Undo), |cx|{
                    Label::new(cx, "Undo").width(Pixels(50.0))
                });

                Button::new(cx, |cx| {}, |cx|{
                    Label::new(cx, "Redo").width(Pixels(50.0))
                });
            })
            .col_between(Pixels(20.0))
            .height(Pixels(40.0))
            .child_space(Stretch(1.0));

            CircleDrawer::new(cx);
        })
        .child_space(Pixels(10.0))
        .row_between(Pixels(10.0));
    });

    app.run();
}

#[derive(Debug, Clone, Data, Lens)]
pub struct CircleData {
    x: f32,
    y: f32,
    r: f32,
}

#[derive(Debug, Lens)]
pub struct CircleDrawerData {
    circles: Vec<CircleData>,
    selected: usize,
    menu_open: bool,
    dialog_open: bool,

    undo_list: VecDeque<CircleDrawerEvent>,
    redo_list: VecDeque<CircleDrawerEvent>,
}

#[derive(Debug)]
pub enum CircleDrawerEvent {
    SelectCircle(f32, f32),
    OpenMenu,
    CloseMenu,
    OpenDialog,
    CloseDialog,
    SetRadius(f32),
    AddCircle(f32, f32),
    RemoveCircle(usize),
    Undo,
    Redo,
}

impl Model for CircleDrawerData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(circle_drawer_event) = event.message.downcast() {
            match circle_drawer_event {
                CircleDrawerEvent::SelectCircle(px, py) => {
                    let mut circle_selected = false;
                    let mut min_dist2 = std::f32::MAX;
                    for (index, circle) in self.circles.iter_mut().enumerate() {
                        let dx = (circle.x - *px).abs();
                        let dy = (circle.y - *py).abs();
                        let dist2 = dx * dx + dy * dy;
                        min_dist2 = min_dist2.min(dist2);
                        if dist2 <= circle.r * circle.r {
                            //circle.selected = true;
                            circle_selected = true;
                            self.selected = index;
                        } 
                    }

                    if !circle_selected && min_dist2 > 400.0 {
                        
                        self.selected = self.circles.len();
                        
                        self.circles.push(CircleData{
                            x: *px,
                            y: *py,
                            r: 10.0,
                        });

                        self.undo_list.push_back(CircleDrawerEvent::RemoveCircle(self.selected));
                    }

                    cx.style.needs_redraw = true;
                }

                CircleDrawerEvent::OpenMenu => {
                    self.menu_open = true;
                }

                CircleDrawerEvent::CloseMenu => {
                    self.menu_open = false;
                }

                CircleDrawerEvent::OpenDialog => {
                    self.dialog_open = true;
                }

                CircleDrawerEvent::CloseDialog => {
                    self.dialog_open = false;
                }

                CircleDrawerEvent::SetRadius(val) => {
                    if let Some(circle) = self.circles.get_mut(self.selected) {
                        circle.r = 10.0 + *val * 200.0;
                    }
                }

                CircleDrawerEvent::RemoveCircle(index) => {
                    self.circles.remove(*index);
                    // if self.selected > self.circles.len() {
                    //     self.selected = self.circles.len();
                    // }
                    self.selected = 0;
                }

                CircleDrawerEvent::Undo => {
                    if let Some(action) = self.undo_list.pop_back() {
                        cx.emit(action);
                    }
                }

                _=> {}
            }
        }
    }
}

pub struct CircleDrawer {

}

impl CircleDrawer {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {

        }.build2(cx, |cx|{

            Popup::new(cx, CircleDrawerData::menu_open, |cx|{
                Button::new(cx, |cx| {
                    cx.emit(CircleDrawerEvent::OpenDialog);
                    cx.emit(CircleDrawerEvent::CloseMenu);
                }, |cx|{
                    Label::new(cx, "Adjust diameter...")
                });
            })
            .something(|cx| cx.emit(CircleDrawerEvent::CloseMenu))
            .size(Auto)
            .background_color(Color::green());

            // HStack::new(cx, |cx|{
            //     Binding::new(cx, CircleDrawerData::selected, |cx, selected|{

            //     });
            // });

            Popup::new(cx, CircleDrawerData::dialog_open, |cx|{
                Binding::new(cx, CircleDrawerData::selected, |cx, selected|{
                    let selected_index = *selected.get(cx);
                    println!("Do this");
                    Slider::new(cx, CircleDrawerData::circles.index(selected_index).then(CircleData::r).map(|radius| (*radius - 10.0)/200.0), Orientation::Horizontal)
                        .on_changing(|cx, val| cx.emit(CircleDrawerEvent::SetRadius(val)));
                });
            })
            .something(|cx| cx.emit(CircleDrawerEvent::CloseDialog))
            .width(Pixels(300.0))
            .height(Pixels(200.0))
            .space(Stretch(1.0))
            .class("dialog");
        })
    }
}

impl View for CircleDrawer {
    fn draw(&self, cx: &mut Context, canvas: &mut Canvas) {

        let bounds = cx.cache.get_bounds(cx.current);

        if let Some(circle_drawer_data) = cx.data::<CircleDrawerData>() {
            for (index, circle) in circle_drawer_data.circles.iter().enumerate() {
                let mut path = Path::new();
                path.circle(bounds.x + circle.x, bounds.y + circle.y, circle.r);

                if index == circle_drawer_data.selected {
                    canvas.fill_path(&mut path, Paint::color(Col::rgb(200, 200, 200)));
                }

                canvas.stroke_path(&mut path, Paint::color(Col::black()));
            }
        }

        let mut path = Path::new();
        path.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        canvas.stroke_path(&mut path, Paint::color(Col::black()));
    }
    

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        let bounds = cx.cache.get_bounds(cx.current);
                        let px = cx.mouse.cursorx - bounds.x;
                        let py = cx.mouse.cursory - bounds.y;
                        cx.emit(CircleDrawerEvent::SelectCircle(px, py));
                    }
                }

                WindowEvent::MouseDown(button) if *button == MouseButton::Right => {
                    if event.target == cx.current {
                        cx.emit(CircleDrawerEvent::OpenMenu);
                    }
                }

                _=> {}
            }
        }
    }
}