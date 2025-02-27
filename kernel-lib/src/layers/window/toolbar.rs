use auto_delegate::Delegate;

use common_lib::frame_buffer::FrameBufferConfig;
use common_lib::math::size::Size;
use common_lib::math::vector::Vector2D;
use common_lib::transform::transform2d::{Transform2D, Transformable2D};

use crate::error::KernelResult;
use crate::gop;
use crate::gop::config;
use crate::layers::close_button::{CloseButtonLayer, CLOSE_BUTTON_HEIGHT, CLOSE_BUTTON_WIDTH};
use crate::layers::layer::Layer;
use crate::layers::layer_key::LayerKey;
use crate::layers::multiple_layer::{LayerFindable, MultipleLayer};
use crate::layers::shape::shape_drawer::ShapeDrawer;
use crate::layers::shape::ShapeLayer;
use crate::layers::text::colors::TextColors;
use crate::layers::text::{config, TextLayer};

const BACKGROUND_LAYER_KAY: &str = "Window Toolbar Background";
const TITLE_LAYER_KAY: &str = "Window Toolbar Title";


#[derive(Delegate)]
pub struct ToolbarLayer {
    #[to(Transformable2D, LayerUpdatable, LayerFindable)]
    layers: MultipleLayer,
    active_colors: TextColors,
    deactivate_colors: TextColors,
}


impl ToolbarLayer {
    #[inline]
    pub fn new(
        title: &str,
        transform: Transform2D,
        active_colors: TextColors,
        deactivate_colors: TextColors,
    ) -> Self {
        Self {
            layers: toolbar_layer(title, transform, deactivate_colors),
            active_colors,
            deactivate_colors,
        }
    }


    #[inline]
    pub fn activate(&mut self) -> KernelResult {
        self.layers
            .find_by_key_mut(BACKGROUND_LAYER_KAY)
            .unwrap()
            .require_shape()
            .unwrap()
            .set_color(
                *self
                    .active_colors
                    .background_ref(),
            );

        self.layers
            .force_find_by_key_mut(TITLE_LAYER_KAY)
            .require_text()
            .unwrap()
            .change_colors(self.active_colors)
    }


    #[inline]
    pub fn deactivate(&mut self) -> KernelResult {
        self.layers
            .find_by_key_mut(BACKGROUND_LAYER_KAY)
            .unwrap()
            .require_shape()
            .unwrap()
            .set_color(
                *self
                    .deactivate_colors
                    .background_ref(),
            );

        self.layers
            .force_find_by_key_mut(TITLE_LAYER_KAY)
            .require_text()
            .unwrap()
            .change_colors(self.deactivate_colors)
    }


    #[inline]
    pub fn into_enum(self) -> Layer {
        Layer::Toolbar(self)
    }
}


fn toolbar_layer(
    title: &str,
    transform: Transform2D,
    deactivate_colors: TextColors,
) -> MultipleLayer {
    let mut layer = MultipleLayer::new(transform.clone());

    layer.new_layer(toolbar_background_layer(transform, deactivate_colors));
    layer.new_layer(toolbar_title_layer(title, deactivate_colors));
    layer.new_layer(toolbar_close_button(config(), &layer.transform()));

    layer
}


fn toolbar_background_layer(transform: Transform2D, deactivate_colors: TextColors) -> LayerKey {
    ShapeLayer::new(
        ShapeDrawer::new(config(), *deactivate_colors.background_ref()),
        transform,
    )
    .into_enum()
    .into_layer_key(BACKGROUND_LAYER_KAY)
}


fn toolbar_title_layer(title: &str, deactivate_colors: TextColors) -> LayerKey {
    let config = config::Builder::new()
        .colors(deactivate_colors)
        .build();

    let mut text = TextLayer::new(
        gop::config(),
        Vector2D::new(24, 4),
        Size::new(12, 1),
        config,
    )
    .unwrap();

    text.update_string(title)
        .unwrap();

    text.into_enum()
        .into_layer_key(TITLE_LAYER_KAY)
}


fn toolbar_close_button(config: FrameBufferConfig, transform: &Transform2D) -> LayerKey {
    CloseButtonLayer::new(
        config,
        Vector2D::new(
            transform.size().width() - CLOSE_BUTTON_WIDTH - 5,
            (transform.size().height() - CLOSE_BUTTON_HEIGHT) / 2,
        ),
    )
    .into_enum()
    .into_layer_key("toolbar close button")
}
