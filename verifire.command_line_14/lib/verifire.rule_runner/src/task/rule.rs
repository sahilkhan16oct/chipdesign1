use tokio::sync::mpsc;
use foundation::task::task::{ProgressNotificator, VerificationProgress, VerificationResult};
use crate::rule::angle_task::AngleTask;
use crate::rule::check_shape_task::CheckShapeTask;
use crate::rule::connectivity_task::ConnectivityTask;
use crate::rule::density_task::DensityTask;
use crate::rule::exact_size_task::ExactSizeTask;
use crate::rule::extension_task::ExtensionTask;
use crate::rule::filter_task::FilterTask;
use crate::rule::lvs_task::LvsTask;
use crate::rule::min_area_task::MinAreaTask;
use crate::rule::min_max_width_task::MinMaxWidthTask;
use crate::rule::notch_task::NotchTask;
use crate::rule::off_grid_task::OffGridTask;
use crate::rule::poly_enclosure_task::PolyEnclosureTask;
use crate::rule::rect_enclosure_task::RectEnclosureTask;
use crate::rule::slit_length::SlitLengthTask;
use crate::rule::space_task::SpaceTask;

pub enum RuleTask {
    Angle(AngleTask),
    CheckShape(CheckShapeTask),
    Connectivity(ConnectivityTask),
    Density(DensityTask),
    Lvs(LvsTask),
    RectEnclosure(RectEnclosureTask),
    PolyEnclosure(PolyEnclosureTask),
    ExactSize(ExactSizeTask),
    Extension(ExtensionTask),
    Filter(FilterTask),
    MinArea(MinAreaTask),
    MinMaxWidth(MinMaxWidthTask),
    Notch(NotchTask),
    OffGrid(OffGridTask),
    SlitLength(SlitLengthTask),
    SpaceOneLayer(SpaceTask),
    SpaceTwoLayers(SpaceTask),
}

pub struct RuleResult<Id> {
    pub id: Id,
    pub result: VerificationResult,
}

pub struct VerificationTask<Id> {
    id: Id,
    rule: RuleTask,
}

impl<Id: Clone + Send + 'static> VerificationTask<Id> {
    pub fn id(&self) -> &Id {
        &self.id
    }

    pub fn rule(&self) -> &RuleTask {
        &self.rule
    }

    pub fn new(id: Id, rule: RuleTask) -> Self {
        Self { id, rule }
    }

    pub async fn run(&self, sender: mpsc::Sender<VerificationProgress<Id>>) -> RuleResult<Id> {
        let notificator = ProgressNotificator::new(self.id.clone(), sender);

        match &self.rule {
            RuleTask::Angle(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::CheckShape(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::Connectivity(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::Density(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::Lvs(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::PolyEnclosure(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::RectEnclosure(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::ExactSize(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::Extension(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::Filter(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::MinArea(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::MinMaxWidth(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::Notch(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::OffGrid(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::SlitLength(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::SpaceOneLayer(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
            RuleTask::SpaceTwoLayers(task) => {
                let result = task.run(notificator).await;
                RuleResult { id: self.id.clone(), result }
            }
        }
    }
}