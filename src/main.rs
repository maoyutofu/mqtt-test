mod actor;
mod client;

use std::sync::Arc;

use actor::*;

use actix::prelude::*;
use env_logger::Builder;

#[actix::main]
async fn main() -> std::io::Result<()> {
    let mut builder = Builder::new();
    builder.parse_filters("mqtt_test=debug");

    builder.init();


    if let Ok(client) = client::MqttClient::new().await {
        let client_arc = Arc::new(client);

        let reply_actor = ReplyActor::new(client_arc.clone());
        let reply_addr = reply_actor.start();

        let recv_actor = RecvActor::new(reply_addr);
        let recv_actor_addr = recv_actor.start();
        let recv_actor_acr = Arc::new(recv_actor_addr);

        let qos = 0;
        let topic = "/test/488ad2965e86/request";
        client_arc
            .clone()
            .subscribe(&topic, qos, recv_actor_acr.clone())
            .await
            .unwrap();
    }

    Ok(())
}
