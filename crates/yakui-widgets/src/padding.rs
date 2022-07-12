use yakui_core::dom::Dom;
use yakui_core::layout::LayoutDom;
use yakui_core::{Constraints, Response, Vec2, Widget};

use crate::util::widget_children;

#[derive(Debug, Clone, Copy)]
pub struct Pad {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Pad {
    pub fn even(value: f32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn show<F: FnOnce()>(self, children: F) -> Response<PadWidget> {
        widget_children::<PadWidget, F>(children, self)
    }
}

#[derive(Debug)]
pub struct PadWidget {
    props: Pad,
}

pub type PadResponse = ();

impl Widget for PadWidget {
    type Props = Pad;
    type Response = PadResponse;

    fn new(props: Self::Props) -> Self {
        Self { props }
    }

    fn update(&mut self, props: Self::Props) {
        self.props = props;
    }

    fn layout(&self, dom: &Dom, layout: &mut LayoutDom, input: Constraints) -> Vec2 {
        let node = dom.get_current();

        let mut self_size = Vec2::ZERO;

        let total_padding = Vec2::new(
            self.props.left + self.props.right,
            self.props.top + self.props.bottom,
        );
        let offset = Vec2::new(self.props.left, self.props.top);

        let child_constraints = Constraints {
            min: input.min - total_padding,
            max: input.max - total_padding,
        };

        for &child in &node.children {
            self_size = layout.calculate(dom, child, child_constraints) + total_padding;
            layout.set_pos(child, offset);
        }

        input.constrain(self_size)
    }

    fn respond(&mut self) -> Self::Response {}
}
