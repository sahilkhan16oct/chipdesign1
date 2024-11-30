use std::collections::HashMap;
use foundation::fix::layer::LayerData;
use foundation::fix::layer_binding::LayerBind;
use foundation::fix::mark::Mark;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{VerificationResult, ProgressNotificator};
use lvs::solution::connectivity::ConnectivitySolver;

pub struct ConnectivityTask {
    solver: ConnectivitySolver
}

impl ConnectivityTask {
    pub fn new(
        shapes_res: HashMap<LayerData, ShapeRes>,
        stack_binds: Vec<LayerBind>,
        mark_store: HashMap<LayerData, Vec<Mark>>,
        mark_binds: Vec<LayerBind>,
        only_errors: bool
    ) -> Self {
        Self { solver: ConnectivitySolver::new(shapes_res, stack_binds, mark_store, mark_binds, only_errors) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let result = match self.solver.find_problems() {
            Ok(contents) => {
                VerificationResult::Content(contents)
            }
            Err(e) => {
                VerificationResult::Fail(e)
            }
        };
        notificator.send(1.0).await;

        result
    }
}

