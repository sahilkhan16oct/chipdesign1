use enclosure_poly::solution::solver::EnclosurePolySolver;
use foundation::export::convert::EdgeToExpShape;
use foundation::res::shape_res::ShapeRes;
use foundation::task::direction::SpaceDirection;
use foundation::task::task::{VerificationResult, ProgressNotificator};

pub struct PolyEnclosureTask {
    solver: EnclosurePolySolver,
}

impl PolyEnclosureTask {
    pub fn new(float_res: ShapeRes, poly_res: ShapeRes, value: i32, direction: SpaceDirection) -> Self {
        let solver = EnclosurePolySolver::new(float_res, poly_res, value, direction);
        Self { solver }
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}