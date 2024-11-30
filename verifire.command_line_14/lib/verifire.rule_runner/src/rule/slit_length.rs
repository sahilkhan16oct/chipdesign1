use foundation::export::convert::EdgeToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{VerificationResult, ProgressNotificator};
use min_max_width::solution::solver::MinMaxAlgorithm;
use slit_length::solution::solver::SlitLengthSolver;

pub struct SlitLengthTask {
    solver: SlitLengthSolver
}

impl SlitLengthTask {
    pub fn new(shapes_res: ShapeRes, algorithm: MinMaxAlgorithm, value: i32) -> Self {
        Self { solver: SlitLengthSolver::new(shapes_res, algorithm, value) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}