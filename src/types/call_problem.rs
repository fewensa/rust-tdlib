use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the exact type of a problem with a call
pub trait TDCallProblem: Debug + RObject {}

/// Describes the exact type of a problem with a call
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum CallProblem {
    #[doc(hidden)]
    _Default(()),
    /// The speech was distorted
    DistortedSpeech(CallProblemDistortedSpeech),
    /// The call ended unexpectedly
    Dropped(CallProblemDropped),
    /// The user heard their own voice
    Echo(CallProblemEcho),
    /// The other side kept disappearing
    Interruptions(CallProblemInterruptions),
    /// The user heard background noise
    Noise(CallProblemNoise),
    /// The user couldn't hear the other side
    SilentLocal(CallProblemSilentLocal),
    /// The other side couldn't hear the user
    SilentRemote(CallProblemSilentRemote),
}

impl Default for CallProblem {
    fn default() -> Self {
        CallProblem::_Default(())
    }
}

impl<'de> Deserialize<'de> for CallProblem {
    fn deserialize<D>(deserializer: D) -> Result<CallProblem, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          CallProblem,
          (callProblemDistortedSpeech, DistortedSpeech);
          (callProblemDropped, Dropped);
          (callProblemEcho, Echo);
          (callProblemInterruptions, Interruptions);
          (callProblemNoise, Noise);
          (callProblemSilentLocal, SilentLocal);
          (callProblemSilentRemote, SilentRemote);

        )(deserializer)
    }
}

