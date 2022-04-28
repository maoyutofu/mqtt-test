use std::{thread, sync::{Arc, Mutex}};

use actix::prelude::*;
use futures::executor::block_on;
use log::error;
use crate::client::MqttMsg;

use super::ReplyActor;

pub struct Process {
    /// reply actor
    reply_addr: Addr<ReplyActor>,
}

impl Process {
    pub fn new(reply_addr: Addr<ReplyActor>) -> Self {
        Process {
            reply_addr,
        }
    }

    /// 解析mqtt的消息，并转换为执行指令
    fn parse_msg(&self, message: String) {
        // 处理完业务逻辑，回复MQTT消息
        let topic = "/test/488ad2965e86/event";
        let qos = 1;
        // 让actor来处理mqtt消息
        block_on(async {
            match self.reply_addr.send(MqttMsg(topic.to_string(), "{\"code\": 200}".to_string(), qos)).await{
                Err(e) => error!("{}", e),
                _ => (),
            }
        });
    }
}
pub struct RecvActor {
    process: Arc<Mutex<Process>>,
}

impl RecvActor {
    pub fn new(process: Arc<Mutex<Process>>) -> Self {
        RecvActor {
            process,
        }
    }
}

impl Actor for RecvActor {
    type Context = Context<Self>;
}

impl Handler<MqttMsg> for RecvActor {
    type Result = ();

    fn handle(&mut self, msg: MqttMsg, ctx: &mut Self::Context) -> Self::Result {
        // 在这里解析mqtt消息
        // self.parse_msg(msg.1);

        let local_self = self.process.clone();
        // 假如我在这里使用thread
        thread::spawn(move ||{
            // info!("{}", msg.1);
            local_self.lock().unwrap().parse_msg(msg.1);
        });
    }
}
