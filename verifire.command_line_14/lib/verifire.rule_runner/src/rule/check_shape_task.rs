use exact_size::solution::check_shape_solver::{CheckShapeSolver, ShapeForm};
use foundation::export::convert::PathToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};

pub struct CheckShapeTask {
    solver: CheckShapeSolver,
}

impl CheckShapeTask {
    pub fn new(shape_res: ShapeRes, form: ShapeForm) -> Self {
        Self { solver: CheckShapeSolver::new(shape_res, form) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}