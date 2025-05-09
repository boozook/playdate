use core::marker::PhantomData;


/// Static storage for one unique type.
pub struct Storage<T>(PhantomData<T>);
