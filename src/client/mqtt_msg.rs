use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "")]
pub struct MqttMsg(pub String, pub String, pub i32);