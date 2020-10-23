use serde::Serialize;
use serde_json::{Value, Map};
use std::time::Duration;
use std::io;

use crate::docker_run::docker;
use crate::docker_run::run;
use crate::docker_run::config;

#[derive(Debug, serde::Deserialize)]
struct RunRequest {
    image: String,
    limits: RunLimits,
    payload: Map<String, Value>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RunLimits {
    max_execution_time: u64,
    max_output_size: usize,
}



#[derive(Debug)]
pub struct Error {
    pub status_code: u16,
    pub body: Vec<u8>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    pub error: String,
    pub message: String,
}


pub fn handle(config: &config::Config, request: &mut tiny_http::Request) -> Result<Vec<u8>, Error> {

    let reader = request.as_reader();

    let run_request: RunRequest = serde_json::from_reader(reader)
        .map_err(|err| Error{
            status_code: 400,
            body: serde_json::to_vec(&ErrorBody{
                error: "request.parse".to_string(),
                message: format!("Failed to parse json from request: {}", err),
            }).unwrap(),
        })?;

    let container_config = docker::default_container_config(run_request.image);

    let res = run::run(config.unix_socket.clone(), run::RunRequest{
        container_config,
        payload: run_request.payload,
        limits: run::Limits{
            max_execution_time: Duration::from_secs(run_request.limits.max_execution_time),
            max_output_size: run_request.limits.max_output_size,
        },
    });

    match res {
        Ok(data) => {
            // TODO: remove unwrap
            Ok(serde_json::to_vec(&data).unwrap())
        }

        Err(err) => {
            Err(Error{
                status_code: 400,
                body: serde_json::to_vec(&ErrorBody{
                    error: error_code(&err),
                    message: err.to_string(),
                }).unwrap(),
            })
        }
    }
}


// TODO: prefix by internal error, user error, temporary?
pub fn error_code(error: &run::Error) -> String {
    match error {
        run::Error::Connect(_) => {
            "docker.connect".to_string()
        }

        run::Error::SetStreamTimeout(_) => {
            "docker.unixsocket".to_string()
        }

        run::Error::CreateContainer(_) => {
            "docker.container.create".to_string()
        }

        run::Error::StartContainer(_) => {
            "docker.container.start".to_string()
        }

        run::Error::AttachContainer(_) => {
            "docker.container.attach".to_string()
        }

        run::Error::SerializePayload(_) => {
            "docker.container.stream.payload.serialize".to_string()
        }

        run::Error::ReadStream(stream_error) => {
            match stream_error {
                docker::StreamError::Read(_) => {
                    "docker.container.stream.read".to_string()
                }

                docker::StreamError::ReadStreamType(_) => {
                    "docker.container.stream.read".to_string()
                }

                docker::StreamError::UnknownStreamType(_) => {
                    "docker.container.stream.type.unknown".to_string()
                }

                docker::StreamError::ReadStreamLength(_) => {
                    "docker.container.stream.read".to_string()
                }

                docker::StreamError::InvalidStreamLength(_) => {
                    "docker.container.stream.read".to_string()
                }

                docker::StreamError::MaxExecutionTime() => {
                    "limits.execution_time".to_string()
                }

                docker::StreamError::MaxReadSize(_) => {
                    "limits.read.size".to_string()
                }
            }
        }

        run::Error::StreamStdinUnexpected(_) => {
            "coderunner.stdin".to_string()
        }

        run::Error::StreamStderr(_) => {
            "coderunner.stderr".to_string()
        }

        run::Error::StreamStdoutDecode(_) => {
            "coderunner.stdout.decode".to_string()
        }
    }
}
