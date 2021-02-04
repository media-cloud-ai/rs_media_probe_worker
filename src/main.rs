#[macro_use]
extern crate serde_derive;
use mcai_worker_sdk::{
  job::JobResult, start_worker, JsonSchema, McaiChannel, MessageError, MessageEvent, Version,
};

mod message;

macro_rules! crate_version {
  () => {
    env!("CARGO_PKG_VERSION")
  };
}

#[derive(Debug, Default)]
struct MediaProbeEvent {}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct MediaProbeParameters {
  source_path: String,
  destination_path: Option<String>,
}

impl MessageEvent<MediaProbeParameters> for MediaProbeEvent {
  fn get_name(&self) -> String {
    "Media probe".to_string()
  }

  fn get_short_description(&self) -> String {
    "Probe a source file with ffmpeg".to_string()
  }

  fn get_description(&self) -> String {
    r#"This worker probes an audio/video media file calling the ffmpeg API.
It returns the result as JSON."#
      .to_string()
  }

  fn get_version(&self) -> Version {
    Version::parse(crate_version!()).expect("unable to locate Package version")
  }

  fn process(
    &self,
    channel: Option<McaiChannel>,
    parameters: MediaProbeParameters,
    job_result: JobResult,
  ) -> Result<JobResult, MessageError> {
    message::process(channel, parameters, job_result)
  }
}

fn main() {
  let message_event = MediaProbeEvent::default();
  start_worker(message_event);
}
