use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of a call server
pub trait TDCallServerType: Debug + RObject {}

/// Describes the type of a call server
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallServerType {
    #[doc(hidden)]
    _Default(()),
    /// A Telegram call reflector
    TelegramReflector(CallServerTypeTelegramReflector),
    /// A WebRTC server
    Webrtc(CallServerTypeWebrtc),
}

impl Default for CallServerType {
    fn default() -> Self {
        CallServerType::_Default(())
    }
}

impl<'de> Deserialize<'de> for CallServerType {
    fn deserialize<D>(deserializer: D) -> Result<CallServerType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          CallServerType,
          (callServerTypeTelegramReflector, TelegramReflector);
          (callServerTypeWebrtc, Webrtc);

        )(deserializer)
    }
}

impl RObject for CallServerType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            CallServerType::TelegramReflector(t) => t.td_name(),
            CallServerType::Webrtc(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            CallServerType::TelegramReflector(t) => t.extra(),
            CallServerType::Webrtc(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CallServerType::TelegramReflector(t) => t.client_id(),
            CallServerType::Webrtc(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CallServerType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallServerType::_Default(_))
    }
}

impl AsRef<CallServerType> for CallServerType {
    fn as_ref(&self) -> &CallServerType {
        self
    }
}

/// A Telegram call reflector
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServerTypeTelegramReflector {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A peer tag to be used with the reflector
    peer_tag: String,
}

impl RObject for CallServerTypeTelegramReflector {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callServerTypeTelegramReflector"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallServerType for CallServerTypeTelegramReflector {}

impl CallServerTypeTelegramReflector {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallServerTypeTelegramReflectorBuilder {
        let mut inner = CallServerTypeTelegramReflector::default();
        inner.td_name = "callServerTypeTelegramReflector".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDCallServerTypeTelegramReflectorBuilder { inner }
    }

    pub fn peer_tag(&self) -> &String {
        &self.peer_tag
    }
}

#[doc(hidden)]
pub struct RTDCallServerTypeTelegramReflectorBuilder {
    inner: CallServerTypeTelegramReflector,
}

impl RTDCallServerTypeTelegramReflectorBuilder {
    pub fn build(&self) -> CallServerTypeTelegramReflector {
        self.inner.clone()
    }

    pub fn peer_tag<T: AsRef<str>>(&mut self, peer_tag: T) -> &mut Self {
        self.inner.peer_tag = peer_tag.as_ref().to_string();
        self
    }
}

impl AsRef<CallServerTypeTelegramReflector> for CallServerTypeTelegramReflector {
    fn as_ref(&self) -> &CallServerTypeTelegramReflector {
        self
    }
}

impl AsRef<CallServerTypeTelegramReflector> for RTDCallServerTypeTelegramReflectorBuilder {
    fn as_ref(&self) -> &CallServerTypeTelegramReflector {
        &self.inner
    }
}

/// A WebRTC server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServerTypeWebrtc {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Username to be used for authentication
    username: String,
    /// Authentication password
    password: String,
    /// True, if the server supports TURN
    supports_turn: bool,
    /// True, if the server supports STUN
    supports_stun: bool,
}

impl RObject for CallServerTypeWebrtc {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callServerTypeWebrtc"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallServerType for CallServerTypeWebrtc {}

impl CallServerTypeWebrtc {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallServerTypeWebrtcBuilder {
        let mut inner = CallServerTypeWebrtc::default();
        inner.td_name = "callServerTypeWebrtc".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDCallServerTypeWebrtcBuilder { inner }
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &String {
        &self.password
    }

    pub fn supports_turn(&self) -> bool {
        self.supports_turn
    }

    pub fn supports_stun(&self) -> bool {
        self.supports_stun
    }
}

#[doc(hidden)]
pub struct RTDCallServerTypeWebrtcBuilder {
    inner: CallServerTypeWebrtc,
}

impl RTDCallServerTypeWebrtcBuilder {
    pub fn build(&self) -> CallServerTypeWebrtc {
        self.inner.clone()
    }

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
        self
    }

    pub fn password<T: AsRef<str>>(&mut self, password: T) -> &mut Self {
        self.inner.password = password.as_ref().to_string();
        self
    }

    pub fn supports_turn(&mut self, supports_turn: bool) -> &mut Self {
        self.inner.supports_turn = supports_turn;
        self
    }

    pub fn supports_stun(&mut self, supports_stun: bool) -> &mut Self {
        self.inner.supports_stun = supports_stun;
        self
    }
}

impl AsRef<CallServerTypeWebrtc> for CallServerTypeWebrtc {
    fn as_ref(&self) -> &CallServerTypeWebrtc {
        self
    }
}

impl AsRef<CallServerTypeWebrtc> for RTDCallServerTypeWebrtcBuilder {
    fn as_ref(&self) -> &CallServerTypeWebrtc {
        &self.inner
    }
}