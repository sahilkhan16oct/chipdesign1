use filter::solution::solver::FilterSolver;
use foundation::export::convert::PathToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{ProgressNotificator, VerificationResult};

pub struct FilterTask {
    solver: FilterSolver,
}

impl FilterTask {
    pub fn new(shapes_res: ShapeRes) -> Self {
        Self { solver: FilterSolver::new(shapes_res) }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let shapes = self.solver.find_problems();
        notificator.send(1.0).await;

        let mut paths = Vec::new();
        for shape in shapes.into_iter() {
            paths.extend(shape)
        }

        paths.into_task_result()
    }
}