use enclosure_rect::solution::solver::EnclosureRectSolver;
use foundation::res::shape_res::ShapeRes;
use foundation::export::convert::PathToExpShape;
use foundation::task::task::{VerificationResult, ProgressNotificator};

pub struct RectEnclosureTask {
    solver: EnclosureRectSolver,
}

impl RectEnclosureTask {
    pub fn new(
        expression: &str,
        float: ShapeRes,
        rects: ShapeRes,
        value_0: i32,
        value_1: Option<i32>,
        only_inner: Option<bool>,
    ) -> Result<Self, String> {
        let solver = EnclosureRectSolver::new(expression, float, rects, value_0, value_1, only_inner)?;
        Ok(Self { solver })
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let errors = self.solver.find_problems();
        notificator.send(1.0).await;

        errors.into_task_result()
    }
}