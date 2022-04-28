use actix::prelude::*;
use futures::executor::block_on;

use crate::client::MqttMsg;

use super::ReplyActor;
pub struct RecvActor {
    /// reply actor
    reply_addr: Addr<ReplyActor>,
}

impl RecvActor {
    pub fn new(reply_addr: Addr<ReplyActor>) -> Self {
        RecvActor {
            reply_addr,
        }
    }

    /// 解析mqtt的消息，并转换为执行指令
    async fn parse_msg(&mut self, message: String) {
        // 处理完业务逻辑，回复MQTT消息
        let topic = "/test/488ad2965e86/reply";
        let qos = 0;
                // 让actor来处理mqtt消息
        self.reply_addr.do_send(MqttMsg(topic.to_string(), "{\"code\": 200}".to_string(), qos));
    }
}

impl Actor for RecvActor {
    type Context = Context<Self>;
}

impl Handler<MqttMsg> for RecvActor {
    type Result = ();

    fn handle(&mut self, msg: MqttMsg, ctx: &mut Self::Context) -> Self::Result {
        // 在这里解析mqtt消息
        block_on(self.parse_msg(msg.1));
    }
}
