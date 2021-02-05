use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a reason why an external chat is shown in a chat list
pub trait TDChatSource: Debug + RObject {}

/// Describes a reason why an external chat is shown in a chat list
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatSource {
    #[doc(hidden)]
    _Default(()),
    /// The chat is sponsored by the user's MTProxy server
    MtprotoProxy(ChatSourceMtprotoProxy),
    /// The chat contains a public service announcement
    PublicServiceAnnouncement(ChatSourcePublicServiceAnnouncement),
}

impl Default for ChatSource {
    fn default() -> Self {
        ChatSource::_Default(())
    }
}

impl<'de> Deserialize<'de> for ChatSource {
    fn deserialize<D>(deserializer: D) -> Result<ChatSource, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          ChatSource,
          (chatSourceMtprotoProxy, MtprotoProxy);
          (chatSourcePublicServiceAnnouncement, PublicServiceAnnouncement);

        )(deserializer)
    }
}

impl RObject for ChatSource {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            ChatSource::MtprotoProxy(t) => t.td_name(),
            ChatSource::PublicServiceAnnouncement(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            ChatSource::MtprotoProxy(t) => t.extra(),
            ChatSource::PublicServiceAnnouncement(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatSource::MtprotoProxy(t) => t.client_id(),
            ChatSource::PublicServiceAnnouncement(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatSource::_Default(_))
    }
}

impl AsRef<ChatSource> for ChatSource {
    fn as_ref(&self) -> &ChatSource {
        self
    }
}

/// The chat is sponsored by the user's MTProxy server
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatSourceMtprotoProxy {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatSourceMtprotoProxy {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatSourceMtprotoProxy"
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

impl TDChatSource for ChatSourceMtprotoProxy {}

impl ChatSourceMtprotoProxy {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatSourceMtprotoProxyBuilder {
        let mut inner = ChatSourceMtprotoProxy::default();
        inner.td_name = "chatSourceMtprotoProxy".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDChatSourceMtprotoProxyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatSourceMtprotoProxyBuilder {
    inner: ChatSourceMtprotoProxy,
}

impl RTDChatSourceMtprotoProxyBuilder {
    pub fn build(&self) -> ChatSourceMtprotoProxy {
        self.inner.clone()
    }
}

impl AsRef<ChatSourceMtprotoProxy> for ChatSourceMtprotoProxy {
    fn as_ref(&self) -> &ChatSourceMtprotoProxy {
        self
    }
}

impl AsRef<ChatSourceMtprotoProxy> for RTDChatSourceMtprotoProxyBuilder {
    fn as_ref(&self) -> &ChatSourceMtprotoProxy {
        &self.inner
    }
}

/// The chat contains a public service announcement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatSourcePublicServiceAnnouncement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The type of the announcement
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_: String,
    /// The text of the announcement
    text: String,
}

impl RObject for ChatSourcePublicServiceAnnouncement {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "chatSourcePublicServiceAnnouncement"
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

impl TDChatSource for ChatSourcePublicServiceAnnouncement {}

impl ChatSourcePublicServiceAnnouncement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatSourcePublicServiceAnnouncementBuilder {
        let mut inner = ChatSourcePublicServiceAnnouncement::default();
        inner.td_name = "chatSourcePublicServiceAnnouncement".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDChatSourcePublicServiceAnnouncementBuilder { inner }
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDChatSourcePublicServiceAnnouncementBuilder {
    inner: ChatSourcePublicServiceAnnouncement,
}

impl RTDChatSourcePublicServiceAnnouncementBuilder {
    pub fn build(&self) -> ChatSourcePublicServiceAnnouncement {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().to_string();
        self
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<ChatSourcePublicServiceAnnouncement> for ChatSourcePublicServiceAnnouncement {
    fn as_ref(&self) -> &ChatSourcePublicServiceAnnouncement {
        self
    }
}

impl AsRef<ChatSourcePublicServiceAnnouncement> for RTDChatSourcePublicServiceAnnouncementBuilder {
    fn as_ref(&self) -> &ChatSourcePublicServiceAnnouncement {
        &self.inner
    }
}