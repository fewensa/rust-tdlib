use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Specifies the kind of chat members to return in getSupergroupMembers
pub trait TDSupergroupMembersFilter: Debug + RObject {}

/// Specifies the kind of chat members to return in getSupergroupMembers
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SupergroupMembersFilter {
    #[doc(hidden)]
    _Default(()),
    /// Returns the owner and administrators
    Administrators(SupergroupMembersFilterAdministrators),
    /// Returns users banned from the supergroup or channel; can be used only by administrators
    Banned(SupergroupMembersFilterBanned),
    /// Returns bot members of the supergroup or channel
    Bots(SupergroupMembersFilterBots),
    /// Returns contacts of the user, which are members of the supergroup or channel
    Contacts(SupergroupMembersFilterContacts),
    /// Returns recently active users in reverse chronological order
    Recent(SupergroupMembersFilterRecent),
    /// Returns restricted supergroup members; can be used only by administrators
    Restricted(SupergroupMembersFilterRestricted),
    /// Used to search for supergroup or channel members via a (string) query
    Search(SupergroupMembersFilterSearch),
}

impl Default for SupergroupMembersFilter {
    fn default() -> Self {
        SupergroupMembersFilter::_Default(())
    }
}

impl<'de> Deserialize<'de> for SupergroupMembersFilter {
    fn deserialize<D>(deserializer: D) -> Result<SupergroupMembersFilter, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          SupergroupMembersFilter,
          (supergroupMembersFilterAdministrators, Administrators);
          (supergroupMembersFilterBanned, Banned);
          (supergroupMembersFilterBots, Bots);
          (supergroupMembersFilterContacts, Contacts);
          (supergroupMembersFilterRecent, Recent);
          (supergroupMembersFilterRestricted, Restricted);
          (supergroupMembersFilterSearch, Search);

        )(deserializer)
    }
}

impl RObject for SupergroupMembersFilter {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            SupergroupMembersFilter::Administrators(t) => t.td_name(),
            SupergroupMembersFilter::Banned(t) => t.td_name(),
            SupergroupMembersFilter::Bots(t) => t.td_name(),
            SupergroupMembersFilter::Contacts(t) => t.td_name(),
            SupergroupMembersFilter::Recent(t) => t.td_name(),
            SupergroupMembersFilter::Restricted(t) => t.td_name(),
            SupergroupMembersFilter::Search(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            SupergroupMembersFilter::Administrators(t) => t.extra(),
            SupergroupMembersFilter::Banned(t) => t.extra(),
            SupergroupMembersFilter::Bots(t) => t.extra(),
            SupergroupMembersFilter::Contacts(t) => t.extra(),
            SupergroupMembersFilter::Recent(t) => t.extra(),
            SupergroupMembersFilter::Restricted(t) => t.extra(),
            SupergroupMembersFilter::Search(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl SupergroupMembersFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SupergroupMembersFilter::_Default(_))
    }
}

impl AsRef<SupergroupMembersFilter> for SupergroupMembersFilter {
    fn as_ref(&self) -> &SupergroupMembersFilter {
        self
    }
}

/// Returns the owner and administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterAdministrators {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for SupergroupMembersFilterAdministrators {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "supergroupMembersFilterAdministrators"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterAdministrators {}

