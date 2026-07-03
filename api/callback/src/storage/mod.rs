pub mod key;

/// Associated KV storage
pub mod associate;

/// Simple storage for K
pub mod kmap;

/// Implementations
pub(crate) mod tmap;


/// Storage-access to get value of `T` for given type.
///
/// Static, type is the key.
pub trait Store<T: ?Sized> {
	/// True if storage __for the `T`__ is empty.
	fn is_empty() -> bool;

	/// Store the `v` into the storage.
	fn set(v: T);
	// Ref to the stored value `T`.
	fn get<'t>() -> Option<&'t T>;
	// Mutable ref to the stored value `T`.
	fn get_mut<'t>() -> Option<&'t mut T>;
	/// Remove from the storage and return.
	fn take() -> Option<T>
		where T: Sized;

	/// Remove from the storage and drop,
	/// returns `true` if the value was and so removed.
	///
	/// Removes value without checking if the value type, depending on implementation.
	fn remove() -> bool;
}


pub(crate) mod ext {
	use super::Store;

	trait StoreTy<T>: Store<T> {
		type Ty: Store<T>;
	}
	impl<T, S: Store<T>> StoreTy<T> for S {
		type Ty = Self;
	}

	#[allow(private_bounds)]
	pub(crate) trait StoreExt<T>: StoreTy<T> {
		fn is_empty(&self) -> bool { <Self::Ty as Store<T>>::is_empty() }
		fn write(&self, v: T) { <Self::Ty as Store<T>>::set(v) }
		fn get<'t>(&self) -> Option<&'t T> { <Self::Ty as Store<T>>::get() }
		fn get_mut<'t>(&self) -> Option<&'t mut T> { <Self::Ty as Store<T>>::get_mut() }
		fn take(&self) -> Option<T> { <Self::Ty as Store<T>>::take() }
	}

	impl<T, S: Store<T>> StoreExt<T> for S {}
}
