use foundation::export::convert::EdgeToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::direction::SpaceDirection;
use foundation::task::task::{ProgressNotificator, VerificationResult};
use spacing::solution::builder::SpaceBuilder;
use spacing::solution::solver::{SpaceSolver};

pub struct SpaceTask {
    solver: SpaceSolver,
}

impl SpaceTask {
    pub fn new(main: ShapeRes, second: Option<ShapeRes>, direction: SpaceDirection, spacing: i32) -> Self {
        let builder = SpaceBuilder::new(main, second);
        Self { solver: SpaceSolver::new(builder, spacing, direction) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}