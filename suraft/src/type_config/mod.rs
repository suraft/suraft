//! Define the configuration of types used by the SuRaft, such as [`NodeId`],
//! log [`Entry`], etc.
//!
//! [`NodeId`]: `TypeConfig::NodeId`
//! [`Entry`]: `TypeConfig::Entry`

pub mod async_runtime;
pub mod declare_suraft_types;
#[cfg(test)]
mod declare_suraft_types_test;
pub(crate) mod type_config_ext;

use std::fmt::Debug;

pub use async_runtime::AsyncRuntime;
pub use async_runtime::MpscUnbounded;
pub use async_runtime::OneshotSender;
pub use type_config_ext::TypeConfigExt;

use crate::app::AppData;
use crate::suraft::responder::Responder;
use crate::OptionalSend;
use crate::OptionalSync;

/// Configuration of types used by the [`SuRaft`] core engine.
///
/// The (empty) implementation structure defines request/response types, node ID
/// type and the like. Refer to the documentation of associated types for more
/// information.
///
/// ## Note
///
/// Since Rust cannot automatically infer traits for various inner types using
/// this config type as a parameter, this trait simply uses all the traits
/// required for various types as its supertraits as a workaround. To ease the
/// declaration, the macro `declare_suraft_types` is provided, which can be used
/// to declare the type easily.
///
/// Example:
/// ```ignore
/// suraft::declare_suraft_types!(
///    pub TypeConfig:
///        AppData      = ClientRequest,
///        NodeId       = u64,
///        Node         = suraft::BasicNode,
///        Entry        = suraft::Entry<TypeConfig>,
///        AsyncRuntime = suraft::TokioRuntime,
/// );
/// ```
/// [`SuRaft`]: crate::SuRaft
pub trait TypeConfig:
    Sized
    + OptionalSend
    + OptionalSync
    + Debug
    + Clone
    + Copy
    + Default
    + Eq
    + PartialEq
    + Ord
    + PartialOrd
    + 'static
{
    /// Application-specific request data passed to the state machine.
    type AppData: AppData;

    /// Asynchronous runtime type.
    type AsyncRuntime: AsyncRuntime;

    /// Send the response or error of a client write request([`WriteResult`]).
    ///
    /// For example, return [`WriteResult`] the to the caller of
    /// [`SuRaft::write`], or send to some application defined channel.
    ///
    /// [`SuRaft::write`]: `crate::suraft::SuRaft::client_write`
    /// [`WriteResult`]: `crate::suraft::message::WriteResult`
    type Responder: Responder<Self>;
}

#[allow(dead_code)]
/// Type alias for types used in `TypeConfig`.
///
/// Alias are enabled by feature flag [`type-alias`].
///
/// [`type-alias`]: crate::docs::feature_flags#feature-flag-type-alias
pub mod alias {
    use crate::async_runtime::watch;
    use crate::async_runtime::Mpsc;
    use crate::async_runtime::MpscUnbounded;
    use crate::async_runtime::Oneshot;
    use crate::suraft::responder::Responder;
    use crate::type_config::AsyncRuntime;
    use crate::TypeConfig;

    pub type AppDataOf<C> = <C as TypeConfig>::AppData;
    pub type AsyncRuntimeOf<C> = <C as TypeConfig>::AsyncRuntime;
    pub type ResponderOf<C> = <C as TypeConfig>::Responder;
    pub type ResponderReceiverOf<C> =
        <ResponderOf<C> as Responder<C>>::Receiver;

    type Rt<C> = AsyncRuntimeOf<C>;

    pub type JoinErrorOf<C> = <Rt<C> as AsyncRuntime>::JoinError;
    pub type JoinHandleOf<C, T> = <Rt<C> as AsyncRuntime>::JoinHandle<T>;
    pub type SleepOf<C> = <Rt<C> as AsyncRuntime>::Sleep;
    pub type InstantOf<C> = <Rt<C> as AsyncRuntime>::Instant;
    pub type TimeoutErrorOf<C> = <Rt<C> as AsyncRuntime>::TimeoutError;
    pub type TimeoutOf<C, R, F> = <Rt<C> as AsyncRuntime>::Timeout<R, F>;

    pub type OneshotOf<C> = <Rt<C> as AsyncRuntime>::Oneshot;
    pub type OneshotSenderOf<C, T> = <OneshotOf<C> as Oneshot>::Sender<T>;
    pub type OneshotReceiverErrorOf<C> =
        <OneshotOf<C> as Oneshot>::ReceiverError;
    pub type OneshotReceiverOf<C, T> = <OneshotOf<C> as Oneshot>::Receiver<T>;

    pub type MpscOf<C> = <Rt<C> as AsyncRuntime>::Mpsc;

    // MPSC bounded
    type MpscB<C> = MpscOf<C>;

    pub type MpscSenderOf<C, T> = <MpscB<C> as Mpsc>::Sender<T>;
    pub type MpscReceiverOf<C, T> = <MpscB<C> as Mpsc>::Receiver<T>;
    pub type MpscWeakSenderOf<C, T> = <MpscB<C> as Mpsc>::WeakSender<T>;

    pub type MpscUnboundedOf<C> = <Rt<C> as AsyncRuntime>::MpscUnbounded;

    // MPSC unbounded
    type MpscUB<C> = MpscUnboundedOf<C>;

    pub type MpscUnboundedSenderOf<C, T> =
        <MpscUB<C> as MpscUnbounded>::Sender<T>;
    pub type MpscUnboundedReceiverOf<C, T> =
        <MpscUB<C> as MpscUnbounded>::Receiver<T>;
    pub type MpscUnboundedWeakSenderOf<C, T> =
        <MpscUB<C> as MpscUnbounded>::WeakSender<T>;

    pub type WatchOf<C> = <Rt<C> as AsyncRuntime>::Watch;
    pub type WatchSenderOf<C, T> = <WatchOf<C> as watch::Watch>::Sender<T>;
    pub type WatchReceiverOf<C, T> = <WatchOf<C> as watch::Watch>::Receiver<T>;

    pub type MutexOf<C, T> = <Rt<C> as AsyncRuntime>::Mutex<T>;

    // Usually used types
    pub type SerdeInstantOf<C> = crate::metrics::SerdeInstant<InstantOf<C>>;
}