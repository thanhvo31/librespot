use super::SpircManager;
use protocol;
use protocol::spirc::{MessageType, State};
use protobuf::RepeatedField;
use std::iter::FromIterator;

#[must_use]
pub struct CommandSender<'a> {
    manager: &'a mut SpircManager,
    cmd: MessageType,
    recipient: Option<String>,
    state: Option<State>,
}

impl<'a> CommandSender<'a> {
    pub fn new(manager: &'a mut SpircManager, cmd: MessageType) -> CommandSender {
        CommandSender {
            manager: manager,
            cmd: cmd,
            recipient: None,
            state: None,
        }
    }

    pub fn recipient<T>(mut self, r: T) -> CommandSender<'a>
        where T: Into<Option<String>>
    {
        self.recipient = r.into();
        self
    }

    pub fn state(mut self, s: State) -> CommandSender<'a> {
        self.state = Some(s);
        self
    }

    pub fn send(self) {
        let manager = self.manager;
        let state = self.state.unwrap_or_else(|| {
            manager.player_state()
        });

        let frame = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            ident: manager.ident.clone(),
            protocol_version: "2.0.0",
            seq_nr: manager.next_seq(),
            typ: self.cmd,
            recipient: RepeatedField::from_iter(self.recipient),
            device_state: manager.device_state(),
            state_update_id: 0,
            state: state,
        });

        manager.send_frame(frame);
    }
}