impl SupergroupMembersFilterAdministrators {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterAdministratorsBuilder {
        let mut inner = SupergroupMembersFilterAdministrators::default();
        inner.td_name = "supergroupMembersFilterAdministrators".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSupergroupMembersFilterAdministratorsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterAdministratorsBuilder {
    inner: SupergroupMembersFilterAdministrators,
}

impl RTDSupergroupMembersFilterAdministratorsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterAdministrators {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterAdministrators> for SupergroupMembersFilterAdministrators {
    fn as_ref(&self) -> &SupergroupMembersFilterAdministrators {
        self
    }
}

impl AsRef<SupergroupMembersFilterAdministrators>
    for RTDSupergroupMembersFilterAdministratorsBuilder
{
    fn as_ref(&self) -> &SupergroupMembersFilterAdministrators {
        &self.inner
    }
}

/// Returns users banned from the supergroup or channel; can be used only by administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterBanned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Query to search for
    query: String,
}

impl RObject for SupergroupMembersFilterBanned {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "supergroupMembersFilterBanned"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterBanned {}

impl SupergroupMembersFilterBanned {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterBannedBuilder {
        let mut inner = SupergroupMembersFilterBanned::default();
        inner.td_name = "supergroupMembersFilterBanned".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSupergroupMembersFilterBannedBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterBannedBuilder {
    inner: SupergroupMembersFilterBanned,
}

impl RTDSupergroupMembersFilterBannedBuilder {
    pub fn build(&self) -> SupergroupMembersFilterBanned {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterBanned> for SupergroupMembersFilterBanned {
    fn as_ref(&self) -> &SupergroupMembersFilterBanned {
        self
    }
}

impl AsRef<SupergroupMembersFilterBanned> for RTDSupergroupMembersFilterBannedBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterBanned {
        &self.inner
    }
}

/// Returns bot members of the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterBots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for SupergroupMembersFilterBots {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "supergroupMembersFilterBots"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterBots {}

impl SupergroupMembersFilterBots {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterBotsBuilder {
        let mut inner = SupergroupMembersFilterBots::default();
        inner.td_name = "supergroupMembersFilterBots".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSupergroupMembersFilterBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterBotsBuilder {
    inner: SupergroupMembersFilterBots,
}

impl RTDSupergroupMembersFilterBotsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterBots {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterBots> for SupergroupMembersFilterBots {
    fn as_ref(&self) -> &SupergroupMembersFilterBots {
        self
    }
}

impl AsRef<SupergroupMembersFilterBots> for RTDSupergroupMembersFilterBotsBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterBots {
        &self.inner
    }
}

/// Returns contacts of the user, which are members of the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Query to search for
    query: String,
}

impl RObject for SupergroupMembersFilterContacts {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "supergroupMembersFilterContacts"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterContacts {}

impl SupergroupMembersFilterContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterContactsBuilder {
        let mut inner = SupergroupMembersFilterContacts::default();
        inner.td_name = "supergroupMembersFilterContacts".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSupergroupMembersFilterContactsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterContactsBuilder {
    inner: SupergroupMembersFilterContacts,
}

impl RTDSupergroupMembersFilterContactsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterContacts {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterContacts> for SupergroupMembersFilterContacts {
    fn as_ref(&self) -> &SupergroupMembersFilterContacts {
        self
    }
}

impl AsRef<SupergroupMembersFilterContacts> for RTDSupergroupMembersFilterContactsBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterContacts {
        &self.inner
    }
}

/// Returns recently active users in reverse chronological order
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterRecent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for SupergroupMembersFilterRecent {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "supergroupMembersFilterRecent"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterRecent {}

impl SupergroupMembersFilterRecent {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterRecentBuilder {
        let mut inner = SupergroupMembersFilterRecent::default();
        inner.td_name = "supergroupMembersFilterRecent".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSupergroupMembersFilterRecentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterRecentBuilder {
    inner: SupergroupMembersFilterRecent,
}

impl RTDSupergroupMembersFilterRecentBuilder {
    pub fn build(&self) -> SupergroupMembersFilterRecent {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterRecent> for SupergroupMembersFilterRecent {
    fn as_ref(&self) -> &SupergroupMembersFilterRecent {
        self
    }
}

impl AsRef<SupergroupMembersFilterRecent> for RTDSupergroupMembersFilterRecentBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterRecent {
        &self.inner
    }
}

/// Returns restricted supergroup members; can be used only by administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterRestricted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Query to search for
    query: String,
}

impl RObject for SupergroupMembersFilterRestricted {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "supergroupMembersFilterRestricted"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterRestricted {}

impl SupergroupMembersFilterRestricted {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterRestrictedBuilder {
        let mut inner = SupergroupMembersFilterRestricted::default();
        inner.td_name = "supergroupMembersFilterRestricted".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSupergroupMembersFilterRestrictedBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterRestrictedBuilder {
    inner: SupergroupMembersFilterRestricted,
}

impl RTDSupergroupMembersFilterRestrictedBuilder {
    pub fn build(&self) -> SupergroupMembersFilterRestricted {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterRestricted> for SupergroupMembersFilterRestricted {
    fn as_ref(&self) -> &SupergroupMembersFilterRestricted {
        self
    }
}

impl AsRef<SupergroupMembersFilterRestricted> for RTDSupergroupMembersFilterRestrictedBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterRestricted {
        &self.inner
    }
}

/// Used to search for supergroup or channel members via a (string) query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterSearch {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Query to search for
    query: String,
}

impl RObject for SupergroupMembersFilterSearch {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "supergroupMembersFilterSearch"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterSearch {}

impl SupergroupMembersFilterSearch {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterSearchBuilder {
        let mut inner = SupergroupMembersFilterSearch::default();
        inner.td_name = "supergroupMembersFilterSearch".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDSupergroupMembersFilterSearchBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterSearchBuilder {
    inner: SupergroupMembersFilterSearch,
}

impl RTDSupergroupMembersFilterSearchBuilder {
    pub fn build(&self) -> SupergroupMembersFilterSearch {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterSearch> for SupergroupMembersFilterSearch {
    fn as_ref(&self) -> &SupergroupMembersFilterSearch {
        self
    }
}

impl AsRef<SupergroupMembersFilterSearch> for RTDSupergroupMembersFilterSearchBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterSearch {
        &self.inner
    }
}
