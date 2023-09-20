use core::marker::PhantomData;
use core::ops::Deref;

use sys::ffi::LCDSprite;
use sys::traits::AsRaw;

use crate::{Sprite, SpriteApi, TypedSprite, SpriteRef, AnySprite, SharedSprite};
use crate::api::{self, Api};


// This is mostly probably should be implemented for OwnedSprite only.
impl<UD, Api: api::Api, const FOD: bool> Sprite<UD, Api, FOD> {
	/// Sets the update function for the this sprite.
	pub fn into_update_handler<T: SpriteUpdate<Userdata = UD>>(self) -> Handle<FOD, Self, T>
		where T::Api: Default {
		Handle::new(self)
	}
}


pub trait SpriteUpdate: Sized + TypedSprite
	where Self: From<SpriteRef> {
	fn on_update(sprite: &Handle<false, SharedSprite<Self::Userdata, Self::Api>, Self>);

	unsafe extern "C" fn proxy(sprite: *mut LCDSprite)
		where Self::Api: Default {
		Self::on_update(&Handle::new_unchanged(SpriteRef::from(sprite).into()))
	}
}


pub struct Handle<const FOD: bool, T, H>(pub(super) T, PhantomData<H>)
	where T: TypedSprite + Sized,
	      H: SpriteUpdate;

impl<const FOD: bool, T, H> AnySprite for Handle<FOD, T, H>
	where T: TypedSprite + Sized,
	      H: SpriteUpdate,
	      T: AnySprite
{
}
impl<const FOD: bool, T, H> SpriteApi for Handle<FOD, T, H>
	where T: TypedSprite + Sized,
	      H: SpriteUpdate,
	      T: SpriteApi
{
	type Api = <T as SpriteApi>::Api;

	fn api(&self) -> Self::Api
		where Self::Api: Copy {
		self.0.api()
	}

	fn api_ref(&self) -> &Self::Api { self.0.api_ref() }
}

impl<const FOD: bool, T, H> AsRaw for Handle<FOD, T, H>
	where T: TypedSprite + Sized,
	      H: SpriteUpdate,
	      T: AsRaw
{
	type Type = <T as AsRaw>::Type;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.0.as_raw() }
}

impl<const FOD: bool, T, H> Handle<FOD, T, H>
	where T: TypedSprite + Sized,
	      H: SpriteUpdate
{
	/// - Unregister inner callback for sprite
	/// - Unwrap, return the underlying sprite
	/// - Remove the update handler
	#[must_use = "Sprite"]
	pub fn into_inner(self) -> T {
		let ptr = unsafe { self.0.as_raw() };
		let f = self.0.api_ref().set_update_function();
		unsafe { f(ptr, None) };
		self.0
	}
}


impl<const FOD: bool, T, H> AsRef<Sprite<T::Userdata, T::Api, FOD>> for Handle<FOD, T, H>
	where T: TypedSprite + AsRef<Sprite<T::Userdata, T::Api, FOD>>,
	      H: SpriteUpdate
{
	fn as_ref(&self) -> &Sprite<T::Userdata, T::Api, FOD> { self.0.as_ref() }
}


impl<const FOD: bool, T, H> AsMut<Sprite<T::Userdata, T::Api, FOD>> for Handle<FOD, T, H>
	where T: TypedSprite + AsMut<Sprite<T::Userdata, T::Api, FOD>>,
	      H: SpriteUpdate
{
	fn as_mut(&mut self) -> &mut Sprite<T::Userdata, T::Api, FOD> { self.0.as_mut() }
}


impl<const FOD: bool, T, H> Deref for Handle<FOD, T, H>
	where T: TypedSprite + AsRef<Sprite<T::Userdata, T::Api, FOD>>,
	      H: SpriteUpdate
{
	type Target = Sprite<T::Userdata, T::Api, FOD>;
	fn deref(&self) -> &Self::Target { self.0.as_ref() }
}


impl<const FOD: bool, T, H> Handle<FOD, T, H>
	where T: TypedSprite + SpriteApi,
	      H: SpriteUpdate
{
	pub(super) fn new(sprite: T) -> Self
		where H::Api: Default {
		let f = sprite.api_ref().set_update_function();
		unsafe { f(sprite.as_raw(), Some(H::proxy)) };
		Self::new_unchanged(sprite)
	}

	fn new_unchanged(sprite: T) -> Self { Self(sprite, PhantomData::<H>) }
}


pub(crate) mod l2 {
	use core::ops::Deref;
	use core::marker::PhantomData;

	use sys::traits::AsRaw;

	use crate::AnySprite;
	use crate::Sprite;
	use crate::SpriteApi;
	use crate::TypedSprite;
	use crate::api;
	use crate::callback::draw;
	use crate::callback::collision;

	use super::SpriteUpdate;


	impl<UD, Api, const FOD: bool, H> collision::Handle<FOD, Sprite<UD, Api, FOD>, H>
		where Api: api::Api,
		      H: collision::SpriteCollisionResponse
	{
		/// Sets the update function for the this sprite.
		pub fn into_update_handler<T: SpriteUpdate<Userdata = UD>>(
			self)
			-> Handle<FOD, Sprite<UD, Api, FOD>, Self, T>
			where T::Api: Default {
			Handle::new(self)
		}
	}

