use yakui_core::geometry::{Color, Constraints, Rect, Vec2};
use yakui_core::paint::PaintRect;
use yakui_core::widget::{LayoutContext, PaintContext, Widget};
use yakui_core::{Response, TextureId};

use crate::util::widget;

/**
Displays an image.

Responds with [ImageResponse].
*/
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Image {
    pub image: Option<TextureId>,
    pub rect: Rect,
    pub atlas_size: Option<Vec2>,
}

impl Image {
    pub fn new<I>(image: I, size: Vec2) -> Self
    where
        I: Into<TextureId>,
    {
        Self {
            image: Some(image.into()),
            rect: Rect::from_pos_size(Vec2::ZERO, size),
            atlas_size: None,
        }
    }

    pub fn new_image_rect<I>(image: I, rect: Rect, atlas_size: Vec2) -> Self
    where
        I: Into<TextureId>,
    {
        Self {
            image: Some(image.into()),
            rect,
            atlas_size: Some(atlas_size),
        }
    }

    pub fn show(self) -> Response<ImageWidget> {
        widget::<ImageWidget>(self)
    }
}

#[derive(Debug)]
pub struct ImageWidget {
    props: Image,
}

pub type ImageResponse = ();

impl Widget for ImageWidget {
    type Props = Image;
    type Response = ImageResponse;

    fn new() -> Self {
        Self {
            props: Image {
                image: None,
                rect: Rect::ZERO,
                atlas_size: None,
            },
        }
    }

    fn update(&mut self, props: Self::Props) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, _ctx: LayoutContext<'_>, input: Constraints) -> Vec2 {
        input.constrain_min(self.props.rect.size())
    }

    fn paint(&self, ctx: PaintContext<'_>) {
        let layout_node = ctx.layout.get(ctx.dom.current()).unwrap();

        if let Some(image) = self.props.image {
            let mut rect = PaintRect::new(layout_node.rect);

            let texture_rect = if let Some(atlas_size) = self.props.atlas_size {
                self.props.rect.div_vec2(atlas_size)
            } else {
                Rect::ONE
            };

            rect.color = Color::WHITE;
            rect.texture = Some((image, texture_rect));
            rect.add(ctx.paint);
        }
    }
}
