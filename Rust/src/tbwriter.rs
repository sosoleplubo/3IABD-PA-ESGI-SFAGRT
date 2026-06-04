use tensorboard_rs::summary_writer::SummaryWriter;

pub struct TBLogger {
    writer: SummaryWriter,
}

impl TBLogger {
    pub fn new(logdir: &str) -> Self {
        Self {
            writer: SummaryWriter::new(logdir),
        }
    }

    pub fn log_loss(&mut self, step: usize, loss: f64) {
        self.writer.add_scalar("loss", loss as f32, step);
    }

    pub fn log_scalar(&mut self, tag: &str, step: usize, value: f64) {
        self.writer.add_scalar(tag, value as f32, step);
    }

    pub fn flush(&mut self) {

    }
}