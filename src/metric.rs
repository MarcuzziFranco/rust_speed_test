pub struct Metric {
    pub latency: Vec<f32>,
    pub download: Vec<f32>,
    pub upload: Vec<f32>,
    pub time: Vec<f32>,
}

impl Metric {
    pub fn new() -> Self {
        Self {
            latency: Vec::new(),
            download: Vec::new(),
            upload: Vec::new(),
            time: Vec::new(),
        }
    }
}
