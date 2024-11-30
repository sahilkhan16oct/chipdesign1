use foundation::export::convert::PathToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};
use min_area::solution::solver::MinAreaSolver;

pub struct MinAreaTask {
    solver: MinAreaSolver,
}

impl MinAreaTask {
    pub fn new(shapes_res: ShapeRes, min_area: i64) -> Self {
        Self { solver: MinAreaSolver::new(shapes_res, min_area) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}