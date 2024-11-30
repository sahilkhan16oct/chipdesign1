use foundation::i_overlay::core::fill_rule::FillRule;
use foundation::i_overlay::core::overlay::Overlay;
use foundation::i_overlay::core::overlay_rule::OverlayRule;
use foundation::i_overlay::i_shape::int::shape::IntShapes;

pub(crate) trait BooleanOperation {
    fn and(&self, clip: &IntShapes) -> IntShapes;
    fn sub(&self, clip: &IntShapes) -> IntShapes;
}

impl BooleanOperation for IntShapes {
    fn and(&self, clip: &IntShapes) -> IntShapes {
        let overlay = Overlay::with_shapes(self, clip);
        let graph = overlay.into_graph(FillRule::NonZero);
        graph.extract_shapes(OverlayRule::Intersect)
    }

    fn sub(&self, clip: &IntShapes) -> IntShapes {
        let overlay = Overlay::with_shapes(self, clip);
        let graph = overlay.into_graph(FillRule::NonZero);
        graph.extract_shapes(OverlayRule::Difference)
    }
}