use std::collections::HashMap;
use foundation::fix::layer::LayerData;
use foundation::fix::layer_binding::LayerBind;
use foundation::fix::mark::Mark;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};
use lvs::solution::lvs::{LvsSolver, LvsTransistor};

pub struct LvsTask {
    solver: LvsSolver,
}

impl LvsTask {
    pub fn new(
        cell_name: String,
        scale_to_physics: f64,
        poly: ShapeRes,
        diffusion: ShapeRes,
        poly_layer: LayerData,
        diffusion_layer: LayerData,
        transistors: Vec<LvsTransistor>,
        shapes_res: HashMap<LayerData, ShapeRes>,
        stack_binds: Vec<LayerBind>,
        mark_store: HashMap<LayerData, Vec<Mark>>,
        mark_binds: Vec<LayerBind>,
    ) -> Self {
        Self {
            solver: LvsSolver::new(
                cell_name,
                scale_to_physics,
                poly,
                diffusion,
                poly_layer,
                diffusion_layer,
                transistors,
                shapes_res,
                stack_binds,
                mark_store,
                mark_binds,
            )
        }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let result = match self.solver.find_problems() {
            Ok(lvs) => {
                VerificationResult::Lvs(lvs)
            }
            Err(e) => {
                VerificationResult::Fail(e)
            }
        };
        notificator.send(1.0).await;

        result
    }
}