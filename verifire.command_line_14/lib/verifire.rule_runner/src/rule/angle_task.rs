use angle::solution::solver::AngleSolver;
use foundation::export::convert::EdgeToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{VerificationResult, ProgressNotificator};

pub struct AngleTask {
    solver: AngleSolver,
}

impl AngleTask {
    pub fn new(shapes_res: ShapeRes, is_45_allowed: bool) -> Self {
        Self { solver: AngleSolver::new(shapes_res, is_45_allowed) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}