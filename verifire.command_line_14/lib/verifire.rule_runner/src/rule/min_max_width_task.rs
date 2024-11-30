use foundation::export::convert::EdgeToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{VerificationResult, ProgressNotificator};
use min_max_width::solution::solver::{MinMaxAlgorithm, MinMaxSolver};

pub struct MinMaxWidthTask {
    solver: MinMaxSolver,
}

impl MinMaxWidthTask {
    pub fn new(shape_res: ShapeRes, algorithm: MinMaxAlgorithm, value: i32) -> Self {
        Self { solver: MinMaxSolver::new(shape_res, algorithm, value) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}