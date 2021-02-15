use crate::types::TdType;
use futures::channel::oneshot;
use std::collections::HashMap;
#[doc(hidden)]
use std::sync::RwLock;

lazy_static! {
    pub(super) static ref OBSERVER: Observer = Observer::new();
}

pub(super) struct Observer {
    channels: RwLock<HashMap<String, oneshot::Sender<TdType>>>,
}

impl Observer {
    fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
        }
    }

    pub fn notify(&self, payload: TdType) -> Option<TdType> {
        match payload.extra() {
            None => {
                trace!("no extra for payload {:?}", payload);
                Some(payload)
            }
            Some(extra) => {
                let mut map = self.channels.write().unwrap();
                match map.remove(extra) {
                    None => {
                        trace!("no subscribers for {}", extra);
                        Some(payload)
                    }
                    Some(sender) => {
                        trace!("signal send for {}", extra);
                        if let Err(t) = sender.send(payload) {
                            warn!("request already closed, received update: {:?}", t)
                        };
                        None
                    }
                }
            }
        }
    }

    pub fn subscribe(&self, extra: &str) -> oneshot::Receiver<TdType> {
        let (sender, receiver) = oneshot::channel::<TdType>();
        match self.channels.write() {
            Ok(mut map) => {
                map.insert(extra.to_string(), sender);
                trace!("subscribed for {}", extra);
            }
            _ => {
                warn!("can't acquire lock for notifier map");
            }
        };
        receiver
    }

    pub fn unsubscribe(&self, extra: &str) {
        if let Ok(mut map) = self.channels.write() {
            trace!("remove {} subscription", &extra);
            map.remove(extra);
        };
    }
}
