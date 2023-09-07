use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CompileTarget;
use playdate::consts::DEVICE_TARGET;


pub trait CompileKindExt {
	fn is_playdate(&self) -> bool;
	fn is_simulator(&self) -> bool;
	fn playdate() -> Self;
}

impl CompileKindExt for CompileKind {
	fn playdate() -> Self { Self::Target(CompileTarget::playdate()) }

	fn is_playdate(&self) -> bool {
		match self {
			CompileKind::Host => false,
			CompileKind::Target(kind) => kind.is_playdate(),
		}
	}

	fn is_simulator(&self) -> bool {
		match self {
			CompileKind::Host => true,
			CompileKind::Target(kind) => kind.is_simulator(),
		}
	}
}

impl CompileKindExt for CompileTarget {
	fn playdate() -> Self { Self::new(DEVICE_TARGET).unwrap() }
	fn is_playdate(&self) -> bool { self.rustc_target() == DEVICE_TARGET }
	fn is_simulator(&self) -> bool { !self.is_playdate() }
}
