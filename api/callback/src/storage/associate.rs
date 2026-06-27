use super::key::{Key, AsKey};


pub(crate) trait Associated<K>
	where for<'t> &'t K: AsKey {
	#[inline(always)]
	fn key(k: &K) -> Key { k.key() }

	fn set<V: 'static>(k: &K, v: V);
	fn get<'t, V: 'static>(k: &K) -> Option<&'t V>;
	fn get_mut<'t, V: 'static>(k: &K) -> Option<&'t mut V>;
	/// Remove from the storage and return.
	fn take<V>(k: &K) -> Option<V>
		where V: Sized + 'static;

	/// True if storage __for the key `T`__ is empty.
	fn is_empty(k: &K) -> bool;

	/// True if storage __for the key `T` and value type `V`__ is empty.
	fn is_empty_for<V: 'static>(k: &K) -> bool;
}
