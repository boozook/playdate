use sys::ffi::Pattern;

pub mod gfxp;


/// Half of a pattern. It could be body or mask.
pub type Half = [u8; 8_usize];
pub type Body = Half;
pub type Mask = Half;

/// Opaque half of a pattern.
pub const OPAQUE: Half = [0xFF; 8];
/// Transparent half of a pattern.
pub const TRANSPARENT: Half = [0; 8];


pub const fn split(pat: Pattern) -> (Body, Mask) { unsafe { core::mem::transmute(pat) } }

pub const fn sptit_ref(pat: &Pattern) -> &(Body, Mask) { unsafe { core::mem::transmute(pat) } }


/// Make new pattern with inverted body of the `pat` and its mask.
pub const fn invert_body(pat: Pattern) -> Pattern {
	let (body, mask) = split(pat);
	let body = invert(body);
	unsafe { core::mem::transmute((body, mask)) }
}

/// Make new pattern with inverted mask of the `pat` and its body.
pub const fn invert_mask(pat: Pattern) -> Pattern {
	let (body, mask) = split(pat);
	let mask = invert(mask);
	unsafe { core::mem::transmute((body, mask)) }
}

/// Binary inver every bit of the `pat`.
pub const fn invert<const LEN: usize>(mut pat: [u8; LEN]) -> [u8; LEN] {
	let mut i = 0;
	while i < LEN {
		pat[i] ^= 255_u8;
		i += 1;
	}
	pat
}

/// Fill the `pat` with given `value`.
pub const fn fill<const LEN: usize>(mut pat: [u8; LEN], value: u8) -> [u8; LEN] {
	let mut i = 0;
	while i < LEN {
		pat[i] = value;
		i += 1;
	}
	pat
}

/// Swap the mask of the `pat` to [`OPAQUE`].
pub const fn to_opaque(pat: Pattern) -> Pattern {
	let (body, _) = split(pat);
	unsafe { core::mem::transmute((body, OPAQUE)) }
}

/// Make new pattern with given `body` and [`OPAQUE`] mask.
pub const fn opaque(body: Body) -> Pattern { unsafe { core::mem::transmute((body, OPAQUE)) } }
