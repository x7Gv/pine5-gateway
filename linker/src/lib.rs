use paho_mqtt::{ConnectOptions, Token, ServerResponse};

pub struct AsyncLinker {
    mqtt: paho_mqtt::AsyncClient,
}

#[derive(thiserror::Error, Debug)]
pub enum LinkerClientError {
    #[error("MQTT Error")]
    MQTT(paho_mqtt::Error),
}

type Result<T> = std::result::Result<T, LinkerClientError>;

/// Wrapper around `paho_mqtt::Client`
impl AsyncLinkerClient {
    pub fn new(create_opts: paho_mqtt::CreateOptions) -> Result<Self> {
        let client = paho_mqtt::AsyncClient::new(create_opts)
            .map_err(|err| LinkerClientError::MQTT(err))?;

        Ok(Self {
            mqtt: client,
        })
    }

    pub async fn connect(&self, connection_opts: ConnectOptions) -> Result<ServerResponse> {
        self.mqtt.connect(connection_opts).await
            .map_err(|err| LinkerClientError::MQTT(err))
    }

    pub async fn subscribe(&mut self, topic: String) -> Result<()> {
        self.mqtt.su
    }
}
