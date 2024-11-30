use tokio::sync::mpsc;
use crate::export::lvs_file::LvsFile;
use crate::export::shape::ExpShape;

pub enum VerificationResult {
    Content(Vec<Content>),
    Shape(Vec<ExpShape>),
    Fail(String),
    Lvs(LvsFile),
    NoErrors,
}

pub struct Content {
    pub text: String,
    pub shapes: Vec<ExpShape>,
}

pub struct VerificationProgress<Id> {
    pub task_id: Id,
    pub progress: f64,
}

pub struct ProgressNotificator<Id> {
    id: Id,
    sender: mpsc::Sender<VerificationProgress<Id>>,
}

impl<Id: Clone> ProgressNotificator<Id> {
    pub fn new(id: Id, sender: mpsc::Sender<VerificationProgress<Id>>) -> Self {
        Self { id, sender }
    }

    pub async fn send(&self, progress: f64) {
        match self.sender.send(VerificationProgress { task_id: self.id.clone(), progress }).await {
            Ok(_) => (),
            Err(e) => log::error!("Failed to send progress: {}", e),
        }
    }
}