use extension::solution::solver::ExtensionSolver;
use foundation::export::convert::EdgeToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};

pub struct ExtensionTask {
    solver: ExtensionSolver,
}

impl ExtensionTask {
    pub fn new(poly: ShapeRes, diffusion: ShapeRes, value: i32) -> Self {
        let solver = ExtensionSolver::new(poly, diffusion, value);
        Self { solver }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}