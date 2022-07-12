use std::fmt;

use yakui_core::{Color3, Constraints, Vec2, Widget};

use crate::util::widget;
use crate::Pad;

pub struct Window {
    pub initial_size: Vec2,
    children: Option<Box<dyn Fn()>>,
}

impl Window {
    pub fn new<S: Into<Vec2>>(initial_size: S) -> Self {
        Self {
            initial_size: initial_size.into(),
            children: None,
        }
    }

    pub fn show<F: 'static + Fn()>(mut self, children: F) -> WindowResponse {
        self.children = Some(Box::new(children));
        widget::<WindowWidget>(self)
    }
}

#[derive(Debug)]
pub struct WindowWidget {
    props: Window,
    size: Vec2,
}

pub type WindowResponse = ();

impl Widget for WindowWidget {
    type Props = Window;
    type Response = WindowResponse;

    fn new(props: Self::Props) -> Self {
        Self {
            size: props.initial_size,
            props,
        }
    }

    fn update(&mut self, props: Self::Props) {
        self.props = props;
    }

    fn respond(&mut self) -> Self::Response {}

    fn children(&self) {
        crate::colored_box_container(Color3::GRAY, || {
            crate::column(|| {
                // Window Title Bar
                let constraints = Constraints::loose(self.size);
                crate::constrained(constraints, || {
                    crate::colored_box_container(Color3::rgb(92, 92, 92), || {
                        crate::pad(Pad::even(8.0), || {
                            let row = crate::List::horizontal();
                            row.show(|| {
                                crate::expanded(|| {
                                    crate::text(16.0, "Yakui Window");
                                });
                                crate::colored_box(Color3::RED, [16.0, 16.0]);
                            });
                        });
                    });
                });

                // Window Contents
                crate::constrained(constraints, || {
                    let contents = crate::ColoredBox::sized(Color3::rgb(60, 60, 60), self.size);
                    if let Some(children) = &self.props.children {
                        contents.show_children(children);
                    } else {
                        contents.show();
                    }
                });
            });
        });
    }
}

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Window")
            .field("size", &self.initial_size)
            .finish_non_exhaustive()
    }
}
