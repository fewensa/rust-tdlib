use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Points to a file
pub trait TDInputFile: Debug + RObject {}

/// Points to a file
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputFile {
    #[doc(hidden)]
    _Default(()),
    /// A file generated by the application
    Generated(InputFileGenerated),
    /// A file defined by its unique ID
    Id(InputFileId),
    /// A file defined by a local path
    Local(InputFileLocal),
    /// A file defined by its remote ID. The remote ID is guaranteed to be usable only if the corresponding file is still accessible to the user and known to TDLib. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
    Remote(InputFileRemote),
}

impl Default for InputFile {
    fn default() -> Self {
        InputFile::_Default(())
    }
}

impl<'de> Deserialize<'de> for InputFile {
    fn deserialize<D>(deserializer: D) -> Result<InputFile, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          InputFile,
          (inputFileGenerated, Generated);
          (inputFileId, Id);
          (inputFileLocal, Local);
          (inputFileRemote, Remote);

        )(deserializer)
    }
}

impl RObject for InputFile {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            InputFile::Generated(t) => t.td_name(),
            InputFile::Id(t) => t.td_name(),
            InputFile::Local(t) => t.td_name(),
            InputFile::Remote(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            InputFile::Generated(t) => t.extra(),
            InputFile::Id(t) => t.extra(),
            InputFile::Local(t) => t.extra(),
            InputFile::Remote(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl InputFile {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputFile::_Default(_))
    }
}

impl AsRef<InputFile> for InputFile {
    fn as_ref(&self) -> &InputFile {
        self
    }
}

/// A file generated by the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputFileGenerated {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Local path to a file from which the file is generated; may be empty if there is no such file
    original_path: String,
    /// String specifying the conversion applied to the original file; should be persistent across application restarts. Conversions beginning with '#' are reserved for internal TDLib usage
    conversion: String,
    /// Expected size of the generated file; 0 if unknown
    expected_size: i32,
}

impl RObject for InputFileGenerated {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputFileGenerated"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputFile for InputFileGenerated {}

impl InputFileGenerated {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputFileGeneratedBuilder {
        let mut inner = InputFileGenerated::default();
        inner.td_name = "inputFileGenerated".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputFileGeneratedBuilder { inner }
    }

    pub fn original_path(&self) -> &String {
        &self.original_path
    }

    pub fn conversion(&self) -> &String {
        &self.conversion
    }

    pub fn expected_size(&self) -> i32 {
        self.expected_size
    }
}

#[doc(hidden)]
pub struct RTDInputFileGeneratedBuilder {
    inner: InputFileGenerated,
}

impl RTDInputFileGeneratedBuilder {
    pub fn build(&self) -> InputFileGenerated {
        self.inner.clone()
    }

    pub fn original_path<T: AsRef<str>>(&mut self, original_path: T) -> &mut Self {
        self.inner.original_path = original_path.as_ref().to_string();
        self
    }

    pub fn conversion<T: AsRef<str>>(&mut self, conversion: T) -> &mut Self {
        self.inner.conversion = conversion.as_ref().to_string();
        self
    }

    pub fn expected_size(&mut self, expected_size: i32) -> &mut Self {
        self.inner.expected_size = expected_size;
        self
    }
}

impl AsRef<InputFileGenerated> for InputFileGenerated {
    fn as_ref(&self) -> &InputFileGenerated {
        self
    }
}

impl AsRef<InputFileGenerated> for RTDInputFileGeneratedBuilder {
    fn as_ref(&self) -> &InputFileGenerated {
        &self.inner
    }
}

/// A file defined by its unique ID
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputFileId {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique file identifier
    id: i32,
}

impl RObject for InputFileId {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputFileId"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputFile for InputFileId {}

impl InputFileId {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputFileIdBuilder {
        let mut inner = InputFileId::default();
        inner.td_name = "inputFileId".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputFileIdBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

#[doc(hidden)]
pub struct RTDInputFileIdBuilder {
    inner: InputFileId,
}

impl RTDInputFileIdBuilder {
    pub fn build(&self) -> InputFileId {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }
}

impl AsRef<InputFileId> for InputFileId {
    fn as_ref(&self) -> &InputFileId {
        self
    }
}

impl AsRef<InputFileId> for RTDInputFileIdBuilder {
    fn as_ref(&self) -> &InputFileId {
        &self.inner
    }
}

/// A file defined by a local path
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputFileLocal {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Local path to the file
    path: String,
}

impl RObject for InputFileLocal {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputFileLocal"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputFile for InputFileLocal {}

impl InputFileLocal {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputFileLocalBuilder {
        let mut inner = InputFileLocal::default();
        inner.td_name = "inputFileLocal".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputFileLocalBuilder { inner }
    }

    pub fn path(&self) -> &String {
        &self.path
    }
}

#[doc(hidden)]
pub struct RTDInputFileLocalBuilder {
    inner: InputFileLocal,
}

impl RTDInputFileLocalBuilder {
    pub fn build(&self) -> InputFileLocal {
        self.inner.clone()
    }

    pub fn path<T: AsRef<str>>(&mut self, path: T) -> &mut Self {
        self.inner.path = path.as_ref().to_string();
        self
    }
}

impl AsRef<InputFileLocal> for InputFileLocal {
    fn as_ref(&self) -> &InputFileLocal {
        self
    }
}

impl AsRef<InputFileLocal> for RTDInputFileLocalBuilder {
    fn as_ref(&self) -> &InputFileLocal {
        &self.inner
    }
}

/// A file defined by its remote ID. The remote ID is guaranteed to be usable only if the corresponding file is still accessible to the user and known to TDLib. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputFileRemote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Remote file identifier
    id: String,
}

impl RObject for InputFileRemote {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputFileRemote"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputFile for InputFileRemote {}

impl InputFileRemote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputFileRemoteBuilder {
        let mut inner = InputFileRemote::default();
        inner.td_name = "inputFileRemote".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputFileRemoteBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}

#[doc(hidden)]
pub struct RTDInputFileRemoteBuilder {
    inner: InputFileRemote,
}

impl RTDInputFileRemoteBuilder {
    pub fn build(&self) -> InputFileRemote {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }
}

impl AsRef<InputFileRemote> for InputFileRemote {
    fn as_ref(&self) -> &InputFileRemote {
        self
    }
}

impl AsRef<InputFileRemote> for RTDInputFileRemoteBuilder {
    fn as_ref(&self) -> &InputFileRemote {
        &self.inner
    }
}
