use mcai_worker_sdk::job::{Job, JobResult};
use mcai_worker_sdk::worker::{Parameter, ParameterType};
use mcai_worker_sdk::{start_worker, Channel, MessageError, MessageEvent, Version};

mod message;

macro_rules! crate_version {
  () => {
    env!("CARGO_PKG_VERSION")
  };
}

#[derive(Debug)]
struct MediaProbeEvent {}

impl MessageEvent for MediaProbeEvent {
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

  fn get_parameters(&self) -> Vec<Parameter> {
    vec![Parameter {
      identifier: message::SOURCE_PATH_PARAMETER.to_string(),
      label: "Source path".to_string(),
      kind: vec![ParameterType::String],
      required: true,
    }]
  }

  fn process(
    &self,
    channel: Option<&Channel>,
    job: &Job,
    job_result: JobResult,
  ) -> Result<JobResult, MessageError> {
    //    let result = message::process(channel, job, job_result);
    //    mcai_worker_sdk::debug!("result: {:?}", result);
    //    result
    message::process(channel, job, job_result)
  }
}

static MEDIA_PROBE_EVENT: MediaProbeEvent = MediaProbeEvent {};

fn main() {
  start_worker(&MEDIA_PROBE_EVENT);
}
