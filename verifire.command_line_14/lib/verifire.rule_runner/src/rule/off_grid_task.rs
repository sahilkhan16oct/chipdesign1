use foundation::export::convert::PathToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};
use off_grid::solution::solver::OffGridSolver;

pub struct OffGridTask {
    solver: OffGridSolver,
}

impl OffGridTask {
    pub fn new(shape_res: ShapeRes, step: i32) -> Self {
        Self { solver: OffGridSolver::new(shape_res, step) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}