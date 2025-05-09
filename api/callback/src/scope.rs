use crate::arg;
use crate::proxy;
use crate::storage;


/// Immediate blocking execution in current thread.
#[deprecated = "Not implemented and useless at all."]
pub(crate) struct Immediate;
#[allow(deprecated)]
impl Scope for Immediate {
	type Storage<Key> = ();
	type Adapter<In, Out> = ();
	type Proxy<C, CIn, COut, R, RIn, ROut> = ();
}

/// Deferred execution in main thread.
pub struct Deferred;
impl Scope for Deferred {
	type Adapter<In, Out> = arg::default::Into<In, Out>;
	type Storage<Key> = storage::tmap::Static;
	type Proxy<C, CIn, COut, R, RIn, ROut> = proxy::default::Default<R, Self::Storage<R>, Self::Adapter<CIn, RIn>>;
}


/// Deferred execution in main thread, unique like singleton subscribtion.
pub(crate) struct Unique<const ID: u64>;
impl<const ID: u64> Scope for Unique<ID> {
	type Adapter<In, Out> = <Deferred as Scope>::Adapter<In, Out>;
	type Storage<Key> = storage::unique::Storage<Key>;
	type Proxy<C, CIn, COut, R, RIn, ROut> = ();
}

/// Deferred execution in other thread.
/// Used in sound and network.
pub(crate) struct Async;
impl Scope for Async
// could be with non_lifetime_binders: where for <In, Out> Self::Adapter<In, Out>::Params: 'static
{
	type Storage<Key> = ();
	type Adapter<In, Out> = ();
	type Proxy<C, CIn, COut, R, RIn, ROut> = ();
}


/// C-side executions scope.
///
/// See predefined scopes: [`Immediate`], [`Deferred`], [`Unique`], [`Async`] _or implement your own_.
pub trait Scope {
	/// Storage for the function as `Key`
	type Storage<Key>;

	/// Proxy & deploy Implementation.
	// type Proxy<C, CIn, COut, R, RIn, ROut> = proxy::default::Default<R, Self::Storage<R>, Self::Adapter<CIn, RIn>>;
	type Proxy<C, CIn, COut, R, RIn, ROut>;

	/// Arguments converter.
	// type Adapter<In, Out> = arg::default::Into<In, Out>;
	type Adapter<In, Out>;
}