	impl<UD, Api, const FOD: bool, H> draw::Handle<FOD, Sprite<UD, Api, FOD>, H>
		where Api: api::Api,
		      H: draw::SpriteDraw
	{
		/// Sets the update function for the this sprite.
		pub fn into_update_handler<T: SpriteUpdate<Userdata = UD>>(
			self)
			-> Handle<FOD, Sprite<UD, Api, FOD>, Self, T>
			where T::Api: Default {
			Handle::new(self)
		}
	}

	impl<UD, Api, const FOD: bool, H, H0>
		draw::l2::Handle<FOD, Sprite<UD, Api, FOD>, collision::Handle<FOD, Sprite<UD, Api, FOD>, H0>, H>
		where Api: api::Api,
		      H: draw::SpriteDraw,
		      H0: collision::SpriteCollisionResponse
	{
		/// Sets the update function for the this sprite.
		pub fn into_update_handler<T: SpriteUpdate<Userdata = UD>>(
			self)
			-> Handle<FOD, Sprite<UD, Api, FOD>, Self, T>
			where T::Api: Default {
			Handle::new(self)
		}
	}

	impl<UD, Api, const FOD: bool, H, H0>
		collision::l2::Handle<FOD, Sprite<UD, Api, FOD>, draw::Handle<FOD, Sprite<UD, Api, FOD>, H0>, H>
		where Api: api::Api,
		      H: collision::SpriteCollisionResponse,
		      H0: draw::SpriteDraw
	{
		/// Sets the update function for the this sprite.
		pub fn into_update_handler<T: SpriteUpdate<Userdata = UD>>(
			self)
			-> Handle<FOD, Sprite<UD, Api, FOD>, Self, T>
			where T::Api: Default {
			Handle::new(self)
		}
	}


	pub struct Handle<const FOD: bool, Sp, T, H>(pub(super) T, PhantomData<Sp>, PhantomData<H>)
		where T: Sized,
		      Sp: TypedSprite,
		      H: SpriteUpdate;


	impl<const FOD: bool, Sp, T, H> AnySprite for Handle<FOD, Sp, T, H>
		where Sp: TypedSprite,
		      T: AnySprite,
		      H: SpriteUpdate
	{
	}
	impl<const FOD: bool, Sp, T, H> SpriteApi for Handle<FOD, Sp, T, H>
		where Sp: TypedSprite,
		      T: SpriteApi,
		      H: SpriteUpdate
	{
		type Api = <T as SpriteApi>::Api;

		fn api(&self) -> Self::Api
			where Self::Api: Copy {
			self.0.api()
		}

		fn api_ref(&self) -> &Self::Api { self.0.api_ref() }
	}

	impl<const FOD: bool, Sp, T, H> AsRaw for Handle<FOD, Sp, T, H>
		where Sp: TypedSprite,
		      T: AsRaw,
		      H: SpriteUpdate
	{
		type Type = <T as AsRaw>::Type;
		unsafe fn as_raw(&self) -> *mut Self::Type { self.0.as_raw() }
	}


	impl<const FOD: bool, Sp, T, H> Handle<FOD, Sp, T, H>
		where T: AsRef<Sp>,
		      Sp: TypedSprite,
		      H: SpriteUpdate
	{
		/// - Unregister inner callback for sprite
		/// - Unwrap, return the underlying sprite
		/// - Remove the collision response handler
		#[must_use = "Sprite"]
		pub fn into_inner(self) -> T {
			use crate::api::Api;

			let ptr = unsafe { self.0.as_ref().as_raw() };
			let f = self.0.as_ref().api_ref().set_update_function();
			unsafe { f(ptr, None) };
			self.0
		}
	}


	impl<const FOD: bool, Sp, T, H> AsRef<Sprite<Sp::Userdata, Sp::Api, FOD>> for Handle<FOD, Sp, T, H>
		where T: AsRef<Sprite<Sp::Userdata, Sp::Api, FOD>>,
		      Sp: TypedSprite,
		      H: SpriteUpdate
	{
		fn as_ref(&self) -> &Sprite<Sp::Userdata, Sp::Api, FOD> { self.0.as_ref() }
	}


	impl<const FOD: bool, Sp, T, H> Deref for Handle<FOD, Sp, T, H>
		where T: AsRef<Sprite<Sp::Userdata, Sp::Api, FOD>>,
		      Sp: TypedSprite,
		      H: SpriteUpdate
	{
		type Target = Sprite<Sp::Userdata, Sp::Api, FOD>;
		fn deref(&self) -> &Self::Target { self.0.as_ref() }
	}


	impl<const FOD: bool, Sp, T, H> Handle<FOD, Sp, T, H>
		where T: AsRef<Sp>,
		      Sp: TypedSprite + SpriteApi,
		      H: SpriteUpdate
	{
		pub(super) fn new(sprite: T) -> Self
			where H::Api: Default {
			use crate::api::Api;

			let f = sprite.as_ref().api_ref().set_update_function();
			unsafe { f(sprite.as_ref().as_raw(), Some(H::proxy)) };
			Self::new_unchanged(sprite)
		}

		fn new_unchanged(sprite: T) -> Self { Self(sprite, PhantomData::<Sp>, PhantomData::<H>) }
	}
}