impl RObject for CallProblem {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            CallProblem::DistortedSpeech(t) => t.td_name(),
            CallProblem::Dropped(t) => t.td_name(),
            CallProblem::Echo(t) => t.td_name(),
            CallProblem::Interruptions(t) => t.td_name(),
            CallProblem::Noise(t) => t.td_name(),
            CallProblem::SilentLocal(t) => t.td_name(),
            CallProblem::SilentRemote(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            CallProblem::DistortedSpeech(t) => t.extra(),
            CallProblem::Dropped(t) => t.extra(),
            CallProblem::Echo(t) => t.extra(),
            CallProblem::Interruptions(t) => t.extra(),
            CallProblem::Noise(t) => t.extra(),
            CallProblem::SilentLocal(t) => t.extra(),
            CallProblem::SilentRemote(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl CallProblem {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CallProblem::_Default(_))
    }
}

impl AsRef<CallProblem> for CallProblem {
    fn as_ref(&self) -> &CallProblem {
        self
    }
}

/// The speech was distorted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemDistortedSpeech {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallProblemDistortedSpeech {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callProblemDistortedSpeech"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallProblem for CallProblemDistortedSpeech {}

impl CallProblemDistortedSpeech {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemDistortedSpeechBuilder {
        let mut inner = CallProblemDistortedSpeech::default();
        inner.td_name = "callProblemDistortedSpeech".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallProblemDistortedSpeechBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemDistortedSpeechBuilder {
    inner: CallProblemDistortedSpeech,
}

impl RTDCallProblemDistortedSpeechBuilder {
    pub fn build(&self) -> CallProblemDistortedSpeech {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDistortedSpeech> for CallProblemDistortedSpeech {
    fn as_ref(&self) -> &CallProblemDistortedSpeech {
        self
    }
}

impl AsRef<CallProblemDistortedSpeech> for RTDCallProblemDistortedSpeechBuilder {
    fn as_ref(&self) -> &CallProblemDistortedSpeech {
        &self.inner
    }
}

/// The call ended unexpectedly
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemDropped {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallProblemDropped {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callProblemDropped"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallProblem for CallProblemDropped {}

impl CallProblemDropped {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemDroppedBuilder {
        let mut inner = CallProblemDropped::default();
        inner.td_name = "callProblemDropped".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallProblemDroppedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemDroppedBuilder {
    inner: CallProblemDropped,
}

impl RTDCallProblemDroppedBuilder {
    pub fn build(&self) -> CallProblemDropped {
        self.inner.clone()
    }
}

impl AsRef<CallProblemDropped> for CallProblemDropped {
    fn as_ref(&self) -> &CallProblemDropped {
        self
    }
}

impl AsRef<CallProblemDropped> for RTDCallProblemDroppedBuilder {
    fn as_ref(&self) -> &CallProblemDropped {
        &self.inner
    }
}

/// The user heard their own voice
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemEcho {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallProblemEcho {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callProblemEcho"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallProblem for CallProblemEcho {}

impl CallProblemEcho {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemEchoBuilder {
        let mut inner = CallProblemEcho::default();
        inner.td_name = "callProblemEcho".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallProblemEchoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemEchoBuilder {
    inner: CallProblemEcho,
}

impl RTDCallProblemEchoBuilder {
    pub fn build(&self) -> CallProblemEcho {
        self.inner.clone()
    }
}

impl AsRef<CallProblemEcho> for CallProblemEcho {
    fn as_ref(&self) -> &CallProblemEcho {
        self
    }
}

impl AsRef<CallProblemEcho> for RTDCallProblemEchoBuilder {
    fn as_ref(&self) -> &CallProblemEcho {
        &self.inner
    }
}

/// The other side kept disappearing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemInterruptions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallProblemInterruptions {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callProblemInterruptions"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallProblem for CallProblemInterruptions {}

impl CallProblemInterruptions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemInterruptionsBuilder {
        let mut inner = CallProblemInterruptions::default();
        inner.td_name = "callProblemInterruptions".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallProblemInterruptionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemInterruptionsBuilder {
    inner: CallProblemInterruptions,
}

impl RTDCallProblemInterruptionsBuilder {
    pub fn build(&self) -> CallProblemInterruptions {
        self.inner.clone()
    }
}

impl AsRef<CallProblemInterruptions> for CallProblemInterruptions {
    fn as_ref(&self) -> &CallProblemInterruptions {
        self
    }
}

impl AsRef<CallProblemInterruptions> for RTDCallProblemInterruptionsBuilder {
    fn as_ref(&self) -> &CallProblemInterruptions {
        &self.inner
    }
}

/// The user heard background noise
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemNoise {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallProblemNoise {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callProblemNoise"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallProblem for CallProblemNoise {}

impl CallProblemNoise {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemNoiseBuilder {
        let mut inner = CallProblemNoise::default();
        inner.td_name = "callProblemNoise".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallProblemNoiseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemNoiseBuilder {
    inner: CallProblemNoise,
}

impl RTDCallProblemNoiseBuilder {
    pub fn build(&self) -> CallProblemNoise {
        self.inner.clone()
    }
}

impl AsRef<CallProblemNoise> for CallProblemNoise {
    fn as_ref(&self) -> &CallProblemNoise {
        self
    }
}

impl AsRef<CallProblemNoise> for RTDCallProblemNoiseBuilder {
    fn as_ref(&self) -> &CallProblemNoise {
        &self.inner
    }
}

/// The user couldn't hear the other side
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemSilentLocal {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallProblemSilentLocal {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callProblemSilentLocal"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallProblem for CallProblemSilentLocal {}

impl CallProblemSilentLocal {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemSilentLocalBuilder {
        let mut inner = CallProblemSilentLocal::default();
        inner.td_name = "callProblemSilentLocal".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallProblemSilentLocalBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemSilentLocalBuilder {
    inner: CallProblemSilentLocal,
}

impl RTDCallProblemSilentLocalBuilder {
    pub fn build(&self) -> CallProblemSilentLocal {
        self.inner.clone()
    }
}

impl AsRef<CallProblemSilentLocal> for CallProblemSilentLocal {
    fn as_ref(&self) -> &CallProblemSilentLocal {
        self
    }
}

impl AsRef<CallProblemSilentLocal> for RTDCallProblemSilentLocalBuilder {
    fn as_ref(&self) -> &CallProblemSilentLocal {
        &self.inner
    }
}

/// The other side couldn't hear the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProblemSilentRemote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for CallProblemSilentRemote {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "callProblemSilentRemote"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDCallProblem for CallProblemSilentRemote {}

impl CallProblemSilentRemote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProblemSilentRemoteBuilder {
        let mut inner = CallProblemSilentRemote::default();
        inner.td_name = "callProblemSilentRemote".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDCallProblemSilentRemoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDCallProblemSilentRemoteBuilder {
    inner: CallProblemSilentRemote,
}

impl RTDCallProblemSilentRemoteBuilder {
    pub fn build(&self) -> CallProblemSilentRemote {
        self.inner.clone()
    }
}

impl AsRef<CallProblemSilentRemote> for CallProblemSilentRemote {
    fn as_ref(&self) -> &CallProblemSilentRemote {
        self
    }
}

impl AsRef<CallProblemSilentRemote> for RTDCallProblemSilentRemoteBuilder {
    fn as_ref(&self) -> &CallProblemSilentRemote {
        &self.inner
    }
}
