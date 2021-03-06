use std::sync::Arc;

use crate::{
    client::observer::OBSERVER,
    errors::{RTDError, RTDResult},
    types::RFunction,
    types::*,
};
use rtdlib_sys::Tdlib;

#[doc(hidden)]
pub trait TdLibClient {
    fn send<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<()>;
    fn receive(&self, timeout: f64) -> Option<String>;
    fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>>;
}

#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct RawApi {
    tdlib: Arc<Tdlib>,
}

impl Default for RawApi {
    fn default() -> Self {
        Self {
            tdlib: Arc::new(Tdlib::new()),
        }
    }
}

impl TdLibClient for RawApi {
    fn send<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<()> {
        let json = fnc.to_json()?;
        self.tdlib.send(&json[..]);
        Ok(())
    }

    fn receive(&self, timeout: f64) -> Option<String> {
        self.tdlib.receive(timeout)
    }

    fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>> {
        let json = fnc.to_json()?;
        Ok(self.tdlib.execute(&json[..]))
    }
}

impl RawApi {
    pub fn new(tdlib: Tdlib) -> Self {
        Self {
            tdlib: Arc::new(tdlib),
        }
    }
}

#[derive(Clone, Debug)]
/// Struct stores all methods which you can call to interact with Telegram, such as:
/// [send_message](Api::send_message), [download_file](Api::download_file), [search_chats](Api::search_chats) and so on.
pub struct Api<S>
where
    S: TdLibClient + Clone,
{
    raw_api: S,
}

