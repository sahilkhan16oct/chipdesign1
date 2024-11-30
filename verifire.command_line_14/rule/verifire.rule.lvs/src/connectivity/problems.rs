pub(crate) struct OpenError {
    pub(crate) label: String,
    pub(crate) cells: Vec<String>,
    pub(crate) device: usize
}

pub(crate) struct OpenWarning {
    pub(crate) label: String,
    pub(crate) device: usize
}

pub(crate) struct ShortError {
    pub(crate) label_a: String,
    pub(crate) label_b: String,
}

pub(crate) struct ShortWarning {
    pub(crate) label_a: String,
    pub(crate) label_b: String,
}

pub(crate) enum ConnectivityProblem {
    OpenError(OpenError),
    OpenWarning(OpenWarning),
    ShortError(ShortError),
    ShortWarning(ShortWarning),
}

impl OpenError {
    pub(crate) fn to_string(&self) -> String {
        if self.cells.len() > 1 {
            format!("(label: {}; device: {}; cells: {})", self.label, self.device, self.cells.join(", "))
        } else {
            format!("(label: {}; device: {}; cell: {})", self.label, self.device, self.cells[0])
        }
    }
}

impl OpenWarning {
    pub(crate) fn to_string(&self) -> String {
        format!("( label: {}; device: {})", self.label, self.device)
    }
}

impl ShortError {
    pub(crate) fn to_string(&self) -> String {
        format!("(labels: [{}, {}])", self.label_a, self.label_b)
    }
}

impl ShortWarning {
    pub(crate) fn to_string(&self) -> String {
        format!("(labels: [{}, {}])", self.label_a, self.label_b)
    }
}