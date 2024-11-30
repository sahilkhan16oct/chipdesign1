use density::solution::solver::{DensityResultType, DensitySolver};
use foundation::export::convert::{EdgeToExpShape, PathToExpShape};
use foundation::res::shape_res::ShapeRes;
use foundation::task::task::{VerificationResult, ProgressNotificator};

pub struct DensityTask {
    solver: DensitySolver,
    is_debug: bool,
}

impl DensityTask {
    pub fn new(
        shapes_res: ShapeRes,
        density: f64,
        sampling_level: usize,
        window_width: i32,
        window_height: i32,
        result_type: DensityResultType,
        is_debug: Option<bool>
    ) -> Result<Self, String> {
        let solver = DensitySolver::new(shapes_res, density, sampling_level, window_width, window_height, result_type)?;
        Ok(Self { solver, is_debug: is_debug.unwrap_or(false) })
    }

    pub async fn run<Id: Clone>(&self, notificator: ProgressNotificator<Id>) -> VerificationResult {
        notificator.send(0.0).await;
        let result = if self.is_debug {
            let errors = self.solver.debug_find_problems();
            notificator.send(1.0).await;
            errors.into_task_result()
        } else {
            let errors = self.solver.find_problems(notificator).await;
            errors.into_task_result()
        };

        result
    }
}