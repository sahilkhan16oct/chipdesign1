use std::collections::HashMap;
use design_format::fix::layer::LayerData;
use design_format::i_overlay::core::overlay_rule::OverlayRule;
use crate::i_overlay::i_shape::int::shape::IntShapes;
use crate::res::filter::{AreaRes, BooleanAreaRes, FilterRes, SelectAreaRes};
use crate::res::shape_res::ShapeRes;
use crate::res::viewport::{AreaOperation, Viewport};

pub trait StoreRes {
    fn resource_by_layer(&self, layer: &LayerData) -> Result<ShapeRes, String>;
    fn resource_by_viewport(&self, viewport: &Viewport) -> Result<ShapeRes, String>;
    fn shapes(&self, layer: &LayerData) -> Result<IntShapes, String>;
}

impl StoreRes for HashMap<LayerData, IntShapes> {
    fn resource_by_layer(&self, layer: &LayerData) -> Result<ShapeRes, String> {
        let base = self.shapes(layer)?.clone();
        Ok(ShapeRes { base, filter: None })
    }

    fn resource_by_viewport(&self, viewport: &Viewport) -> Result<ShapeRes, String> {
        let base = self.shapes(&viewport.base)?.clone();
        let filter = if let Some(filter) = &viewport.filter {
            if filter.areas.is_empty() {
                return Ok(ShapeRes { base, filter: None });
            }
            filter
        } else {
            return Ok(ShapeRes { base, filter: None });
        };

        let mut areas = Vec::with_capacity(filter.areas.len());

        for area in filter.areas.iter() {
            let shapes = self.shapes(&area.layer)?;
            match area.operation {
                AreaOperation::Union => {
                    areas.push(AreaRes::Boolean(BooleanAreaRes { shapes, operation: OverlayRule::Union }));
                }
                AreaOperation::Intersection => {
                    areas.push(AreaRes::Boolean(BooleanAreaRes { shapes, operation: OverlayRule::Intersect }));
                }
                AreaOperation::Difference => {
                    areas.push(AreaRes::Boolean(BooleanAreaRes { shapes, operation: OverlayRule::Difference }));
                }
                AreaOperation::Exclusion => {
                    areas.push(AreaRes::Boolean(BooleanAreaRes { shapes, operation: OverlayRule::Xor }));
                }
                AreaOperation::Select => {
                    areas.push(AreaRes::Select(SelectAreaRes { shapes, inverted: false, full_include: false }));
                }
                AreaOperation::InvertSelect => {
                    areas.push(AreaRes::Select(SelectAreaRes { shapes, inverted: true, full_include: false }));
                }
                AreaOperation::SelectFull => {
                    areas.push(AreaRes::Select(SelectAreaRes { shapes, inverted: false, full_include: true }));
                }
                AreaOperation::InvertSelectFull => {
                    areas.push(AreaRes::Select(SelectAreaRes { shapes, inverted: true, full_include: true }));
                }
            }
        }

        return Ok(ShapeRes { base, filter: Some(FilterRes { areas }) });
    }

    fn shapes(&self, layer: &LayerData) -> Result<IntShapes, String> {
        if let Some(shapes) = self.get(layer) {
            Ok(shapes.clone())
        } else {
            Err(format!("Can not get shapes, layer: {layer} is not present"))
        }
    }
}