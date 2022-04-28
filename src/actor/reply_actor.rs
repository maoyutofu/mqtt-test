use std::sync::Arc;

use actix::prelude::*;
use futures::executor::block_on;
use crate::client::{MqttClient, MqttMsg};

pub struct ReplyActor {
    /// mqtt client
    client: Arc<MqttClient>,
}

impl ReplyActor {
    pub fn new(client: Arc<MqttClient>) -> Self {
        ReplyActor {
            client,
        }
    }
}

impl Actor for ReplyActor {
    type Context = Context<Self>;
}

impl Handler<MqttMsg> for ReplyActor {
    type Result = ();

    fn handle(&mut self, msg: MqttMsg, ctx: &mut Self::Context) -> Self::Result {
        // 在这里解析mqtt消息
        block_on(self.client.publish(&msg.0, &msg.1, msg.2));
    }
}
