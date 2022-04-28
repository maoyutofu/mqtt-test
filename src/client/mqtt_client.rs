use crate::actor::RecvActor;
use actix::Addr;
use log::{info, warn, error};
use paho_mqtt as mqtt;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::client::MqttMsg;

type Result<T> = std::result::Result<T, mqtt::Error>;

pub struct MqttClient {
    cli: mqtt::Client,
    rx: mqtt::Receiver<Option<mqtt::Message>>,
}

impl MqttClient {
    pub async fn new() -> Result<MqttClient> {
        let opts = mqtt::CreateOptionsBuilder::new()
            .server_uri("tcp://broker-cn.emqx.io:1883")
            .client_id("488AD2965E86")
            .persistence(None)
            .finalize();

        let mut cli = mqtt::Client::new(opts)?;

        let conn_opts = mqtt::connect_options::ConnectOptionsBuilder::new()
            .clean_session(true)
            .user_name("488AD2965E86")
            .password("12345678")
            .keep_alive_interval(Duration::from_secs(20))
            .finalize();

        cli.connect(conn_opts)?;
        info!("[mqtt]Successfully connected");

        let rx = cli.start_consuming();

        Ok(MqttClient { cli, rx })
    }

    pub async fn publish(&self, topic: &str, msg: &str, qos: i32) -> Result<()> {
        info!("[pub]topic: {}, qos: {}, payload: {}", topic, qos, msg);
        let msg = mqtt::Message::new(topic, msg, qos);
        self.cli.publish(msg)?;
        Ok(())
    }

    pub async fn subscribe(&self, topic: &str, qos: i32, recv_addr: Arc<Addr<RecvActor>>) -> Result<()> {
        self.cli.subscribe(topic, qos)?;
        info!("[subscribe]topic: {}, qos: {}", topic, qos);

        for msg in self.rx.iter() {
            if let Some(msg) = msg {
                let payload = String::from_utf8_lossy(msg.payload()).to_string();
                info!(
                    "[recv]topic: {}, qos: {}, payload: {}",
                    msg.topic(),
                    msg.qos(),
                    payload
                );
                // 让actor来处理mqtt消息
                match recv_addr.send(MqttMsg(msg.topic().to_string(), payload, msg.qos())).await {
                    Err(e) => error!("{}", e),
                    _ => (),
                }
            } else if self.cli.is_connected() || !self.try_reconnect(topic, qos).await? {
                break;
            }
        }

        if self.cli.is_connected() {
            info!("[mqtt]Disconnected");
            self.cli.unsubscribe(topic)?;
            self.cli.disconnect(None)?;
        }

        Ok(())
    }

    async fn try_reconnect(&self, topic: &str, qos: i32) -> Result<bool> {
        warn!("[mqtt]Connection lost. Waiting to retry connection");
        let mut reconnect_counter = 0;
        loop {
            reconnect_counter += 1;
            thread::sleep(Duration::from_millis(reconnect_counter * 1000));
            if self.cli.reconnect().is_ok() {
                info!("[mqtt]Successfully reconnected");
                info!("[subscribe]topic: {}, qos: {}", topic, qos);
                self.cli.subscribe(topic, qos)?;
                return Ok(true);
            }
            if reconnect_counter == 60 {
                reconnect_counter = 0;
            }
        }
    }
}
