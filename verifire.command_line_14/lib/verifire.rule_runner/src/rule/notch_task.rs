use foundation::export::convert::EdgeToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};
use notch_error::solution::solver::NotchSolver;

pub struct NotchTask {
    solver: NotchSolver,
}

impl NotchTask {
    pub fn new(shape_res: ShapeRes, min_length: i32) -> Self {
        Self { solver: NotchSolver::new(shape_res, min_length) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}