impl Default for Api<RawApi> {
    fn default() -> Self {
        Self {
            raw_api: RawApi::default(),
        }
    }
}
/// TDLib high-level API methods.
/// Methods documentation can be found in https://core.telegram.org/tdlib/docs/td__api_8h.html
impl<S> Api<S>
where
    S: TdLibClient + Clone,
{
    pub fn new(raw_api: S) -> Self {
        Self { raw_api }
    }

    pub fn raw_api(&self) -> &S {
        &self.raw_api
    }

    // Accepts an incoming call
    pub async fn accept_call<C: AsRef<AcceptCall>>(&self, accept_call: C) -> RTDResult<Ok> {
        let extra = accept_call.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(accept_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Accepts Telegram terms of services
    pub async fn accept_terms_of_service<C: AsRef<AcceptTermsOfService>>(
        &self,
        accept_terms_of_service: C,
    ) -> RTDResult<Ok> {
        let extra = accept_terms_of_service
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(accept_terms_of_service.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a new member to a chat. Members can't be added to private or secret chats. Members will not be added until the chat state has been synchronized with the server
    pub async fn add_chat_member<C: AsRef<AddChatMember>>(
        &self,
        add_chat_member: C,
    ) -> RTDResult<Ok> {
        let extra = add_chat_member.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_chat_member.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds multiple new members to a chat. Currently this option is only available for supergroups and channels. This option can't be used to join a chat. Members can't be added to a channel if it has more than 200 members. Members will not be added until the chat state has been synchronized with the server
    pub async fn add_chat_members<C: AsRef<AddChatMembers>>(
        &self,
        add_chat_members: C,
    ) -> RTDResult<Ok> {
        let extra = add_chat_members.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_chat_members.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a user to the contact list or edits an existing contact by their user identifier
    pub async fn add_contact<C: AsRef<AddContact>>(&self, add_contact: C) -> RTDResult<Ok> {
        let extra = add_contact.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_contact.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization
    pub async fn add_custom_server_language_pack<C: AsRef<AddCustomServerLanguagePack>>(
        &self,
        add_custom_server_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = add_custom_server_language_pack
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(add_custom_server_language_pack.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a new sticker to the list of favorite stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
    pub async fn add_favorite_sticker<C: AsRef<AddFavoriteSticker>>(
        &self,
        add_favorite_sticker: C,
    ) -> RTDResult<Ok> {
        let extra = add_favorite_sticker
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_favorite_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a local message to a chat. The message is persistent across application restarts only if the message database is used. Returns the added message
    pub async fn add_local_message<C: AsRef<AddLocalMessage>>(
        &self,
        add_local_message: C,
    ) -> RTDResult<Message> {
        let extra = add_local_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_local_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a message to TDLib internal log. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn add_log_message<C: AsRef<AddLogMessage>>(
        &self,
        add_log_message: C,
    ) -> RTDResult<Ok> {
        let extra = add_log_message.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_log_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds the specified data to data usage statistics. Can be called before authorization
    pub async fn add_network_statistics<C: AsRef<AddNetworkStatistics>>(
        &self,
        add_network_statistics: C,
    ) -> RTDResult<Ok> {
        let extra = add_network_statistics
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_network_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a proxy server for network requests. Can be called before authorization
    pub async fn add_proxy<C: AsRef<AddProxy>>(&self, add_proxy: C) -> RTDResult<Proxy> {
        let extra = add_proxy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Proxy(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Manually adds a new sticker to the list of recently used stickers. The new sticker is added to the top of the list. If the sticker was already in the list, it is removed from the list first. Only stickers belonging to a sticker set can be added to this list
    pub async fn add_recent_sticker<C: AsRef<AddRecentSticker>>(
        &self,
        add_recent_sticker: C,
    ) -> RTDResult<Stickers> {
        let extra = add_recent_sticker
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_recent_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Stickers(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a chat to the list of recently found chats. The chat is added to the beginning of the list. If the chat is already in the list, it will be removed from the list first
    pub async fn add_recently_found_chat<C: AsRef<AddRecentlyFoundChat>>(
        &self,
        add_recently_found_chat: C,
    ) -> RTDResult<Ok> {
        let extra = add_recently_found_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_recently_found_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Manually adds a new animation to the list of saved animations. The new animation is added to the beginning of the list. If the animation was already in the list, it is removed first. Only non-secret video animations with MIME type "video/mp4" can be added to the list
    pub async fn add_saved_animation<C: AsRef<AddSavedAnimation>>(
        &self,
        add_saved_animation: C,
    ) -> RTDResult<Ok> {
        let extra = add_saved_animation
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_saved_animation.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a new sticker to a set; for bots only. Returns the sticker set
    pub async fn add_sticker_to_set<C: AsRef<AddStickerToSet>>(
        &self,
        add_sticker_to_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = add_sticker_to_set
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(add_sticker_to_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSet(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the result of a callback query; for bots only
    pub async fn answer_callback_query<C: AsRef<AnswerCallbackQuery>>(
        &self,
        answer_callback_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_callback_query
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(answer_callback_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Answers a custom query; for bots only
    pub async fn answer_custom_query<C: AsRef<AnswerCustomQuery>>(
        &self,
        answer_custom_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_custom_query
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(answer_custom_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the result of an inline query; for bots only
    pub async fn answer_inline_query<C: AsRef<AnswerInlineQuery>>(
        &self,
        answer_inline_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_inline_query
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(answer_inline_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the result of a pre-checkout query; for bots only
    pub async fn answer_pre_checkout_query<C: AsRef<AnswerPreCheckoutQuery>>(
        &self,
        answer_pre_checkout_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_pre_checkout_query
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(answer_pre_checkout_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the result of a shipping query; for bots only
    pub async fn answer_shipping_query<C: AsRef<AnswerShippingQuery>>(
        &self,
        answer_shipping_query: C,
    ) -> RTDResult<Ok> {
        let extra = answer_shipping_query
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(answer_shipping_query.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds a user to the blacklist
    pub async fn block_user<C: AsRef<BlockUser>>(&self, block_user: C) -> RTDResult<Ok> {
        let extra = block_user.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(block_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks whether the current session can be used to transfer a chat ownership to another user
    pub async fn can_transfer_ownership<C: AsRef<CanTransferOwnership>>(
        &self,
        can_transfer_ownership: C,
    ) -> RTDResult<CanTransferOwnershipResult> {
        let extra = can_transfer_ownership
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(can_transfer_ownership.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::CanTransferOwnershipResult(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Stops the downloading of a file. If a file has already been downloaded, does nothing
    pub async fn cancel_download_file<C: AsRef<CancelDownloadFile>>(
        &self,
        cancel_download_file: C,
    ) -> RTDResult<Ok> {
        let extra = cancel_download_file
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(cancel_download_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Stops the uploading of a file. Supported only for files uploaded by using uploadFile. For other files the behavior is undefined
    pub async fn cancel_upload_file<C: AsRef<CancelUploadFile>>(
        &self,
        cancel_upload_file: C,
    ) -> RTDResult<Ok> {
        let extra = cancel_upload_file
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(cancel_upload_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes imported contacts using the list of current user contacts saved on the device. Imports newly added contacts and, if at least the file database is enabled, deletes recently deleted contacts. Query result depends on the result of the previous query, so only one query is possible at the same time
    pub async fn change_imported_contacts<C: AsRef<ChangeImportedContacts>>(
        &self,
        change_imported_contacts: C,
    ) -> RTDResult<ImportedContacts> {
        let extra = change_imported_contacts
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(change_imported_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ImportedContacts(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the phone number of the user and sends an authentication code to the user's new phone number. On success, returns information about the sent code
    pub async fn change_phone_number<C: AsRef<ChangePhoneNumber>>(
        &self,
        change_phone_number: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = change_phone_number
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(change_phone_number.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Installs/uninstalls or activates/archives a sticker set
    pub async fn change_sticker_set<C: AsRef<ChangeStickerSet>>(
        &self,
        change_sticker_set: C,
    ) -> RTDResult<Ok> {
        let extra = change_sticker_set
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(change_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is authorizationStateWaitPhoneNumber. Can be used instead of setAuthenticationPhoneNumber and checkAuthenticationCode to log in
    pub async fn check_authentication_bot_token<C: AsRef<CheckAuthenticationBotToken>>(
        &self,
        check_authentication_bot_token: C,
    ) -> RTDResult<Ok> {
        let extra = check_authentication_bot_token
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(check_authentication_bot_token.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the authentication code. Works only when the current authorization state is authorizationStateWaitCode
    pub async fn check_authentication_code<C: AsRef<CheckAuthenticationCode>>(
        &self,
        check_authentication_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_authentication_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(check_authentication_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the authentication password for correctness. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn check_authentication_password<C: AsRef<CheckAuthenticationPassword>>(
        &self,
        check_authentication_password: C,
    ) -> RTDResult<Ok> {
        let extra = check_authentication_password
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(check_authentication_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the authentication code sent to confirm a new phone number of the user
    pub async fn check_change_phone_number_code<C: AsRef<CheckChangePhoneNumberCode>>(
        &self,
        check_change_phone_number_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_change_phone_number_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(check_change_phone_number_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the validity of an invite link for a chat and returns information about the corresponding chat
    pub async fn check_chat_invite_link<C: AsRef<CheckChatInviteLink>>(
        &self,
        check_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLinkInfo> {
        let extra = check_chat_invite_link
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(check_chat_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatInviteLinkInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks whether a username can be set for a chat
    pub async fn check_chat_username<C: AsRef<CheckChatUsername>>(
        &self,
        check_chat_username: C,
    ) -> RTDResult<CheckChatUsernameResult> {
        let extra = check_chat_username
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(check_chat_username.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::CheckChatUsernameResult(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks whether the maximum number of owned public chats has been reached. Returns corresponding error if the limit was reached
    pub async fn check_created_public_chats_limit<C: AsRef<CheckCreatedPublicChatsLimit>>(
        &self,
        check_created_public_chats_limit: C,
    ) -> RTDResult<Ok> {
        let extra = check_created_public_chats_limit
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(check_created_public_chats_limit.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the database encryption key for correctness. Works only when the current authorization state is authorizationStateWaitEncryptionKey
    pub async fn check_database_encryption_key<C: AsRef<CheckDatabaseEncryptionKey>>(
        &self,
        check_database_encryption_key: C,
    ) -> RTDResult<Ok> {
        let extra = check_database_encryption_key
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(check_database_encryption_key.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the email address verification code for Telegram Passport
    pub async fn check_email_address_verification_code<
        C: AsRef<CheckEmailAddressVerificationCode>,
    >(
        &self,
        check_email_address_verification_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_email_address_verification_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(check_email_address_verification_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks phone number confirmation code
    pub async fn check_phone_number_confirmation_code<
        C: AsRef<CheckPhoneNumberConfirmationCode>,
    >(
        &self,
        check_phone_number_confirmation_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_phone_number_confirmation_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(check_phone_number_confirmation_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the phone number verification code for Telegram Passport
    pub async fn check_phone_number_verification_code<
        C: AsRef<CheckPhoneNumberVerificationCode>,
    >(
        &self,
        check_phone_number_verification_code: C,
    ) -> RTDResult<Ok> {
        let extra = check_phone_number_verification_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(check_phone_number_verification_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Checks the 2-step verification recovery email address verification code
    pub async fn check_recovery_email_address_code<C: AsRef<CheckRecoveryEmailAddressCode>>(
        &self,
        check_recovery_email_address_code: C,
    ) -> RTDResult<PasswordState> {
        let extra =
            check_recovery_email_address_code
                .as_ref()
                .extra()
                .ok_or(RTDError::Internal(
                    "invalid tdlib response type, not have `extra` field",
                ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(check_recovery_email_address_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes potentially dangerous characters from the name of a file. The encoding of the file name is supposed to be UTF-8. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn clean_file_name<C: AsRef<CleanFileName>>(
        &self,
        clean_file_name: C,
    ) -> RTDResult<Text> {
        let extra = clean_file_name.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(clean_file_name.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Clears draft messages in all chats
    pub async fn clear_all_draft_messages<C: AsRef<ClearAllDraftMessages>>(
        &self,
        clear_all_draft_messages: C,
    ) -> RTDResult<Ok> {
        let extra = clear_all_draft_messages
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(clear_all_draft_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Clears all imported contacts, contact list remains unchanged
    pub async fn clear_imported_contacts<C: AsRef<ClearImportedContacts>>(
        &self,
        clear_imported_contacts: C,
    ) -> RTDResult<Ok> {
        let extra = clear_imported_contacts
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(clear_imported_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Clears the list of recently used stickers
    pub async fn clear_recent_stickers<C: AsRef<ClearRecentStickers>>(
        &self,
        clear_recent_stickers: C,
    ) -> RTDResult<Ok> {
        let extra = clear_recent_stickers
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(clear_recent_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Clears the list of recently found chats
    pub async fn clear_recently_found_chats<C: AsRef<ClearRecentlyFoundChats>>(
        &self,
        clear_recently_found_chats: C,
    ) -> RTDResult<Ok> {
        let extra = clear_recently_found_chats
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(clear_recently_found_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Closes the TDLib instance. All databases will be flushed to disk and properly closed. After the close completes, updateAuthorizationState with authorizationStateClosed will be sent
    pub async fn close<C: AsRef<Close>>(&self, close: C) -> RTDResult<Ok> {
        let extra = close.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(close.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs TDLib that the chat is closed by the user. Many useful activities depend on the chat being opened or closed
    pub async fn close_chat<C: AsRef<CloseChat>>(&self, close_chat: C) -> RTDResult<Ok> {
        let extra = close_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(close_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Closes a secret chat, effectively transferring its state to secretChatStateClosed
    pub async fn close_secret_chat<C: AsRef<CloseSecretChat>>(
        &self,
        close_secret_chat: C,
    ) -> RTDResult<Ok> {
        let extra = close_secret_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(close_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Confirms QR code authentication on another device. Returns created session on success
    pub async fn confirm_qr_code_authentication<C: AsRef<ConfirmQrCodeAuthentication>>(
        &self,
        confirm_qr_code_authentication: C,
    ) -> RTDResult<Session> {
        let extra = confirm_qr_code_authentication
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(confirm_qr_code_authentication.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Session(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an existing chat corresponding to a known basic group
    pub async fn create_basic_group_chat<C: AsRef<CreateBasicGroupChat>>(
        &self,
        create_basic_group_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_basic_group_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_basic_group_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Creates a new call
    pub async fn create_call<C: AsRef<CreateCall>>(&self, create_call: C) -> RTDResult<CallId> {
        let extra = create_call.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::CallId(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Creates a new basic group and sends a corresponding messageBasicGroupChatCreate. Returns the newly created chat
    pub async fn create_new_basic_group_chat<C: AsRef<CreateNewBasicGroupChat>>(
        &self,
        create_new_basic_group_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_new_basic_group_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_new_basic_group_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Creates a new secret chat. Returns the newly created chat
    pub async fn create_new_secret_chat<C: AsRef<CreateNewSecretChat>>(
        &self,
        create_new_secret_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_new_secret_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_new_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Creates a new sticker set; for bots only. Returns the newly created sticker set
    pub async fn create_new_sticker_set<C: AsRef<CreateNewStickerSet>>(
        &self,
        create_new_sticker_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = create_new_sticker_set
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_new_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSet(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Creates a new supergroup or channel and sends a corresponding messageSupergroupChatCreate. Returns the newly created chat
    pub async fn create_new_supergroup_chat<C: AsRef<CreateNewSupergroupChat>>(
        &self,
        create_new_supergroup_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_new_supergroup_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_new_supergroup_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an existing chat corresponding to a given user
    pub async fn create_private_chat<C: AsRef<CreatePrivateChat>>(
        &self,
        create_private_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_private_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_private_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an existing chat corresponding to a known secret chat
    pub async fn create_secret_chat<C: AsRef<CreateSecretChat>>(
        &self,
        create_secret_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_secret_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an existing chat corresponding to a known supergroup or channel
    pub async fn create_supergroup_chat<C: AsRef<CreateSupergroupChat>>(
        &self,
        create_supergroup_chat: C,
    ) -> RTDResult<Chat> {
        let extra = create_supergroup_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_supergroup_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Creates a new temporary password for processing payments
    pub async fn create_temporary_password<C: AsRef<CreateTemporaryPassword>>(
        &self,
        create_temporary_password: C,
    ) -> RTDResult<TemporaryPasswordState> {
        let extra = create_temporary_password
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(create_temporary_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TemporaryPasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes the account of the current user, deleting all information associated with the user from the server. The phone number of the account can be used to create a new account. Can be called before authorization when the current authorization state is authorizationStateWaitPassword
    pub async fn delete_account<C: AsRef<DeleteAccount>>(
        &self,
        delete_account: C,
    ) -> RTDResult<Ok> {
        let extra = delete_account.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_account.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes all messages in the chat. Use Chat.can_be_deleted_only_for_self and Chat.can_be_deleted_for_all_users fields to find whether and how the method can be applied to the chat
    pub async fn delete_chat_history<C: AsRef<DeleteChatHistory>>(
        &self,
        delete_chat_history: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_history
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_chat_history.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes all messages sent by the specified user to a chat. Supported only for supergroups; requires can_delete_messages administrator privileges
    pub async fn delete_chat_messages_from_user<C: AsRef<DeleteChatMessagesFromUser>>(
        &self,
        delete_chat_messages_from_user: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_messages_from_user
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_chat_messages_from_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes the default reply markup from a chat. Must be called after a one-time keyboard or a ForceReply reply markup has been used. UpdateChatReplyMarkup will be sent if the reply markup will be changed
    pub async fn delete_chat_reply_markup<C: AsRef<DeleteChatReplyMarkup>>(
        &self,
        delete_chat_reply_markup: C,
    ) -> RTDResult<Ok> {
        let extra = delete_chat_reply_markup
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_chat_reply_markup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes a file from the TDLib file cache
    pub async fn delete_file<C: AsRef<DeleteFile>>(&self, delete_file: C) -> RTDResult<Ok> {
        let extra = delete_file.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization
    pub async fn delete_language_pack<C: AsRef<DeleteLanguagePack>>(
        &self,
        delete_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = delete_language_pack
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_language_pack.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes messages
    pub async fn delete_messages<C: AsRef<DeleteMessages>>(
        &self,
        delete_messages: C,
    ) -> RTDResult<Ok> {
        let extra = delete_messages.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes a Telegram Passport element
    pub async fn delete_passport_element<C: AsRef<DeletePassportElement>>(
        &self,
        delete_passport_element: C,
    ) -> RTDResult<Ok> {
        let extra = delete_passport_element
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_passport_element.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes a profile photo. If something changes, updateUser will be sent
    pub async fn delete_profile_photo<C: AsRef<DeleteProfilePhoto>>(
        &self,
        delete_profile_photo: C,
    ) -> RTDResult<Ok> {
        let extra = delete_profile_photo
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_profile_photo.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes saved credentials for all payment provider bots
    pub async fn delete_saved_credentials<C: AsRef<DeleteSavedCredentials>>(
        &self,
        delete_saved_credentials: C,
    ) -> RTDResult<Ok> {
        let extra = delete_saved_credentials
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_saved_credentials.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes saved order info
    pub async fn delete_saved_order_info<C: AsRef<DeleteSavedOrderInfo>>(
        &self,
        delete_saved_order_info: C,
    ) -> RTDResult<Ok> {
        let extra = delete_saved_order_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_saved_order_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires owner privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method
    pub async fn delete_supergroup<C: AsRef<DeleteSupergroup>>(
        &self,
        delete_supergroup: C,
    ) -> RTDResult<Ok> {
        let extra = delete_supergroup
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(delete_supergroup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Closes the TDLib instance, destroying all local data without a proper logout. The current user session will remain in the list of all active sessions. All local data will be destroyed. After the destruction completes updateAuthorizationState with authorizationStateClosed will be sent
    pub async fn destroy<C: AsRef<Destroy>>(&self, destroy: C) -> RTDResult<Ok> {
        let extra = destroy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(destroy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Disables the currently enabled proxy. Can be called before authorization
    pub async fn disable_proxy<C: AsRef<DisableProxy>>(&self, disable_proxy: C) -> RTDResult<Ok> {
        let extra = disable_proxy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(disable_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Discards a call
    pub async fn discard_call<C: AsRef<DiscardCall>>(&self, discard_call: C) -> RTDResult<Ok> {
        let extra = discard_call.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(discard_call.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Disconnects all websites from the current user's Telegram account
    pub async fn disconnect_all_websites<C: AsRef<DisconnectAllWebsites>>(
        &self,
        disconnect_all_websites: C,
    ) -> RTDResult<Ok> {
        let extra = disconnect_all_websites
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(disconnect_all_websites.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Disconnects website from the current user's Telegram account
    pub async fn disconnect_website<C: AsRef<DisconnectWebsite>>(
        &self,
        disconnect_website: C,
    ) -> RTDResult<Ok> {
        let extra = disconnect_website
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(disconnect_website.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Downloads a file from the cloud. Download progress and completion of the download will be notified through updateFile updates
    pub async fn download_file<C: AsRef<DownloadFile>>(&self, download_file: C) -> RTDResult<File> {
        let extra = download_file.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(download_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::File(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits information about a custom local language pack in the current localization target. Can be called before authorization
    pub async fn edit_custom_language_pack_info<C: AsRef<EditCustomLanguagePackInfo>>(
        &self,
        edit_custom_language_pack_info: C,
    ) -> RTDResult<Ok> {
        let extra = edit_custom_language_pack_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_custom_language_pack_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the caption of an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_caption<C: AsRef<EditInlineMessageCaption>>(
        &self,
        edit_inline_message_caption: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_caption
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_inline_message_caption.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the content of a live location in an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_live_location<C: AsRef<EditInlineMessageLiveLocation>>(
        &self,
        edit_inline_message_live_location: C,
    ) -> RTDResult<Ok> {
        let extra =
            edit_inline_message_live_location
                .as_ref()
                .extra()
                .ok_or(RTDError::Internal(
                    "invalid tdlib response type, not have `extra` field",
                ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(edit_inline_message_live_location.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the content of a message with an animation, an audio, a document, a photo or a video in an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_media<C: AsRef<EditInlineMessageMedia>>(
        &self,
        edit_inline_message_media: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_media
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_inline_message_media.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the reply markup of an inline message sent via a bot; for bots only
    pub async fn edit_inline_message_reply_markup<C: AsRef<EditInlineMessageReplyMarkup>>(
        &self,
        edit_inline_message_reply_markup: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_reply_markup
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(edit_inline_message_reply_markup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the text of an inline text or game message sent via a bot; for bots only
    pub async fn edit_inline_message_text<C: AsRef<EditInlineMessageText>>(
        &self,
        edit_inline_message_text: C,
    ) -> RTDResult<Ok> {
        let extra = edit_inline_message_text
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_inline_message_text.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the message content caption. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_caption<C: AsRef<EditMessageCaption>>(
        &self,
        edit_message_caption: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_caption
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_message_caption.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the message content of a live location. Messages can be edited for a limited period of time specified in the live location. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_live_location<C: AsRef<EditMessageLiveLocation>>(
        &self,
        edit_message_live_location: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_live_location
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_message_live_location.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the content of a message with an animation, an audio, a document, a photo or a video. The media in the message can't be replaced if the message was set to self-destruct. Media can't be replaced by self-destructing media. Media in an album can be edited only to contain a photo or a video. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_media<C: AsRef<EditMessageMedia>>(
        &self,
        edit_message_media: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_media
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_message_media.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the message reply markup; for bots only. Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_reply_markup<C: AsRef<EditMessageReplyMarkup>>(
        &self,
        edit_message_reply_markup: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_reply_markup
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_message_reply_markup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the time when a scheduled message will be sent. Scheduling state of all messages in the same album or forwarded together with the message will be also changed
    pub async fn edit_message_scheduling_state<C: AsRef<EditMessageSchedulingState>>(
        &self,
        edit_message_scheduling_state: C,
    ) -> RTDResult<Ok> {
        let extra = edit_message_scheduling_state
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_message_scheduling_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits the text of a message (or a text of a game message). Returns the edited message after the edit is completed on the server side
    pub async fn edit_message_text<C: AsRef<EditMessageText>>(
        &self,
        edit_message_text: C,
    ) -> RTDResult<Message> {
        let extra = edit_message_text
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_message_text.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Edits an existing proxy server for network requests. Can be called before authorization
    pub async fn edit_proxy<C: AsRef<EditProxy>>(&self, edit_proxy: C) -> RTDResult<Proxy> {
        let extra = edit_proxy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(edit_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Proxy(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Enables a proxy. Only one proxy can be enabled at a time. Can be called before authorization
    pub async fn enable_proxy<C: AsRef<EnableProxy>>(&self, enable_proxy: C) -> RTDResult<Ok> {
        let extra = enable_proxy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(enable_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Finishes the file generation
    pub async fn finish_file_generation<C: AsRef<FinishFileGeneration>>(
        &self,
        finish_file_generation: C,
    ) -> RTDResult<Ok> {
        let extra = finish_file_generation
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(finish_file_generation.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Forwards previously sent messages. Returns the forwarded messages in the same order as the message identifiers passed in message_ids. If a message can't be forwarded, null will be returned instead of the message
    pub async fn forward_messages<C: AsRef<ForwardMessages>>(
        &self,
        forward_messages: C,
    ) -> RTDResult<Messages> {
        let extra = forward_messages.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(forward_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Generates a new invite link for a chat; the previously generated link is revoked. Available for basic groups, supergroups, and channels. Requires administrator privileges and can_invite_users right
    pub async fn generate_chat_invite_link<C: AsRef<GenerateChatInviteLink>>(
        &self,
        generate_chat_invite_link: C,
    ) -> RTDResult<ChatInviteLink> {
        let extra = generate_chat_invite_link
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(generate_chat_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatInviteLink(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the period of inactivity after which the account of the current user will automatically be deleted
    pub async fn get_account_ttl<C: AsRef<GetAccountTtl>>(
        &self,
        get_account_ttl: C,
    ) -> RTDResult<AccountTtl> {
        let extra = get_account_ttl.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_account_ttl.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AccountTtl(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all active live locations that should be updated by the client. The list is persistent across application restarts only if the message database is used
    pub async fn get_active_live_location_messages<C: AsRef<GetActiveLiveLocationMessages>>(
        &self,
        get_active_live_location_messages: C,
    ) -> RTDResult<Messages> {
        let extra =
            get_active_live_location_messages
                .as_ref()
                .extra()
                .ok_or(RTDError::Internal(
                    "invalid tdlib response type, not have `extra` field",
                ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(get_active_live_location_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all active sessions of the current user
    pub async fn get_active_sessions<C: AsRef<GetActiveSessions>>(
        &self,
        get_active_sessions: C,
    ) -> RTDResult<Sessions> {
        let extra = get_active_sessions
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_active_sessions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Sessions(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all available Telegram Passport elements
    pub async fn get_all_passport_elements<C: AsRef<GetAllPassportElements>>(
        &self,
        get_all_passport_elements: C,
    ) -> RTDResult<PassportElements> {
        let extra = get_all_passport_elements
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_all_passport_elements.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PassportElements(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns application config, provided by the server. Can be called before authorization
    pub async fn get_application_config<C: AsRef<GetApplicationConfig>>(
        &self,
        get_application_config: C,
    ) -> RTDResult<JsonValue> {
        let extra = get_application_config
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_application_config.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::JsonValue(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of archived sticker sets
    pub async fn get_archived_sticker_sets<C: AsRef<GetArchivedStickerSets>>(
        &self,
        get_archived_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_archived_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_archived_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSets(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of sticker sets attached to a file. Currently only photos and videos can have attached sticker sets
    pub async fn get_attached_sticker_sets<C: AsRef<GetAttachedStickerSets>>(
        &self,
        get_attached_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_attached_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_attached_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSets(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state
    pub async fn get_authorization_state<C: AsRef<GetAuthorizationState>>(
        &self,
        get_authorization_state: C,
    ) -> RTDResult<AuthorizationState> {
        let extra = get_authorization_state
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_authorization_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AuthorizationState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns auto-download settings presets for the currently logged in user
    pub async fn get_auto_download_settings_presets<C: AsRef<GetAutoDownloadSettingsPresets>>(
        &self,
        get_auto_download_settings_presets: C,
    ) -> RTDResult<AutoDownloadSettingsPresets> {
        let extra =
            get_auto_download_settings_presets
                .as_ref()
                .extra()
                .ok_or(RTDError::Internal(
                    "invalid tdlib response type, not have `extra` field",
                ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(get_auto_download_settings_presets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AutoDownloadSettingsPresets(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Constructs a persistent HTTP URL for a background
    pub async fn get_background_url<C: AsRef<GetBackgroundUrl>>(
        &self,
        get_background_url: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_background_url
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_background_url.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::HttpUrl(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns backgrounds installed by the user
    pub async fn get_backgrounds<C: AsRef<GetBackgrounds>>(
        &self,
        get_backgrounds: C,
    ) -> RTDResult<Backgrounds> {
        let extra = get_backgrounds.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_backgrounds.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Backgrounds(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a basic group by its identifier. This is an offline request if the current user is not a bot
    pub async fn get_basic_group<C: AsRef<GetBasicGroup>>(
        &self,
        get_basic_group: C,
    ) -> RTDResult<BasicGroup> {
        let extra = get_basic_group.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_basic_group.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::BasicGroup(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns full information about a basic group by its identifier
    pub async fn get_basic_group_full_info<C: AsRef<GetBasicGroupFullInfo>>(
        &self,
        get_basic_group_full_info: C,
    ) -> RTDResult<BasicGroupFullInfo> {
        let extra = get_basic_group_full_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_basic_group_full_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::BasicGroupFullInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns users that were blocked by the current user
    pub async fn get_blocked_users<C: AsRef<GetBlockedUsers>>(
        &self,
        get_blocked_users: C,
    ) -> RTDResult<Users> {
        let extra = get_blocked_users
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_blocked_users.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Users(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a callback query to a bot and returns an answer. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
    pub async fn get_callback_query_answer<C: AsRef<GetCallbackQueryAnswer>>(
        &self,
        get_callback_query_answer: C,
    ) -> RTDResult<CallbackQueryAnswer> {
        let extra = get_callback_query_answer
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_callback_query_answer.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::CallbackQueryAnswer(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a chat by its identifier, this is an offline request if the current user is not a bot
    pub async fn get_chat<C: AsRef<GetChat>>(&self, get_chat: C) -> RTDResult<Chat> {
        let extra = get_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of administrators of the chat with their custom titles
    pub async fn get_chat_administrators<C: AsRef<GetChatAdministrators>>(
        &self,
        get_chat_administrators: C,
    ) -> RTDResult<ChatAdministrators> {
        let extra = get_chat_administrators
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_administrators.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatAdministrators(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of service actions taken by chat members and administrators in the last 48 hours. Available only for supergroups and channels. Requires administrator rights. Returns results in reverse chronological order (i. e., in order of decreasing event_id)
    pub async fn get_chat_event_log<C: AsRef<GetChatEventLog>>(
        &self,
        get_chat_event_log: C,
    ) -> RTDResult<ChatEvents> {
        let extra = get_chat_event_log
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_event_log.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatEvents(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library. This is an offline request if only_local is true
    pub async fn get_chat_history<C: AsRef<GetChatHistory>>(
        &self,
        get_chat_history: C,
    ) -> RTDResult<Messages> {
        let extra = get_chat_history.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_history.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a single member of a chat
    pub async fn get_chat_member<C: AsRef<GetChatMember>>(
        &self,
        get_chat_member: C,
    ) -> RTDResult<ChatMember> {
        let extra = get_chat_member.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_member.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatMember(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the last message sent in a chat no later than the specified date
    pub async fn get_chat_message_by_date<C: AsRef<GetChatMessageByDate>>(
        &self,
        get_chat_message_by_date: C,
    ) -> RTDResult<Message> {
        let extra = get_chat_message_by_date
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_message_by_date.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns approximate number of messages of the specified type in the chat
    pub async fn get_chat_message_count<C: AsRef<GetChatMessageCount>>(
        &self,
        get_chat_message_count: C,
    ) -> RTDResult<Count> {
        let extra = get_chat_message_count
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_message_count.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Count(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns list of chats with non-default notification settings
    pub async fn get_chat_notification_settings_exceptions<
        C: AsRef<GetChatNotificationSettingsExceptions>,
    >(
        &self,
        get_chat_notification_settings_exceptions: C,
    ) -> RTDResult<Chats> {
        let extra = get_chat_notification_settings_exceptions
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(get_chat_notification_settings_exceptions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a pinned chat message
    pub async fn get_chat_pinned_message<C: AsRef<GetChatPinnedMessage>>(
        &self,
        get_chat_pinned_message: C,
    ) -> RTDResult<Message> {
        let extra = get_chat_pinned_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_pinned_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all scheduled messages in a chat. The messages are returned in a reverse chronological order (i.e., in order of decreasing message_id)
    pub async fn get_chat_scheduled_messages<C: AsRef<GetChatScheduledMessages>>(
        &self,
        get_chat_scheduled_messages: C,
    ) -> RTDResult<Messages> {
        let extra = get_chat_scheduled_messages
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_scheduled_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an HTTP URL with the chat statistics. Currently this method can be used only for channels. Can be used only if SupergroupFullInfo.can_view_statistics == true
    pub async fn get_chat_statistics_url<C: AsRef<GetChatStatisticsUrl>>(
        &self,
        get_chat_statistics_url: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_chat_statistics_url
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chat_statistics_url.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::HttpUrl(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an ordered list of chats in a chat list. Chats are sorted by the pair (order, chat_id) in decreasing order. (For example, to get a list of chats from the beginning, the offset_order should be equal to a biggest signed 64-bit number 9223372036854775807 == 2^63  1). For optimal performance the number of returned chats is chosen by the library
    pub async fn get_chats<C: AsRef<GetChats>>(&self, get_chats: C) -> RTDResult<Chats> {
        let extra = get_chats.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all website where the current user used Telegram to log in
    pub async fn get_connected_websites<C: AsRef<GetConnectedWebsites>>(
        &self,
        get_connected_websites: C,
    ) -> RTDResult<ConnectedWebsites> {
        let extra = get_connected_websites
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_connected_websites.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ConnectedWebsites(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all user contacts
    pub async fn get_contacts<C: AsRef<GetContacts>>(&self, get_contacts: C) -> RTDResult<Users> {
        let extra = get_contacts.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Users(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Uses current user IP to found their country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization
    pub async fn get_country_code<C: AsRef<GetCountryCode>>(
        &self,
        get_country_code: C,
    ) -> RTDResult<Text> {
        let extra = get_country_code.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_country_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of public chats of the specified type, owned by the user
    pub async fn get_created_public_chats<C: AsRef<GetCreatedPublicChats>>(
        &self,
        get_created_public_chats: C,
    ) -> RTDResult<Chats> {
        let extra = get_created_public_chats
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_created_public_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all updates needed to restore current TDLib state, i.e. all actual UpdateAuthorizationState/UpdateUser/UpdateNewChat and others. This is especially useful if TDLib is run in a separate process. This is an offline method. Can be called before authorization
    pub async fn get_current_state<C: AsRef<GetCurrentState>>(
        &self,
        get_current_state: C,
    ) -> RTDResult<Updates> {
        let extra = get_current_state
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_current_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Updates(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns database statistics
    pub async fn get_database_statistics<C: AsRef<GetDatabaseStatistics>>(
        &self,
        get_database_statistics: C,
    ) -> RTDResult<DatabaseStatistics> {
        let extra = get_database_statistics
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_database_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::DatabaseStatistics(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization
    pub async fn get_deep_link_info<C: AsRef<GetDeepLinkInfo>>(
        &self,
        get_deep_link_info: C,
    ) -> RTDResult<DeepLinkInfo> {
        let extra = get_deep_link_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_deep_link_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::DeepLinkInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an HTTP URL which can be used to automatically log in to the translation platform and suggest new emoji replacements. The URL will be valid for 30 seconds after generation
    pub async fn get_emoji_suggestions_url<C: AsRef<GetEmojiSuggestionsUrl>>(
        &self,
        get_emoji_suggestions_url: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_emoji_suggestions_url
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_emoji_suggestions_url.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::HttpUrl(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns favorite stickers
    pub async fn get_favorite_stickers<C: AsRef<GetFavoriteStickers>>(
        &self,
        get_favorite_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = get_favorite_stickers
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_favorite_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Stickers(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a file; this is an offline request
    pub async fn get_file<C: AsRef<GetFile>>(&self, get_file: C) -> RTDResult<File> {
        let extra = get_file.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::File(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns file downloaded prefix size from a given offset
    pub async fn get_file_downloaded_prefix_size<C: AsRef<GetFileDownloadedPrefixSize>>(
        &self,
        get_file_downloaded_prefix_size: C,
    ) -> RTDResult<Count> {
        let extra = get_file_downloaded_prefix_size
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(get_file_downloaded_prefix_size.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Count(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the extension of a file, guessed by its MIME type. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_file_extension<C: AsRef<GetFileExtension>>(
        &self,
        get_file_extension: C,
    ) -> RTDResult<Text> {
        let extra = get_file_extension
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_file_extension.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the MIME type of a file, guessed by its extension. Returns an empty string on failure. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_file_mime_type<C: AsRef<GetFileMimeType>>(
        &self,
        get_file_mime_type: C,
    ) -> RTDResult<Text> {
        let extra = get_file_mime_type
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_file_mime_type.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only
    pub async fn get_game_high_scores<C: AsRef<GetGameHighScores>>(
        &self,
        get_game_high_scores: C,
    ) -> RTDResult<GameHighScores> {
        let extra = get_game_high_scores
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_game_high_scores.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::GameHighScores(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of common group chats with a given user. Chats are sorted by their type and creation date
    pub async fn get_groups_in_common<C: AsRef<GetGroupsInCommon>>(
        &self,
        get_groups_in_common: C,
    ) -> RTDResult<Chats> {
        let extra = get_groups_in_common
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_groups_in_common.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the total number of imported contacts
    pub async fn get_imported_contact_count<C: AsRef<GetImportedContactCount>>(
        &self,
        get_imported_contact_count: C,
    ) -> RTDResult<Count> {
        let extra = get_imported_contact_count
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_imported_contact_count.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Count(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of recently inactive supergroups and channels. Can be used when user reaches limit on the number of joined supergroups and channels and receives CHANNELS_TOO_MUCH error
    pub async fn get_inactive_supergroup_chats<C: AsRef<GetInactiveSupergroupChats>>(
        &self,
        get_inactive_supergroup_chats: C,
    ) -> RTDResult<Chats> {
        let extra = get_inactive_supergroup_chats
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_inactive_supergroup_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns game high scores and some part of the high score table in the range of the specified user; for bots only
    pub async fn get_inline_game_high_scores<C: AsRef<GetInlineGameHighScores>>(
        &self,
        get_inline_game_high_scores: C,
    ) -> RTDResult<GameHighScores> {
        let extra = get_inline_game_high_scores
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_inline_game_high_scores.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::GameHighScores(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends an inline query to a bot and returns its results. Returns an error with code 502 if the bot fails to answer the query before the query timeout expires
    pub async fn get_inline_query_results<C: AsRef<GetInlineQueryResults>>(
        &self,
        get_inline_query_results: C,
    ) -> RTDResult<InlineQueryResults> {
        let extra = get_inline_query_results
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_inline_query_results.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::InlineQueryResults(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of installed sticker sets
    pub async fn get_installed_sticker_sets<C: AsRef<GetInstalledStickerSets>>(
        &self,
        get_installed_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_installed_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_installed_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSets(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram
    pub async fn get_invite_text<C: AsRef<GetInviteText>>(
        &self,
        get_invite_text: C,
    ) -> RTDResult<Text> {
        let extra = get_invite_text.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_invite_text.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Converts a JsonValue object to corresponding JSON-serialized string. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_json_string<C: AsRef<GetJsonString>>(
        &self,
        get_json_string: C,
    ) -> RTDResult<Text> {
        let extra = get_json_string.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_json_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Converts a JSON-serialized string to corresponding JsonValue object. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_json_value<C: AsRef<GetJsonValue>>(
        &self,
        get_json_value: C,
    ) -> RTDResult<JsonValue> {
        let extra = get_json_value.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_json_value.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::JsonValue(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization
    pub async fn get_language_pack_info<C: AsRef<GetLanguagePackInfo>>(
        &self,
        get_language_pack_info: C,
    ) -> RTDResult<LanguagePackInfo> {
        let extra = get_language_pack_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_language_pack_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LanguagePackInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_language_pack_string<C: AsRef<GetLanguagePackString>>(
        &self,
        get_language_pack_string: C,
    ) -> RTDResult<LanguagePackStringValue> {
        let extra = get_language_pack_string
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_language_pack_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LanguagePackStringValue(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns strings from a language pack in the current localization target by their keys. Can be called before authorization
    pub async fn get_language_pack_strings<C: AsRef<GetLanguagePackStrings>>(
        &self,
        get_language_pack_strings: C,
    ) -> RTDResult<LanguagePackStrings> {
        let extra = get_language_pack_strings
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_language_pack_strings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LanguagePackStrings(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization
    pub async fn get_localization_target_info<C: AsRef<GetLocalizationTargetInfo>>(
        &self,
        get_localization_target_info: C,
    ) -> RTDResult<LocalizationTargetInfo> {
        let extra = get_localization_target_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_localization_target_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LocalizationTargetInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about currently used log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_log_stream<C: AsRef<GetLogStream>>(
        &self,
        get_log_stream: C,
    ) -> RTDResult<LogStream> {
        let extra = get_log_stream.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_log_stream.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LogStream(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns current verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_log_tag_verbosity_level<C: AsRef<GetLogTagVerbosityLevel>>(
        &self,
        get_log_tag_verbosity_level: C,
    ) -> RTDResult<LogVerbosityLevel> {
        let extra = get_log_tag_verbosity_level
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_log_tag_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LogVerbosityLevel(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_log_tags<C: AsRef<GetLogTags>>(&self, get_log_tags: C) -> RTDResult<LogTags> {
        let extra = get_log_tags.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_log_tags.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LogTags(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns current verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_log_verbosity_level<C: AsRef<GetLogVerbosityLevel>>(
        &self,
        get_log_verbosity_level: C,
    ) -> RTDResult<LogVerbosityLevel> {
        let extra = get_log_verbosity_level
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_log_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LogVerbosityLevel(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an HTTP URL which can be used to automatically authorize the user on a website after clicking an inline button of type inlineKeyboardButtonTypeLoginUrl. Use the method getLoginUrlInfo to find whether a prior user confirmation is needed. If an error is returned, then the button must be handled as an ordinary URL button
    pub async fn get_login_url<C: AsRef<GetLoginUrl>>(
        &self,
        get_login_url: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_login_url.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_login_url.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::HttpUrl(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a button of type inlineKeyboardButtonTypeLoginUrl. The method needs to be called when the user presses the button
    pub async fn get_login_url_info<C: AsRef<GetLoginUrlInfo>>(
        &self,
        get_login_url_info: C,
    ) -> RTDResult<LoginUrlInfo> {
        let extra = get_login_url_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_login_url_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::LoginUrlInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded
    pub async fn get_map_thumbnail_file<C: AsRef<GetMapThumbnailFile>>(
        &self,
        get_map_thumbnail_file: C,
    ) -> RTDResult<File> {
        let extra = get_map_thumbnail_file
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_map_thumbnail_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::File(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the current user
    pub async fn get_me<C: AsRef<GetMe>>(&self, get_me: C) -> RTDResult<User> {
        let extra = get_me.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_me.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::User(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a message
    pub async fn get_message<C: AsRef<GetMessage>>(&self, get_message: C) -> RTDResult<Message> {
        let extra = get_message.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a private HTTPS link to a message in a chat. Available only for already sent messages in supergroups and channels. The link will work only for members of the chat
    pub async fn get_message_link<C: AsRef<GetMessageLink>>(
        &self,
        get_message_link: C,
    ) -> RTDResult<HttpUrl> {
        let extra = get_message_link.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_message_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::HttpUrl(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a public or private message link
    pub async fn get_message_link_info<C: AsRef<GetMessageLinkInfo>>(
        &self,
        get_message_link_info: C,
    ) -> RTDResult<MessageLinkInfo> {
        let extra = get_message_link_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_message_link_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::MessageLinkInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a message, if it is available locally without sending network request. This is an offline request
    pub async fn get_message_locally<C: AsRef<GetMessageLocally>>(
        &self,
        get_message_locally: C,
    ) -> RTDResult<Message> {
        let extra = get_message_locally
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_message_locally.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about messages. If a message is not found, returns null on the corresponding position of the result
    pub async fn get_messages<C: AsRef<GetMessages>>(
        &self,
        get_messages: C,
    ) -> RTDResult<Messages> {
        let extra = get_messages.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns network data usage statistics. Can be called before authorization
    pub async fn get_network_statistics<C: AsRef<GetNetworkStatistics>>(
        &self,
        get_network_statistics: C,
    ) -> RTDResult<NetworkStatistics> {
        let extra = get_network_statistics
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_network_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::NetworkStatistics(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
    pub async fn get_option<C: AsRef<GetOption>>(&self, get_option: C) -> RTDResult<OptionValue> {
        let extra = get_option.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_option.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::OptionValue(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a Telegram Passport authorization form for sharing data with a service
    pub async fn get_passport_authorization_form<C: AsRef<GetPassportAuthorizationForm>>(
        &self,
        get_passport_authorization_form: C,
    ) -> RTDResult<PassportAuthorizationForm> {
        let extra = get_passport_authorization_form
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(get_passport_authorization_form.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PassportAuthorizationForm(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns already available Telegram Passport elements suitable for completing a Telegram Passport authorization form. Result can be received only once for each authorization form
    pub async fn get_passport_authorization_form_available_elements<
        C: AsRef<GetPassportAuthorizationFormAvailableElements>,
    >(
        &self,
        get_passport_authorization_form_available_elements: C,
    ) -> RTDResult<PassportElementsWithErrors> {
        let extra = get_passport_authorization_form_available_elements
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(get_passport_authorization_form_available_elements.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PassportElementsWithErrors(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns one of the available Telegram Passport elements
    pub async fn get_passport_element<C: AsRef<GetPassportElement>>(
        &self,
        get_passport_element: C,
    ) -> RTDResult<PassportElement> {
        let extra = get_passport_element
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_passport_element.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PassportElement(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the current state of 2-step verification
    pub async fn get_password_state<C: AsRef<GetPasswordState>>(
        &self,
        get_password_state: C,
    ) -> RTDResult<PasswordState> {
        let extra = get_password_state
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_password_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an invoice payment form. This method should be called when the user presses inlineKeyboardButtonBuy
    pub async fn get_payment_form<C: AsRef<GetPaymentForm>>(
        &self,
        get_payment_form: C,
    ) -> RTDResult<PaymentForm> {
        let extra = get_payment_form.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_payment_form.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PaymentForm(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a successful payment
    pub async fn get_payment_receipt<C: AsRef<GetPaymentReceipt>>(
        &self,
        get_payment_receipt: C,
    ) -> RTDResult<PaymentReceipt> {
        let extra = get_payment_receipt
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_payment_receipt.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PaymentReceipt(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns users voted for the specified option in a non-anonymous polls. For the optimal performance the number of returned users is chosen by the library
    pub async fn get_poll_voters<C: AsRef<GetPollVoters>>(
        &self,
        get_poll_voters: C,
    ) -> RTDResult<Users> {
        let extra = get_poll_voters.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_poll_voters.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Users(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an IETF language tag of the language preferred in the country, which should be used to fill native fields in Telegram Passport personal details. Returns a 404 error if unknown
    pub async fn get_preferred_country_language<C: AsRef<GetPreferredCountryLanguage>>(
        &self,
        get_preferred_country_language: C,
    ) -> RTDResult<Text> {
        let extra = get_preferred_country_language
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_preferred_country_language.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns list of proxies that are currently set up. Can be called before authorization
    pub async fn get_proxies<C: AsRef<GetProxies>>(&self, get_proxies: C) -> RTDResult<Proxies> {
        let extra = get_proxies.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_proxies.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Proxies(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an HTTPS link, which can be used to add a proxy. Available only for SOCKS5 and MTProto proxies. Can be called before authorization
    pub async fn get_proxy_link<C: AsRef<GetProxyLink>>(
        &self,
        get_proxy_link: C,
    ) -> RTDResult<Text> {
        let extra = get_proxy_link.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_proxy_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Text(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a public HTTPS link to a message. Available only for messages in supergroups and channels with a username
    pub async fn get_public_message_link<C: AsRef<GetPublicMessageLink>>(
        &self,
        get_public_message_link: C,
    ) -> RTDResult<PublicMessageLink> {
        let extra = get_public_message_link
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_public_message_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PublicMessageLink(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_push_receiver_id<C: AsRef<GetPushReceiverId>>(
        &self,
        get_push_receiver_id: C,
    ) -> RTDResult<PushReceiverId> {
        let extra = get_push_receiver_id
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_push_receiver_id.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PushReceiverId(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns up to 20 recently used inline bots in the order of their last usage
    pub async fn get_recent_inline_bots<C: AsRef<GetRecentInlineBots>>(
        &self,
        get_recent_inline_bots: C,
    ) -> RTDResult<Users> {
        let extra = get_recent_inline_bots
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_recent_inline_bots.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Users(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of recently used stickers
    pub async fn get_recent_stickers<C: AsRef<GetRecentStickers>>(
        &self,
        get_recent_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = get_recent_stickers
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_recent_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Stickers(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns t.me URLs recently visited by a newly registered user
    pub async fn get_recently_visited_t_me_urls<C: AsRef<GetRecentlyVisitedTMeUrls>>(
        &self,
        get_recently_visited_t_me_urls: C,
    ) -> RTDResult<TMeUrls> {
        let extra = get_recently_visited_t_me_urls
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_recently_visited_t_me_urls.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TMeUrls(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a 2-step verification recovery email address that was previously set up. This method can be used to verify a password provided by the user
    pub async fn get_recovery_email_address<C: AsRef<GetRecoveryEmailAddress>>(
        &self,
        get_recovery_email_address: C,
    ) -> RTDResult<RecoveryEmailAddress> {
        let extra = get_recovery_email_address
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_recovery_email_address.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::RecoveryEmailAddress(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a file by its remote ID; this is an offline request. Can be used to register a URL as a file for further uploading, or sending as a message. Even the request succeeds, the file can be used only if it is still accessible to the user. For example, if the file is from a message, then the message must be not deleted and accessible to the user. If the file database is disabled, then the corresponding object with the file must be preloaded by the client
    pub async fn get_remote_file<C: AsRef<GetRemoteFile>>(
        &self,
        get_remote_file: C,
    ) -> RTDResult<File> {
        let extra = get_remote_file.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_remote_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::File(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a message that is replied by given message
    pub async fn get_replied_message<C: AsRef<GetRepliedMessage>>(
        &self,
        get_replied_message: C,
    ) -> RTDResult<Message> {
        let extra = get_replied_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_replied_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns saved animations
    pub async fn get_saved_animations<C: AsRef<GetSavedAnimations>>(
        &self,
        get_saved_animations: C,
    ) -> RTDResult<Animations> {
        let extra = get_saved_animations
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_saved_animations.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Animations(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns saved order info, if any
    pub async fn get_saved_order_info<C: AsRef<GetSavedOrderInfo>>(
        &self,
        get_saved_order_info: C,
    ) -> RTDResult<OrderInfo> {
        let extra = get_saved_order_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_saved_order_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::OrderInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the notification settings for chats of a given type
    pub async fn get_scope_notification_settings<C: AsRef<GetScopeNotificationSettings>>(
        &self,
        get_scope_notification_settings: C,
    ) -> RTDResult<ScopeNotificationSettings> {
        let extra = get_scope_notification_settings
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(get_scope_notification_settings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ScopeNotificationSettings(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a secret chat by its identifier. This is an offline request
    pub async fn get_secret_chat<C: AsRef<GetSecretChat>>(
        &self,
        get_secret_chat: C,
    ) -> RTDResult<SecretChat> {
        let extra = get_secret_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_secret_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::SecretChat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns emoji corresponding to a sticker. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
    pub async fn get_sticker_emojis<C: AsRef<GetStickerEmojis>>(
        &self,
        get_sticker_emojis: C,
    ) -> RTDResult<Emojis> {
        let extra = get_sticker_emojis
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_sticker_emojis.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Emojis(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a sticker set by its identifier
    pub async fn get_sticker_set<C: AsRef<GetStickerSet>>(
        &self,
        get_sticker_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = get_sticker_set.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSet(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns stickers from the installed sticker sets that correspond to a given emoji. If the emoji is not empty, favorite and recently used stickers may also be returned
    pub async fn get_stickers<C: AsRef<GetStickers>>(
        &self,
        get_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = get_stickers.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Stickers(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns storage usage statistics. Can be called before authorization
    pub async fn get_storage_statistics<C: AsRef<GetStorageStatistics>>(
        &self,
        get_storage_statistics: C,
    ) -> RTDResult<StorageStatistics> {
        let extra = get_storage_statistics
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_storage_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StorageStatistics(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Quickly returns approximate storage usage statistics. Can be called before authorization
    pub async fn get_storage_statistics_fast<C: AsRef<GetStorageStatisticsFast>>(
        &self,
        get_storage_statistics_fast: C,
    ) -> RTDResult<StorageStatisticsFast> {
        let extra = get_storage_statistics_fast
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_storage_statistics_fast.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StorageStatisticsFast(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of basic group and supergroup chats, which can be used as a discussion group for a channel. Basic group chats need to be first upgraded to supergroups before they can be set as a discussion group
    pub async fn get_suitable_discussion_chats<C: AsRef<GetSuitableDiscussionChats>>(
        &self,
        get_suitable_discussion_chats: C,
    ) -> RTDResult<Chats> {
        let extra = get_suitable_discussion_chats
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_suitable_discussion_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a supergroup or a channel by its identifier. This is an offline request if the current user is not a bot
    pub async fn get_supergroup<C: AsRef<GetSupergroup>>(
        &self,
        get_supergroup: C,
    ) -> RTDResult<Supergroup> {
        let extra = get_supergroup.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_supergroup.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Supergroup(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns full information about a supergroup or a channel by its identifier, cached for up to 1 minute
    pub async fn get_supergroup_full_info<C: AsRef<GetSupergroupFullInfo>>(
        &self,
        get_supergroup_full_info: C,
    ) -> RTDResult<SupergroupFullInfo> {
        let extra = get_supergroup_full_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_supergroup_full_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::SupergroupFullInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about members or banned users in a supergroup or channel. Can be used only if SupergroupFullInfo.can_get_members == true; additionally, administrator privileges may be required for some filters
    pub async fn get_supergroup_members<C: AsRef<GetSupergroupMembers>>(
        &self,
        get_supergroup_members: C,
    ) -> RTDResult<ChatMembers> {
        let extra = get_supergroup_members
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_supergroup_members.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatMembers(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a user that can be contacted to get support
    pub async fn get_support_user<C: AsRef<GetSupportUser>>(
        &self,
        get_support_user: C,
    ) -> RTDResult<User> {
        let extra = get_support_user.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_support_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::User(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about the current temporary password
    pub async fn get_temporary_password_state<C: AsRef<GetTemporaryPasswordState>>(
        &self,
        get_temporary_password_state: C,
    ) -> RTDResult<TemporaryPasswordState> {
        let extra = get_temporary_password_state
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_temporary_password_state.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TemporaryPasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns all entities (mentions, hashtags, cashtags, bot commands, URLs, and email addresses) contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn get_text_entities<C: AsRef<GetTextEntities>>(
        &self,
        get_text_entities: C,
    ) -> RTDResult<TextEntities> {
        let extra = get_text_entities
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_text_entities.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TextEntities(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of frequently used chats. Supported only if the chat info database is enabled
    pub async fn get_top_chats<C: AsRef<GetTopChats>>(&self, get_top_chats: C) -> RTDResult<Chats> {
        let extra = get_top_chats.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_top_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of trending sticker sets
    pub async fn get_trending_sticker_sets<C: AsRef<GetTrendingStickerSets>>(
        &self,
        get_trending_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = get_trending_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_trending_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSets(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about a user by their identifier. This is an offline request if the current user is not a bot
    pub async fn get_user<C: AsRef<GetUser>>(&self, get_user: C) -> RTDResult<User> {
        let extra = get_user.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::User(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns full information about a user by their identifier
    pub async fn get_user_full_info<C: AsRef<GetUserFullInfo>>(
        &self,
        get_user_full_info: C,
    ) -> RTDResult<UserFullInfo> {
        let extra = get_user_full_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_user_full_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::UserFullInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the current privacy settings
    pub async fn get_user_privacy_setting_rules<C: AsRef<GetUserPrivacySettingRules>>(
        &self,
        get_user_privacy_setting_rules: C,
    ) -> RTDResult<UserPrivacySettingRules> {
        let extra = get_user_privacy_setting_rules
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_user_privacy_setting_rules.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::UserPrivacySettingRules(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the profile photos of a user. The result of this query may be outdated: some photos might have been deleted already
    pub async fn get_user_profile_photos<C: AsRef<GetUserProfilePhotos>>(
        &self,
        get_user_profile_photos: C,
    ) -> RTDResult<UserProfilePhotos> {
        let extra = get_user_profile_photos
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_user_profile_photos.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::UserProfilePhotos(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page
    pub async fn get_web_page_instant_view<C: AsRef<GetWebPageInstantView>>(
        &self,
        get_web_page_instant_view: C,
    ) -> RTDResult<WebPageInstantView> {
        let extra = get_web_page_instant_view
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_web_page_instant_view.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::WebPageInstantView(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview
    pub async fn get_web_page_preview<C: AsRef<GetWebPagePreview>>(
        &self,
        get_web_page_preview: C,
    ) -> RTDResult<WebPage> {
        let extra = get_web_page_preview
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(get_web_page_preview.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::WebPage(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds new contacts or edits existing contacts by their phone numbers; contacts' user identifiers are ignored
    pub async fn import_contacts<C: AsRef<ImportContacts>>(
        &self,
        import_contacts: C,
    ) -> RTDResult<ImportedContacts> {
        let extra = import_contacts.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(import_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ImportedContacts(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds current user as a new member to a chat. Private and secret chats can't be joined using this method
    pub async fn join_chat<C: AsRef<JoinChat>>(&self, join_chat: C) -> RTDResult<Ok> {
        let extra = join_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(join_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Uses an invite link to add the current user to the chat if possible. The new member will not be added until the chat state has been synchronized with the server
    pub async fn join_chat_by_invite_link<C: AsRef<JoinChatByInviteLink>>(
        &self,
        join_chat_by_invite_link: C,
    ) -> RTDResult<Chat> {
        let extra = join_chat_by_invite_link
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(join_chat_by_invite_link.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes current user from chat members. Private and secret chats can't be left using this method
    pub async fn leave_chat<C: AsRef<LeaveChat>>(&self, leave_chat: C) -> RTDResult<Ok> {
        let extra = leave_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(leave_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, updateAuthorizationState with authorizationStateClosed will be sent
    pub async fn log_out<C: AsRef<LogOut>>(&self, log_out: C) -> RTDResult<Ok> {
        let extra = log_out.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(log_out.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs TDLib that the chat is opened by the user. Many useful activities depend on the chat being opened or closed (e.g., in supergroups and channels all updates are received only for opened chats)
    pub async fn open_chat<C: AsRef<OpenChat>>(&self, open_chat: C) -> RTDResult<Ok> {
        let extra = open_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(open_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs TDLib that the message content has been opened (e.g., the user has opened a photo, video, document, location or venue, or has listened to an audio file or voice note message). An updateMessageContentOpened update will be generated if something has changed
    pub async fn open_message_content<C: AsRef<OpenMessageContent>>(
        &self,
        open_message_content: C,
    ) -> RTDResult<Ok> {
        let extra = open_message_content
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(open_message_content.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Optimizes storage usage, i.e. deletes some files and returns new storage usage statistics. Secret thumbnails can't be deleted
    pub async fn optimize_storage<C: AsRef<OptimizeStorage>>(
        &self,
        optimize_storage: C,
    ) -> RTDResult<StorageStatistics> {
        let extra = optimize_storage.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(optimize_storage.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StorageStatistics(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Parses Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities contained in the text. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn parse_text_entities<C: AsRef<ParseTextEntities>>(
        &self,
        parse_text_entities: C,
    ) -> RTDResult<FormattedText> {
        let extra = parse_text_entities
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(parse_text_entities.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::FormattedText(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Pins a message in a chat; requires can_pin_messages rights
    pub async fn pin_chat_message<C: AsRef<PinChatMessage>>(
        &self,
        pin_chat_message: C,
    ) -> RTDResult<Ok> {
        let extra = pin_chat_message.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(pin_chat_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Computes time needed to receive a response from a Telegram server through a proxy. Can be called before authorization
    pub async fn ping_proxy<C: AsRef<PingProxy>>(&self, ping_proxy: C) -> RTDResult<Seconds> {
        let extra = ping_proxy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(ping_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Seconds(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization
    pub async fn process_push_notification<C: AsRef<ProcessPushNotification>>(
        &self,
        process_push_notification: C,
    ) -> RTDResult<Ok> {
        let extra = process_push_notification
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(process_push_notification.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Marks all mentions in a chat as read
    pub async fn read_all_chat_mentions<C: AsRef<ReadAllChatMentions>>(
        &self,
        read_all_chat_mentions: C,
    ) -> RTDResult<Ok> {
        let extra = read_all_chat_mentions
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(read_all_chat_mentions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Reads a part of a file from the TDLib file cache and returns read bytes. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct read from the file
    pub async fn read_file_part<C: AsRef<ReadFilePart>>(
        &self,
        read_file_part: C,
    ) -> RTDResult<FilePart> {
        let extra = read_file_part.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(read_file_part.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::FilePart(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Recovers the password with a password recovery code sent to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn recover_authentication_password<C: AsRef<RecoverAuthenticationPassword>>(
        &self,
        recover_authentication_password: C,
    ) -> RTDResult<Ok> {
        let extra = recover_authentication_password
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(recover_authentication_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Recovers the password using a recovery code sent to an email address that was previously set up
    pub async fn recover_password<C: AsRef<RecoverPassword>>(
        &self,
        recover_password: C,
    ) -> RTDResult<PasswordState> {
        let extra = recover_password.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(recover_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Registers the currently used device for receiving push notifications. Returns a globally unique identifier of the push notification subscription
    pub async fn register_device<C: AsRef<RegisterDevice>>(
        &self,
        register_device: C,
    ) -> RTDResult<PushReceiverId> {
        let extra = register_device.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(register_device.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PushReceiverId(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Finishes user registration. Works only when the current authorization state is authorizationStateWaitRegistration
    pub async fn register_user<C: AsRef<RegisterUser>>(&self, register_user: C) -> RTDResult<Ok> {
        let extra = register_user.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(register_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes background from the list of installed backgrounds
    pub async fn remove_background<C: AsRef<RemoveBackground>>(
        &self,
        remove_background: C,
    ) -> RTDResult<Ok> {
        let extra = remove_background
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_background.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a chat action bar without any other action
    pub async fn remove_chat_action_bar<C: AsRef<RemoveChatActionBar>>(
        &self,
        remove_chat_action_bar: C,
    ) -> RTDResult<Ok> {
        let extra = remove_chat_action_bar
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_chat_action_bar.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes users from the contact list
    pub async fn remove_contacts<C: AsRef<RemoveContacts>>(
        &self,
        remove_contacts: C,
    ) -> RTDResult<Ok> {
        let extra = remove_contacts.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a sticker from the list of favorite stickers
    pub async fn remove_favorite_sticker<C: AsRef<RemoveFavoriteSticker>>(
        &self,
        remove_favorite_sticker: C,
    ) -> RTDResult<Ok> {
        let extra = remove_favorite_sticker
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_favorite_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes an active notification from notification list. Needs to be called only if the notification is removed by the current user
    pub async fn remove_notification<C: AsRef<RemoveNotification>>(
        &self,
        remove_notification: C,
    ) -> RTDResult<Ok> {
        let extra = remove_notification
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_notification.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a group of active notifications. Needs to be called only if the notification group is removed by the current user
    pub async fn remove_notification_group<C: AsRef<RemoveNotificationGroup>>(
        &self,
        remove_notification_group: C,
    ) -> RTDResult<Ok> {
        let extra = remove_notification_group
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_notification_group.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a proxy server. Can be called before authorization
    pub async fn remove_proxy<C: AsRef<RemoveProxy>>(&self, remove_proxy: C) -> RTDResult<Ok> {
        let extra = remove_proxy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a hashtag from the list of recently used hashtags
    pub async fn remove_recent_hashtag<C: AsRef<RemoveRecentHashtag>>(
        &self,
        remove_recent_hashtag: C,
    ) -> RTDResult<Ok> {
        let extra = remove_recent_hashtag
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_recent_hashtag.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a sticker from the list of recently used stickers
    pub async fn remove_recent_sticker<C: AsRef<RemoveRecentSticker>>(
        &self,
        remove_recent_sticker: C,
    ) -> RTDResult<Ok> {
        let extra = remove_recent_sticker
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_recent_sticker.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a chat from the list of recently found chats
    pub async fn remove_recently_found_chat<C: AsRef<RemoveRecentlyFoundChat>>(
        &self,
        remove_recently_found_chat: C,
    ) -> RTDResult<Ok> {
        let extra = remove_recently_found_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_recently_found_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes an animation from the list of saved animations
    pub async fn remove_saved_animation<C: AsRef<RemoveSavedAnimation>>(
        &self,
        remove_saved_animation: C,
    ) -> RTDResult<Ok> {
        let extra = remove_saved_animation
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_saved_animation.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a sticker from the set to which it belongs; for bots only. The sticker set must have been created by the bot
    pub async fn remove_sticker_from_set<C: AsRef<RemoveStickerFromSet>>(
        &self,
        remove_sticker_from_set: C,
    ) -> RTDResult<Ok> {
        let extra = remove_sticker_from_set
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_sticker_from_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a chat from the list of frequently used chats. Supported only if the chat info database is enabled
    pub async fn remove_top_chat<C: AsRef<RemoveTopChat>>(
        &self,
        remove_top_chat: C,
    ) -> RTDResult<Ok> {
        let extra = remove_top_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(remove_top_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the order of installed sticker sets
    pub async fn reorder_installed_sticker_sets<C: AsRef<ReorderInstalledStickerSets>>(
        &self,
        reorder_installed_sticker_sets: C,
    ) -> RTDResult<Ok> {
        let extra = reorder_installed_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(reorder_installed_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Reports a chat to the Telegram moderators. Supported only for supergroups, channels, or private chats with bots, since other chats can't be checked by moderators, or when the report is done from the chat action bar
    pub async fn report_chat<C: AsRef<ReportChat>>(&self, report_chat: C) -> RTDResult<Ok> {
        let extra = report_chat.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(report_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Reports some messages from a user in a supergroup as spam; requires administrator rights in the supergroup
    pub async fn report_supergroup_spam<C: AsRef<ReportSupergroupSpam>>(
        &self,
        report_supergroup_spam: C,
    ) -> RTDResult<Ok> {
        let extra = report_supergroup_spam
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(report_supergroup_spam.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Requests to send a password recovery code to an email address that was previously set up. Works only when the current authorization state is authorizationStateWaitPassword
    pub async fn request_authentication_password_recovery<
        C: AsRef<RequestAuthenticationPasswordRecovery>,
    >(
        &self,
        request_authentication_password_recovery: C,
    ) -> RTDResult<Ok> {
        let extra = request_authentication_password_recovery
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(request_authentication_password_recovery.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Requests to send a password recovery code to an email address that was previously set up
    pub async fn request_password_recovery<C: AsRef<RequestPasswordRecovery>>(
        &self,
        request_password_recovery: C,
    ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
        let extra = request_password_recovery
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(request_password_recovery.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::EmailAddressAuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Requests QR code authentication by scanning a QR code on another logged in device. Works only when the current authorization state is authorizationStateWaitPhoneNumber
    pub async fn request_qr_code_authentication<C: AsRef<RequestQrCodeAuthentication>>(
        &self,
        request_qr_code_authentication: C,
    ) -> RTDResult<Ok> {
        let extra = request_qr_code_authentication
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(request_qr_code_authentication.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Re-sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitCode and the next_code_type of the result is not null
    pub async fn resend_authentication_code<C: AsRef<ResendAuthenticationCode>>(
        &self,
        resend_authentication_code: C,
    ) -> RTDResult<Ok> {
        let extra = resend_authentication_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(resend_authentication_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Re-sends the authentication code sent to confirm a new phone number for the user. Works only if the previously received authenticationCodeInfo next_code_type was not null
    pub async fn resend_change_phone_number_code<C: AsRef<ResendChangePhoneNumberCode>>(
        &self,
        resend_change_phone_number_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = resend_change_phone_number_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(resend_change_phone_number_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Re-sends the code to verify an email address to be added to a user's Telegram Passport
    pub async fn resend_email_address_verification_code<
        C: AsRef<ResendEmailAddressVerificationCode>,
    >(
        &self,
        resend_email_address_verification_code: C,
    ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
        let extra = resend_email_address_verification_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(resend_email_address_verification_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::EmailAddressAuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Resends messages which failed to send. Can be called only for messages for which messageSendingStateFailed.can_retry is true and after specified in messageSendingStateFailed.retry_after time passed. If a message is re-sent, the corresponding failed to send message is deleted. Returns the sent messages in the same order as the message identifiers passed in message_ids. If a message can't be re-sent, null will be returned instead of the message
    pub async fn resend_messages<C: AsRef<ResendMessages>>(
        &self,
        resend_messages: C,
    ) -> RTDResult<Messages> {
        let extra = resend_messages.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(resend_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Resends phone number confirmation code
    pub async fn resend_phone_number_confirmation_code<
        C: AsRef<ResendPhoneNumberConfirmationCode>,
    >(
        &self,
        resend_phone_number_confirmation_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = resend_phone_number_confirmation_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(resend_phone_number_confirmation_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Re-sends the code to verify a phone number to be added to a user's Telegram Passport
    pub async fn resend_phone_number_verification_code<
        C: AsRef<ResendPhoneNumberVerificationCode>,
    >(
        &self,
        resend_phone_number_verification_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra = resend_phone_number_verification_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(resend_phone_number_verification_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Resends the 2-step verification recovery email address verification code
    pub async fn resend_recovery_email_address_code<C: AsRef<ResendRecoveryEmailAddressCode>>(
        &self,
        resend_recovery_email_address_code: C,
    ) -> RTDResult<PasswordState> {
        let extra =
            resend_recovery_email_address_code
                .as_ref()
                .extra()
                .ok_or(RTDError::Internal(
                    "invalid tdlib response type, not have `extra` field",
                ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(resend_recovery_email_address_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Resets all notification settings to their default values. By default, all chats are unmuted, the sound is set to "default" and message previews are shown
    pub async fn reset_all_notification_settings<C: AsRef<ResetAllNotificationSettings>>(
        &self,
        reset_all_notification_settings: C,
    ) -> RTDResult<Ok> {
        let extra = reset_all_notification_settings
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(reset_all_notification_settings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Resets list of installed backgrounds to its default value
    pub async fn reset_backgrounds<C: AsRef<ResetBackgrounds>>(
        &self,
        reset_backgrounds: C,
    ) -> RTDResult<Ok> {
        let extra = reset_backgrounds
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(reset_backgrounds.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Resets all network data usage statistics to zero. Can be called before authorization
    pub async fn reset_network_statistics<C: AsRef<ResetNetworkStatistics>>(
        &self,
        reset_network_statistics: C,
    ) -> RTDResult<Ok> {
        let extra = reset_network_statistics
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(reset_network_statistics.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Saves application log event on the server. Can be called before authorization
    pub async fn save_application_log_event<C: AsRef<SaveApplicationLogEvent>>(
        &self,
        save_application_log_event: C,
    ) -> RTDResult<Ok> {
        let extra = save_application_log_event
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(save_application_log_event.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for a background by its name
    pub async fn search_background<C: AsRef<SearchBackground>>(
        &self,
        search_background: C,
    ) -> RTDResult<Background> {
        let extra = search_background
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_background.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Background(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for call messages. Returns the results in reverse chronological order (i. e., in order of decreasing message_id). For optimal performance the number of returned messages is chosen by the library
    pub async fn search_call_messages<C: AsRef<SearchCallMessages>>(
        &self,
        search_call_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_call_messages
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_call_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for a specified query in the first name, last name and username of the members of a specified chat. Requires administrator rights in channels
    pub async fn search_chat_members<C: AsRef<SearchChatMembers>>(
        &self,
        search_chat_members: C,
    ) -> RTDResult<ChatMembers> {
        let extra = search_chat_members
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_chat_members.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatMembers(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for messages with given words in the chat. Returns the results in reverse chronological order, i.e. in order of decreasing message_id. Cannot be used in secret chats with a non-empty query (searchSecretMessages should be used instead), or without an enabled message database. For optimal performance the number of returned messages is chosen by the library
    pub async fn search_chat_messages<C: AsRef<SearchChatMessages>>(
        &self,
        search_chat_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_chat_messages
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_chat_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns information about the recent locations of chat members that were sent to the chat. Returns up to 1 location message per user
    pub async fn search_chat_recent_location_messages<
        C: AsRef<SearchChatRecentLocationMessages>,
    >(
        &self,
        search_chat_recent_location_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_chat_recent_location_messages
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(search_chat_recent_location_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for the specified query in the title and username of already known chats, this is an offline request. Returns chats in the order seen in the chat list
    pub async fn search_chats<C: AsRef<SearchChats>>(&self, search_chats: C) -> RTDResult<Chats> {
        let extra = search_chats.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns a list of users and location-based supergroups nearby. The list of users nearby will be updated for 60 seconds after the request by the updates updateUsersNearby. The request should be sent again every 25 seconds with adjusted location to not miss new chats
    pub async fn search_chats_nearby<C: AsRef<SearchChatsNearby>>(
        &self,
        search_chats_nearby: C,
    ) -> RTDResult<ChatsNearby> {
        let extra = search_chats_nearby
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_chats_nearby.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ChatsNearby(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for the specified query in the title and username of already known chats via request to the server. Returns chats in the order seen in the chat list
    pub async fn search_chats_on_server<C: AsRef<SearchChatsOnServer>>(
        &self,
        search_chats_on_server: C,
    ) -> RTDResult<Chats> {
        let extra = search_chats_on_server
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_chats_on_server.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for the specified query in the first names, last names and usernames of the known user contacts
    pub async fn search_contacts<C: AsRef<SearchContacts>>(
        &self,
        search_contacts: C,
    ) -> RTDResult<Users> {
        let extra = search_contacts.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_contacts.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Users(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for emojis by keywords. Supported only if the file database is enabled
    pub async fn search_emojis<C: AsRef<SearchEmojis>>(
        &self,
        search_emojis: C,
    ) -> RTDResult<Emojis> {
        let extra = search_emojis.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_emojis.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Emojis(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for recently used hashtags by their prefix
    pub async fn search_hashtags<C: AsRef<SearchHashtags>>(
        &self,
        search_hashtags: C,
    ) -> RTDResult<Hashtags> {
        let extra = search_hashtags.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_hashtags.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Hashtags(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for installed sticker sets by looking for specified query in their title and name
    pub async fn search_installed_sticker_sets<C: AsRef<SearchInstalledStickerSets>>(
        &self,
        search_installed_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = search_installed_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_installed_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSets(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for messages in all chats except secret chats. Returns the results in reverse chronological order (i.e., in order of decreasing (date, chat_id, message_id)). For optimal performance the number of returned messages is chosen by the library
    pub async fn search_messages<C: AsRef<SearchMessages>>(
        &self,
        search_messages: C,
    ) -> RTDResult<Messages> {
        let extra = search_messages.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches a public chat by its username. Currently only private chats, supergroups and channels can be public. Returns the chat if found; otherwise an error is returned
    pub async fn search_public_chat<C: AsRef<SearchPublicChat>>(
        &self,
        search_public_chat: C,
    ) -> RTDResult<Chat> {
        let extra = search_public_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_public_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches public chats by looking for specified query in their username and title. Currently only private chats, supergroups and channels can be public. Returns a meaningful number of results. Returns nothing if the length of the searched username prefix is less than 5. Excludes private chats with contacts and chats from the chat list from the results
    pub async fn search_public_chats<C: AsRef<SearchPublicChats>>(
        &self,
        search_public_chats: C,
    ) -> RTDResult<Chats> {
        let extra = search_public_chats
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_public_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chats(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for messages in secret chats. Returns the results in reverse chronological order. For optimal performance the number of returned messages is chosen by the library
    pub async fn search_secret_messages<C: AsRef<SearchSecretMessages>>(
        &self,
        search_secret_messages: C,
    ) -> RTDResult<FoundMessages> {
        let extra = search_secret_messages
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_secret_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::FoundMessages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for a sticker set by its name
    pub async fn search_sticker_set<C: AsRef<SearchStickerSet>>(
        &self,
        search_sticker_set: C,
    ) -> RTDResult<StickerSet> {
        let extra = search_sticker_set
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSet(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for ordinary sticker sets by looking for specified query in their title and name. Excludes installed sticker sets from the results
    pub async fn search_sticker_sets<C: AsRef<SearchStickerSets>>(
        &self,
        search_sticker_sets: C,
    ) -> RTDResult<StickerSets> {
        let extra = search_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::StickerSets(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Searches for stickers from public sticker sets that correspond to a given emoji
    pub async fn search_stickers<C: AsRef<SearchStickers>>(
        &self,
        search_stickers: C,
    ) -> RTDResult<Stickers> {
        let extra = search_stickers.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(search_stickers.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Stickers(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Invites a bot to a chat (if it is not yet a member) and sends it the /start command. Bots can't be invited to a private chat other than the chat with the bot. Bots can't be invited to channels (although they can be added as admins) and secret chats. Returns the sent message
    pub async fn send_bot_start_message<C: AsRef<SendBotStartMessage>>(
        &self,
        send_bot_start_message: C,
    ) -> RTDResult<Message> {
        let extra = send_bot_start_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_bot_start_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends debug information for a call
    pub async fn send_call_debug_information<C: AsRef<SendCallDebugInformation>>(
        &self,
        send_call_debug_information: C,
    ) -> RTDResult<Ok> {
        let extra = send_call_debug_information
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_call_debug_information.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a call rating
    pub async fn send_call_rating<C: AsRef<SendCallRating>>(
        &self,
        send_call_rating: C,
    ) -> RTDResult<Ok> {
        let extra = send_call_rating.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_call_rating.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a notification about user activity in a chat
    pub async fn send_chat_action<C: AsRef<SendChatAction>>(
        &self,
        send_chat_action: C,
    ) -> RTDResult<Ok> {
        let extra = send_chat_action.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_chat_action.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a notification about a screenshot taken in a chat. Supported only in private and secret chats
    pub async fn send_chat_screenshot_taken_notification<
        C: AsRef<SendChatScreenshotTakenNotification>,
    >(
        &self,
        send_chat_screenshot_taken_notification: C,
    ) -> RTDResult<Ok> {
        let extra = send_chat_screenshot_taken_notification
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(send_chat_screenshot_taken_notification.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the current TTL setting (sets a new self-destruct timer) in a secret chat and sends the corresponding message
    pub async fn send_chat_set_ttl_message<C: AsRef<SendChatSetTtlMessage>>(
        &self,
        send_chat_set_ttl_message: C,
    ) -> RTDResult<Message> {
        let extra = send_chat_set_ttl_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_chat_set_ttl_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a custom request; for bots only
    pub async fn send_custom_request<C: AsRef<SendCustomRequest>>(
        &self,
        send_custom_request: C,
    ) -> RTDResult<CustomRequestResult> {
        let extra = send_custom_request
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_custom_request.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::CustomRequestResult(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a code to verify an email address to be added to a user's Telegram Passport
    pub async fn send_email_address_verification_code<
        C: AsRef<SendEmailAddressVerificationCode>,
    >(
        &self,
        send_email_address_verification_code: C,
    ) -> RTDResult<EmailAddressAuthenticationCodeInfo> {
        let extra = send_email_address_verification_code
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(send_email_address_verification_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::EmailAddressAuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends the result of an inline query as a message. Returns the sent message. Always clears a chat draft message
    pub async fn send_inline_query_result_message<C: AsRef<SendInlineQueryResultMessage>>(
        &self,
        send_inline_query_result_message: C,
    ) -> RTDResult<Message> {
        let extra = send_inline_query_result_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(send_inline_query_result_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a message. Returns the sent message
    pub async fn send_message<C: AsRef<SendMessage>>(&self, send_message: C) -> RTDResult<Message> {
        let extra = send_message.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends messages grouped together into an album. Currently only photo and video messages can be grouped into an album. Returns sent messages
    pub async fn send_message_album<C: AsRef<SendMessageAlbum>>(
        &self,
        send_message_album: C,
    ) -> RTDResult<Messages> {
        let extra = send_message_album
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_message_album.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Messages(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a Telegram Passport authorization form, effectively sharing data with the service. This method must be called after getPassportAuthorizationFormAvailableElements if some previously available elements need to be used
    pub async fn send_passport_authorization_form<C: AsRef<SendPassportAuthorizationForm>>(
        &self,
        send_passport_authorization_form: C,
    ) -> RTDResult<Ok> {
        let extra = send_passport_authorization_form
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(send_passport_authorization_form.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a filled-out payment form to the bot for final verification
    pub async fn send_payment_form<C: AsRef<SendPaymentForm>>(
        &self,
        send_payment_form: C,
    ) -> RTDResult<PaymentResult> {
        let extra = send_payment_form
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(send_payment_form.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PaymentResult(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends phone number confirmation code. Should be called when user presses "https://t.me/confirmphone?phone=*******&hash=**********" or "tg://confirmphone?phone=*******&hash=**********" link
    pub async fn send_phone_number_confirmation_code<C: AsRef<SendPhoneNumberConfirmationCode>>(
        &self,
        send_phone_number_confirmation_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra =
            send_phone_number_confirmation_code
                .as_ref()
                .extra()
                .ok_or(RTDError::Internal(
                    "invalid tdlib response type, not have `extra` field",
                ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(send_phone_number_confirmation_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a code to verify a phone number to be added to a user's Telegram Passport
    pub async fn send_phone_number_verification_code<C: AsRef<SendPhoneNumberVerificationCode>>(
        &self,
        send_phone_number_verification_code: C,
    ) -> RTDResult<AuthenticationCodeInfo> {
        let extra =
            send_phone_number_verification_code
                .as_ref()
                .extra()
                .ok_or(RTDError::Internal(
                    "invalid tdlib response type, not have `extra` field",
                ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(send_phone_number_verification_code.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::AuthenticationCodeInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the period of inactivity after which the account of the current user will automatically be deleted
    pub async fn set_account_ttl<C: AsRef<SetAccountTtl>>(
        &self,
        set_account_ttl: C,
    ) -> RTDResult<Ok> {
        let extra = set_account_ttl.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_account_ttl.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Succeeds after a specified amount of time has passed. Can be called before authorization. Can be called before initialization
    pub async fn set_alarm<C: AsRef<SetAlarm>>(&self, set_alarm: C) -> RTDResult<Ok> {
        let extra = set_alarm.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_alarm.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the phone number of the user and sends an authentication code to the user. Works only when the current authorization state is authorizationStateWaitPhoneNumber, or if there is no pending authentication query and the current authorization state is authorizationStateWaitCode, authorizationStateWaitRegistration, or authorizationStateWaitPassword
    pub async fn set_authentication_phone_number<C: AsRef<SetAuthenticationPhoneNumber>>(
        &self,
        set_authentication_phone_number: C,
    ) -> RTDResult<Ok> {
        let extra = set_authentication_phone_number
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(set_authentication_phone_number.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets auto-download settings
    pub async fn set_auto_download_settings<C: AsRef<SetAutoDownloadSettings>>(
        &self,
        set_auto_download_settings: C,
    ) -> RTDResult<Ok> {
        let extra = set_auto_download_settings
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_auto_download_settings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the background selected by the user; adds background to the list of installed backgrounds
    pub async fn set_background<C: AsRef<SetBackground>>(
        &self,
        set_background: C,
    ) -> RTDResult<Background> {
        let extra = set_background.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_background.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Background(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the bio of the current user
    pub async fn set_bio<C: AsRef<SetBio>>(&self, set_bio: C) -> RTDResult<Ok> {
        let extra = set_bio.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_bio.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only
    pub async fn set_bot_updates_status<C: AsRef<SetBotUpdatesStatus>>(
        &self,
        set_bot_updates_status: C,
    ) -> RTDResult<Ok> {
        let extra = set_bot_updates_status
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_bot_updates_status.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Moves a chat to a different chat list. Current chat list of the chat must ne non-null
    pub async fn set_chat_chat_list<C: AsRef<SetChatChatList>>(
        &self,
        set_chat_chat_list: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_chat_list
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_chat_list.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes client data associated with a chat
    pub async fn set_chat_client_data<C: AsRef<SetChatClientData>>(
        &self,
        set_chat_client_data: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_client_data
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_client_data.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes information about a chat. Available for basic groups, supergroups, and channels. Requires can_change_info rights
    pub async fn set_chat_description<C: AsRef<SetChatDescription>>(
        &self,
        set_chat_description: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_description
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_description.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the discussion group of a channel chat; requires can_change_info rights in the channel if it is specified
    pub async fn set_chat_discussion_group<C: AsRef<SetChatDiscussionGroup>>(
        &self,
        set_chat_discussion_group: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_discussion_group
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_discussion_group.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the draft message in a chat
    pub async fn set_chat_draft_message<C: AsRef<SetChatDraftMessage>>(
        &self,
        set_chat_draft_message: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_draft_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_draft_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the location of a chat. Available only for some location-based supergroups, use supergroupFullInfo.can_set_location to check whether the method is allowed to use
    pub async fn set_chat_location<C: AsRef<SetChatLocation>>(
        &self,
        set_chat_location: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_location
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_location.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the status of a chat member, needs appropriate privileges. This function is currently not suitable for adding new members to the chat and transferring chat ownership; instead, use addChatMember or transferChatOwnership. The chat member status will not be changed until it has been synchronized with the server
    pub async fn set_chat_member_status<C: AsRef<SetChatMemberStatus>>(
        &self,
        set_chat_member_status: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_member_status
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_member_status.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the notification settings of a chat. Notification settings of a chat with the current user (Saved Messages) can't be changed
    pub async fn set_chat_notification_settings<C: AsRef<SetChatNotificationSettings>>(
        &self,
        set_chat_notification_settings: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_notification_settings
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_notification_settings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the chat members permissions. Supported only for basic groups and supergroups. Requires can_restrict_members administrator right
    pub async fn set_chat_permissions<C: AsRef<SetChatPermissions>>(
        &self,
        set_chat_permissions: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_permissions
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_permissions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the photo of a chat. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The photo will not be changed before request to the server has been completed
    pub async fn set_chat_photo<C: AsRef<SetChatPhoto>>(&self, set_chat_photo: C) -> RTDResult<Ok> {
        let extra = set_chat_photo.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_photo.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the slow mode delay of a chat. Available only for supergroups; requires can_restrict_members rights
    pub async fn set_chat_slow_mode_delay<C: AsRef<SetChatSlowModeDelay>>(
        &self,
        set_chat_slow_mode_delay: C,
    ) -> RTDResult<Ok> {
        let extra = set_chat_slow_mode_delay
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_slow_mode_delay.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the chat title. Supported only for basic groups, supergroups and channels. Requires can_change_info rights. The title will not be changed until the request to the server has been completed
    pub async fn set_chat_title<C: AsRef<SetChatTitle>>(&self, set_chat_title: C) -> RTDResult<Ok> {
        let extra = set_chat_title.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_chat_title.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds or changes a custom local language pack to the current localization target
    pub async fn set_custom_language_pack<C: AsRef<SetCustomLanguagePack>>(
        &self,
        set_custom_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = set_custom_language_pack
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_custom_language_pack.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds, edits or deletes a string in a custom local language pack. Can be called before authorization
    pub async fn set_custom_language_pack_string<C: AsRef<SetCustomLanguagePackString>>(
        &self,
        set_custom_language_pack_string: C,
    ) -> RTDResult<Ok> {
        let extra = set_custom_language_pack_string
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(set_custom_language_pack_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the database encryption key. Usually the encryption key is never changed and is stored in some OS keychain
    pub async fn set_database_encryption_key<C: AsRef<SetDatabaseEncryptionKey>>(
        &self,
        set_database_encryption_key: C,
    ) -> RTDResult<Ok> {
        let extra = set_database_encryption_key
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_database_encryption_key.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs TDLib on a file generation progress
    pub async fn set_file_generation_progress<C: AsRef<SetFileGenerationProgress>>(
        &self,
        set_file_generation_progress: C,
    ) -> RTDResult<Ok> {
        let extra = set_file_generation_progress
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_file_generation_progress.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Updates the game score of the specified user in the game; for bots only
    pub async fn set_game_score<C: AsRef<SetGameScore>>(
        &self,
        set_game_score: C,
    ) -> RTDResult<Message> {
        let extra = set_game_score.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_game_score.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Message(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Updates the game score of the specified user in a game; for bots only
    pub async fn set_inline_game_score<C: AsRef<SetInlineGameScore>>(
        &self,
        set_inline_game_score: C,
    ) -> RTDResult<Ok> {
        let extra = set_inline_game_score
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_inline_game_score.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets new log stream for internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn set_log_stream<C: AsRef<SetLogStream>>(&self, set_log_stream: C) -> RTDResult<Ok> {
        let extra = set_log_stream.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_log_stream.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the verbosity level for a specified TDLib internal log tag. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn set_log_tag_verbosity_level<C: AsRef<SetLogTagVerbosityLevel>>(
        &self,
        set_log_tag_verbosity_level: C,
    ) -> RTDResult<Ok> {
        let extra = set_log_tag_verbosity_level
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_log_tag_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the verbosity level of the internal logging of TDLib. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn set_log_verbosity_level<C: AsRef<SetLogVerbosityLevel>>(
        &self,
        set_log_verbosity_level: C,
    ) -> RTDResult<Ok> {
        let extra = set_log_verbosity_level
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_log_verbosity_level.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the first and last name of the current user. If something changes, updateUser will be sent
    pub async fn set_name<C: AsRef<SetName>>(&self, set_name: C) -> RTDResult<Ok> {
        let extra = set_name.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_name.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics
    pub async fn set_network_type<C: AsRef<SetNetworkType>>(
        &self,
        set_network_type: C,
    ) -> RTDResult<Ok> {
        let extra = set_network_type.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_network_type.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the value of an option. (Check the list of available options on https://core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization
    pub async fn set_option<C: AsRef<SetOption>>(&self, set_option: C) -> RTDResult<Ok> {
        let extra = set_option.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_option.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
    pub async fn set_passport_element<C: AsRef<SetPassportElement>>(
        &self,
        set_passport_element: C,
    ) -> RTDResult<PassportElement> {
        let extra = set_passport_element
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_passport_element.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PassportElement(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed
    pub async fn set_passport_element_errors<C: AsRef<SetPassportElementErrors>>(
        &self,
        set_passport_element_errors: C,
    ) -> RTDResult<Ok> {
        let extra = set_passport_element_errors
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_passport_element_errors.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the password for the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed
    pub async fn set_password<C: AsRef<SetPassword>>(
        &self,
        set_password: C,
    ) -> RTDResult<PasswordState> {
        let extra = set_password.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_password.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the order of pinned chats
    pub async fn set_pinned_chats<C: AsRef<SetPinnedChats>>(
        &self,
        set_pinned_chats: C,
    ) -> RTDResult<Ok> {
        let extra = set_pinned_chats.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_pinned_chats.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the user answer to a poll. A poll in quiz mode can be answered only once
    pub async fn set_poll_answer<C: AsRef<SetPollAnswer>>(
        &self,
        set_poll_answer: C,
    ) -> RTDResult<Ok> {
        let extra = set_poll_answer.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_poll_answer.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Uploads a new profile photo for the current user. If something changes, updateUser will be sent
    pub async fn set_profile_photo<C: AsRef<SetProfilePhoto>>(
        &self,
        set_profile_photo: C,
    ) -> RTDResult<Ok> {
        let extra = set_profile_photo
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_profile_photo.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the 2-step verification recovery email address of the user. If a new recovery email address is specified, then the change will not be applied until the new recovery email address is confirmed. If new_recovery_email_address is the same as the email address that is currently set up, this call succeeds immediately and aborts all other requests waiting for an email confirmation
    pub async fn set_recovery_email_address<C: AsRef<SetRecoveryEmailAddress>>(
        &self,
        set_recovery_email_address: C,
    ) -> RTDResult<PasswordState> {
        let extra = set_recovery_email_address
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_recovery_email_address.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::PasswordState(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes notification settings for chats of a given type
    pub async fn set_scope_notification_settings<C: AsRef<SetScopeNotificationSettings>>(
        &self,
        set_scope_notification_settings: C,
    ) -> RTDResult<Ok> {
        let extra = set_scope_notification_settings
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(set_scope_notification_settings.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the position of a sticker in the set to which it belongs; for bots only. The sticker set must have been created by the bot
    pub async fn set_sticker_position_in_set<C: AsRef<SetStickerPositionInSet>>(
        &self,
        set_sticker_position_in_set: C,
    ) -> RTDResult<Ok> {
        let extra = set_sticker_position_in_set
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_sticker_position_in_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the sticker set of a supergroup; requires can_change_info rights
    pub async fn set_supergroup_sticker_set<C: AsRef<SetSupergroupStickerSet>>(
        &self,
        set_supergroup_sticker_set: C,
    ) -> RTDResult<Ok> {
        let extra = set_supergroup_sticker_set
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_supergroup_sticker_set.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the username of a supergroup or channel, requires owner privileges in the supergroup or channel
    pub async fn set_supergroup_username<C: AsRef<SetSupergroupUsername>>(
        &self,
        set_supergroup_username: C,
    ) -> RTDResult<Ok> {
        let extra = set_supergroup_username
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_supergroup_username.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
    pub async fn set_tdlib_parameters<C: AsRef<SetTdlibParameters>>(
        &self,
        set_tdlib_parameters: C,
    ) -> RTDResult<Ok> {
        let extra = set_tdlib_parameters
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_tdlib_parameters.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes user privacy settings
    pub async fn set_user_privacy_setting_rules<C: AsRef<SetUserPrivacySettingRules>>(
        &self,
        set_user_privacy_setting_rules: C,
    ) -> RTDResult<Ok> {
        let extra = set_user_privacy_setting_rules
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_user_privacy_setting_rules.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the username of the current user. If something changes, updateUser will be sent
    pub async fn set_username<C: AsRef<SetUsername>>(&self, set_username: C) -> RTDResult<Ok> {
        let extra = set_username.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(set_username.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Shares the phone number of the current user with a mutual contact. Supposed to be called when the user clicks on chatActionBarSharePhoneNumber
    pub async fn share_phone_number<C: AsRef<SharePhoneNumber>>(
        &self,
        share_phone_number: C,
    ) -> RTDResult<Ok> {
        let extra = share_phone_number
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(share_phone_number.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Stops a poll. A poll in a message can be stopped when the message has can_be_edited flag set
    pub async fn stop_poll<C: AsRef<StopPoll>>(&self, stop_poll: C) -> RTDResult<Ok> {
        let extra = stop_poll.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(stop_poll.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization
    pub async fn synchronize_language_pack<C: AsRef<SynchronizeLanguagePack>>(
        &self,
        synchronize_language_pack: C,
    ) -> RTDResult<Ok> {
        let extra = synchronize_language_pack
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(synchronize_language_pack.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Terminates all other sessions of the current user
    pub async fn terminate_all_other_sessions<C: AsRef<TerminateAllOtherSessions>>(
        &self,
        terminate_all_other_sessions: C,
    ) -> RTDResult<Ok> {
        let extra = terminate_all_other_sessions
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(terminate_all_other_sessions.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Terminates a session of the current user
    pub async fn terminate_session<C: AsRef<TerminateSession>>(
        &self,
        terminate_session: C,
    ) -> RTDResult<Ok> {
        let extra = terminate_session
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(terminate_session.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the received bytes; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_bytes<C: AsRef<TestCallBytes>>(
        &self,
        test_call_bytes: C,
    ) -> RTDResult<TestBytes> {
        let extra = test_call_bytes.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_call_bytes.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TestBytes(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Does nothing; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_empty<C: AsRef<TestCallEmpty>>(
        &self,
        test_call_empty: C,
    ) -> RTDResult<Ok> {
        let extra = test_call_empty.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_call_empty.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the received string; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_string<C: AsRef<TestCallString>>(
        &self,
        test_call_string: C,
    ) -> RTDResult<TestString> {
        let extra = test_call_string.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_call_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TestString(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_int<C: AsRef<TestCallVectorInt>>(
        &self,
        test_call_vector_int: C,
    ) -> RTDResult<TestVectorInt> {
        let extra = test_call_vector_int
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_call_vector_int.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TestVectorInt(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_int_object<C: AsRef<TestCallVectorIntObject>>(
        &self,
        test_call_vector_int_object: C,
    ) -> RTDResult<TestVectorIntObject> {
        let extra = test_call_vector_int_object
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_call_vector_int_object.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TestVectorIntObject(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_string<C: AsRef<TestCallVectorString>>(
        &self,
        test_call_vector_string: C,
    ) -> RTDResult<TestVectorString> {
        let extra = test_call_vector_string
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_call_vector_string.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TestVectorString(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_call_vector_string_object<C: AsRef<TestCallVectorStringObject>>(
        &self,
        test_call_vector_string_object: C,
    ) -> RTDResult<TestVectorStringObject> {
        let extra = test_call_vector_string_object
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_call_vector_string_object.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TestVectorStringObject(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Forces an updates.getDifference call to the Telegram servers; for testing only
    pub async fn test_get_difference<C: AsRef<TestGetDifference>>(
        &self,
        test_get_difference: C,
    ) -> RTDResult<Ok> {
        let extra = test_get_difference
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_get_difference.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization
    pub async fn test_network<C: AsRef<TestNetwork>>(&self, test_network: C) -> RTDResult<Ok> {
        let extra = test_network.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_network.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Sends a simple network request to the Telegram servers via proxy; for testing only. Can be called before authorization
    pub async fn test_proxy<C: AsRef<TestProxy>>(&self, test_proxy: C) -> RTDResult<Ok> {
        let extra = test_proxy.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_proxy.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the specified error and ensures that the Error object is used; for testing only. This is an offline method. Can be called before authorization. Can be called synchronously
    pub async fn test_return_error<C: AsRef<TestReturnError>>(
        &self,
        test_return_error: C,
    ) -> RTDResult<Error> {
        let extra = test_return_error
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_return_error.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Error(v) => Ok(v),

                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Returns the squared received number; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_square_int<C: AsRef<TestSquareInt>>(
        &self,
        test_square_int: C,
    ) -> RTDResult<TestInt> {
        let extra = test_square_int.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_square_int.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::TestInt(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
    pub async fn test_use_update<C: AsRef<TestUseUpdate>>(
        &self,
        test_use_update: C,
    ) -> RTDResult<Update> {
        let extra = test_use_update.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(test_use_update.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Update(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the value of the default disable_notification parameter, used when a message is sent to a chat
    pub async fn toggle_chat_default_disable_notification<
        C: AsRef<ToggleChatDefaultDisableNotification>,
    >(
        &self,
        toggle_chat_default_disable_notification: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_chat_default_disable_notification
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(toggle_chat_default_disable_notification.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the marked as unread state of a chat
    pub async fn toggle_chat_is_marked_as_unread<C: AsRef<ToggleChatIsMarkedAsUnread>>(
        &self,
        toggle_chat_is_marked_as_unread: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_chat_is_marked_as_unread
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(toggle_chat_is_marked_as_unread.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the pinned state of a chat. You can pin up to GetOption("pinned_chat_count_max")/GetOption("pinned_archived_chat_count_max") non-secret chats and the same number of secret chats in the main/archive chat list
    pub async fn toggle_chat_is_pinned<C: AsRef<ToggleChatIsPinned>>(
        &self,
        toggle_chat_is_pinned: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_chat_is_pinned
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(toggle_chat_is_pinned.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Toggles whether the message history of a supergroup is available to new members; requires can_change_info rights
    pub async fn toggle_supergroup_is_all_history_available<
        C: AsRef<ToggleSupergroupIsAllHistoryAvailable>,
    >(
        &self,
        toggle_supergroup_is_all_history_available: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_supergroup_is_all_history_available
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(toggle_supergroup_is_all_history_available.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Toggles sender signatures messages sent in a channel; requires can_change_info rights
    pub async fn toggle_supergroup_sign_messages<C: AsRef<ToggleSupergroupSignMessages>>(
        &self,
        toggle_supergroup_sign_messages: C,
    ) -> RTDResult<Ok> {
        let extra = toggle_supergroup_sign_messages
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(toggle_supergroup_sign_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Changes the owner of a chat. The current user must be a current owner of the chat. Use the method canTransferOwnership to check whether the ownership can be transferred from the current session. Available only for supergroups and channel chats
    pub async fn transfer_chat_ownership<C: AsRef<TransferChatOwnership>>(
        &self,
        transfer_chat_ownership: C,
    ) -> RTDResult<Ok> {
        let extra = transfer_chat_ownership
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(transfer_chat_ownership.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes a user from the blacklist
    pub async fn unblock_user<C: AsRef<UnblockUser>>(&self, unblock_user: C) -> RTDResult<Ok> {
        let extra = unblock_user.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(unblock_user.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Removes the pinned message from a chat; requires can_pin_messages rights in the group or channel
    pub async fn unpin_chat_message<C: AsRef<UnpinChatMessage>>(
        &self,
        unpin_chat_message: C,
    ) -> RTDResult<Ok> {
        let extra = unpin_chat_message
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(unpin_chat_message.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires creator privileges. Deactivates the original basic group
    pub async fn upgrade_basic_group_chat_to_supergroup_chat<
        C: AsRef<UpgradeBasicGroupChatToSupergroupChat>,
    >(
        &self,
        upgrade_basic_group_chat_to_supergroup_chat: C,
    ) -> RTDResult<Chat> {
        let extra = upgrade_basic_group_chat_to_supergroup_chat
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api
            .send(upgrade_basic_group_chat_to_supergroup_chat.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Chat(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Asynchronously uploads a file to the cloud without sending it in a message. updateFile will be used to notify about upload progress and successful completion of the upload. The file will not have a persistent remote identifier until it will be sent in a message
    pub async fn upload_file<C: AsRef<UploadFile>>(&self, upload_file: C) -> RTDResult<File> {
        let extra = upload_file.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(upload_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::File(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Uploads a PNG image with a sticker; for bots only; returns the uploaded file
    pub async fn upload_sticker_file<C: AsRef<UploadStickerFile>>(
        &self,
        upload_sticker_file: C,
    ) -> RTDResult<File> {
        let extra = upload_sticker_file
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(upload_sticker_file.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::File(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Validates the order information provided by a user and returns the available shipping options for a flexible invoice
    pub async fn validate_order_info<C: AsRef<ValidateOrderInfo>>(
        &self,
        validate_order_info: C,
    ) -> RTDResult<ValidatedOrderInfo> {
        let extra = validate_order_info
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(validate_order_info.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::ValidatedOrderInfo(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs TDLib that messages are being viewed by the user. Many useful activities depend on whether the messages are currently being viewed or not (e.g., marking messages as read, incrementing a view counter, updating a view counter, removing deleted messages in supergroups and channels)
    pub async fn view_messages<C: AsRef<ViewMessages>>(&self, view_messages: C) -> RTDResult<Ok> {
        let extra = view_messages.as_ref().extra().ok_or(RTDError::Internal(
            "invalid tdlib response type, not have `extra` field",
        ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(view_messages.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Informs the server that some trending sticker sets have been viewed by the user
    pub async fn view_trending_sticker_sets<C: AsRef<ViewTrendingStickerSets>>(
        &self,
        view_trending_sticker_sets: C,
    ) -> RTDResult<Ok> {
        let extra = view_trending_sticker_sets
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(view_trending_sticker_sets.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    } // Writes a part of a generated file. This method is intended to be used only if the client has no direct access to TDLib's file system, because it is usually slower than a direct write to the destination file
    pub async fn write_generated_file_part<C: AsRef<WriteGeneratedFilePart>>(
        &self,
        write_generated_file_part: C,
    ) -> RTDResult<Ok> {
        let extra = write_generated_file_part
            .as_ref()
            .extra()
            .ok_or(RTDError::Internal(
                "invalid tdlib response type, not have `extra` field",
            ))?;
        let signal = OBSERVER.subscribe(&extra);
        self.raw_api.send(write_generated_file_part.as_ref())?;
        let received = signal.await;
        OBSERVER.unsubscribe(&extra);
        match received {
            Err(_) => Err(RTDError::Internal("receiver already closed")),
            Ok(v) => match v {
                TdType::Ok(v) => Ok(v),
                TdType::Error(v) => Err(RTDError::TdlibError(v.message().clone())),
                _ => {
                    error!("invalid response received: {:?}", v);
                    Err(RTDError::Internal("receive invalid response"))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::api::TdLibClient;
    use crate::client::client::{Client, ConsoleAuthStateHandler};
    use crate::errors::RTDResult;
    use crate::types::{
        Chats, RFunction, RObject, SearchPublicChats, TdlibParameters, UpdateAuthorizationState,
    };
    use std::time::Duration;
    use tokio::sync::mpsc;
    use tokio::time::timeout;

    #[derive(Clone)]
    struct MockedRawApi {
        to_receive: Option<String>,
    }

    impl MockedRawApi {
        pub fn set_to_receive(&mut self, value: String) {
            trace!("delayed to receive: {}", value);
            self.to_receive = Some(value);
        }

        pub fn new() -> Self {
            Self { to_receive: None }
        }
    }

    impl TdLibClient for MockedRawApi {
        fn send<Fnc: RFunction>(&self, _fnc: Fnc) -> RTDResult<()> {
            Ok(())
        }

        fn receive(&self, timeout: f64) -> Option<String> {
            std::thread::sleep(Duration::from_secs(timeout as u64));
            if self.to_receive.is_none() {
                panic!("value to receive not set");
            }
            self.to_receive.clone()
        }

        fn execute<Fnc: RFunction>(&self, _fnc: Fnc) -> RTDResult<Option<String>> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_request_flow() {
        // here we just test request-response flow with SearchPublicChats request
        env_logger::init();
        let mut mocked_raw_api = MockedRawApi::new();

        let search_req = SearchPublicChats::builder().build();

        let chats: serde_json::Value = serde_json::from_str(
            Chats::builder()
                .chat_ids(vec![1, 2, 3])
                .build()
                .to_json()
                .unwrap()
                .as_str(),
        )
        .unwrap();
        let mut chats_object = chats.as_object().unwrap().clone();
        chats_object.insert(
            "@extra".to_string(),
            serde_json::Value::String(search_req.extra().unwrap()),
        );
        let to_receive = serde_json::to_string(&chats_object).unwrap();
        mocked_raw_api.set_to_receive(to_receive);

        let client = Client::new(
            mocked_raw_api.clone(),
            ConsoleAuthStateHandler::default(),
            TdlibParameters::builder().build(),
            None,
            2.0,
        );

        let (sx, _rx) = mpsc::channel::<UpdateAuthorizationState>(10);
        let _updates_handle = client.init_updates_task(sx);
        trace!("chats objects: {:?}", chats_object);
        match timeout(
            Duration::from_secs(10),
            client.api().search_public_chats(search_req),
        )
        .await
        {
            Err(_) => panic!("did not receive response within 1 s"),
            Ok(Err(e)) => panic!("{}", e),
            Ok(Ok(result)) => assert_eq!(result.chat_ids(), &vec![1, 2, 3]),
        }
    }
}
