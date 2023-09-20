//! WiP

use core::ops::Deref;
use core::pin::Pin;

use sys::traits::AsRaw;
use sys::ffi::LCDSprite;

use crate::AnySprite;
use crate::Sprite;
use crate::SpriteApi;
use crate::SpriteRef;
use crate::api;


impl<UD, Api: api::Api, const FOD: bool> Sprite<UD, Api, FOD> {
	/// Wraps into sprite that supports fn callbacks.
	///
	/// But this doesn’t supports static stateless handlers.
	pub fn with_callbacks(self) -> SpriteCb<UD, Api, FOD> { SpriteCb::new(self) }
}


#[derive(Debug)]
#[repr(transparent)]
pub struct SpriteCb<Userdata = (), Api: api::Api = api::Default, const FREE_ON_DROP: bool = true>(Sprite<Userdata, Api, FREE_ON_DROP>);


impl<UD, Api: api::Api, const FOD: bool> SpriteCb<UD, Api, FOD> {
	fn new(sprite: Sprite<UD, Api, FOD>) -> Self {
		// reassign userdata
		let ud = sprite.take_userdata();
		// TODO: init ud layout/structure. set ud into it.

		sprite.set_userdata(*ud.unwrap());
		//
		todo!()
	}

	// TODO: override get/set/take_userdata methods


	pub fn set_update_function<F>(&self, on_update: Pin<F>)
		where F: FnMut(SpriteRef) {
		let f = self.0.api_ref().set_update_function();
		// unsafe { f(self.0.as_raw(), ) }
	}


	pub fn set_draw_function(&self) {
		let f = self.0.api_ref().set_draw_function();
		// unsafe { f(self.0.as_raw(), ) }
	}

	pub fn set_collision_response_function(&self) {
		let f = self.0.api_ref().set_collision_response_function();
		// 	unsafe { f(self.0.as_raw(), ) }
		todo!()
	}
}


impl<UD, Api: api::Api, const FOD: bool> Deref for SpriteCb<UD, Api, FOD> {
	type Target = Sprite<UD, Api, FOD>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<UD, Api: api::Api, const FOD: bool> AnySprite for SpriteCb<UD, Api, FOD>
	where SpriteCb<UD, Api, FOD>: AnySprite
{
}

impl<UD, Api: api::Api, const FOD: bool> SpriteApi for SpriteCb<UD, Api, FOD>
	where SpriteCb<UD, Api, FOD>: SpriteApi,
	      SpriteCb<UD, Api, FOD>: SpriteApi<Api = Api>
{
	type Api = <SpriteCb<UD, Api, FOD> as SpriteApi>::Api;

	fn api(&self) -> Self::Api
		where Self::Api: Copy {
		use api::Api;
		self.0.api()
	}

	fn api_ref(&self) -> &Self::Api {
		use api::Api;
		self.0.api_ref()
	}
}

impl<UD, Api: api::Api, const FOD: bool> AsRaw for SpriteCb<UD, Api, FOD>
	where SpriteCb<UD, Api, FOD>: AsRaw<Type = LCDSprite>
{
	type Type = <SpriteCb<UD, Api, FOD> as AsRaw>::Type;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.0.as_raw() }
}
