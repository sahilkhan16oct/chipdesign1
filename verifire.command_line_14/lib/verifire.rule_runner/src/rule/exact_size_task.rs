use exact_size::solution::exact_size_solver::ExactSizeSolver;
use foundation::export::convert::PathToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};

pub struct ExactSizeTask {
    solver: ExactSizeSolver,
}

impl ExactSizeTask {
    pub fn new(shape_res: ShapeRes, width: i32, height: i32) -> Self {
        Self { solver: ExactSizeSolver::new(shape_res, width, height) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}