#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;
use std::fmt::Binary;
use std::fmt::Display;
use std::fmt::Debug;

use crate::fmt::addr::Addr;


/// SCB+
#[derive(Debug, Clone)]
pub struct ExceptionFrame {
	// General-purpose registers:
	// Low regs:
	r0: u32,
	r1: u32,
	r2: u32,
	r3: u32,
	// High regs:
	r12: u32,
	/// Link Register (r14)
	lr: Addr<u32>,
	/// Program Counter (r15)
	/// Current program address.
	pc: Addr<u32>,

	// Special registers:
	/// Program status register.
	///
	/// The Program Status Register combines:
	/// - Application Program Status Register ([`APSR`]).
	/// - Interrupt Program Status Register ([`IPSR`]).
	/// - Execution Program Status Register ([`EPSR`]).
	psr: PSR,

	/// Configurable Fault Status
	///
	/// combines [`ufsr`][Self::ufsr] + [`bfsr`][Self::bfsr] + [`mmfsr`][Self::mmfsr].
	cfsr: CFSR,

	/// HardFault Status
	hfsr: HSFR,

	/// MemManage Fault Address
	///
	/// The BFAR address is associated with a precise data access BusFault.
	/// _This field is valid only when [`MMFSR::MMARVALID`] is set._
	mmfar: Addr<u32>,

	/// BusFault Address.
	/// Data address for a precise BusFault.
	///
	/// Contains the address of a location that produced a BusFault.
	/// The [`BFSR`] shows the reason for the fault.
	/// _This field is valid only when [`BFSR::BFARVALID`] is set._
	bfar: Addr<u32>,

	/// `bootinfo.rcccsr` value
	rcccsr: u32,
}


pub const SCB_CPACR_FPU_MASK: u32 = 0b11_11 << 20;
pub const SCB_CPACR_FPU_ENABLE: u32 = 0b01_01 << 20;
pub const SCB_CPACR_FPU_USER: u32 = 0b10_10 << 20;

impl ExceptionFrame {
	pub fn ufsr(&self) -> UFSR { self.cfsr.ufsr() }
	pub fn bfsr(&self) -> BFSR { self.cfsr.bfsr() }
	pub fn mmfsr(&self) -> MMFSR { self.cfsr.mmfsr() }
}


impl ExceptionFrame {
	pub fn new_from(values: &HashMap<String, Addr<u32>>) -> Result<Self, &'static str> {
		Ok(Self { r0: values.get("r0").ok_or("no r0")?.value(),
		          r1: values.get("r1").ok_or("no r1")?.value(),
		          r2: values.get("r2").ok_or("no r2")?.value(),
		          r3: values.get("r3").ok_or("no r3")?.value(),
		          r12: values.get("r12").ok_or("no r12")?.value(),
		          lr: values.get("lr").ok_or("no lr")?.to_owned(),
		          pc: values.get("pc").ok_or("no pc")?.to_owned(),
		          psr: values.get("psr").ok_or("no psr")?.value().into(),
		          cfsr: values.get("cfsr").ok_or("no cfsr")?.value().into(),
		          hfsr: values.get("hfsr").ok_or("no hfsr")?.value().into(),
		          mmfar: values.get("mmfar").ok_or("no mmfar")?.to_owned(),
		          bfar: values.get("bfar").ok_or("no bfar")?.to_owned(),
		          rcccsr: values.get("rcccsr").ok_or("no rcccsr")?.value().into() })
	}
}


macro_rules! bit {
	($mask:literal, $name:ident.$field:ident, $doc:literal) => {
		impl $name {
			bit! {$mask, $field, $doc}
		}
	};
	($mask:literal, $name:ident) => {
		pub fn $name(&self) -> bool { self.0 & $mask != 0 }

		paste::paste! {
			pub const [<DOC_ $name>]: &'static str = stringify!($name);
		}
	};
	($mask:literal, $name:ident, $doc:literal) => {
		#[doc = $doc]
		pub fn $name(&self) -> bool { self.0 & $mask != 0 }

		paste::paste! {
			pub const [<DOC_ $name>]: &'static str = $doc;
		}
	};

	($name:ident, $mask:literal) => {
		bit!($mask, $name)
	};
	($name:ident, $mask:literal, $doc:literal) => {
		bit!($mask, $name, $doc)
	};
}


macro_rules! impl_fmt {
		($name:ident) => {
			impl Display for $name {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Display::fmt(&self.0, f) }
			}
			impl Binary for $name {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { Binary::fmt(&self.0, f) }
			}
		};

		($name:ident, $($next:ident),+) => {
			impl_fmt!($name);
         impl_fmt!($($next),+);
		}
	}


macro_rules! impl_from {
	($name:ident<u8>) => {
		impl_from! {impl $name<u8>}
		impl_from! {impl $name<i8>}
	};
	($name:ident<u16>) => {
		impl_from! {impl $name<u16>}
		impl_from! {impl $name<i16>}
		impl_from! {$name<u8>}
	};
	($name:ident<u32>) => {
		impl_from! {impl $name<u32>}
		impl_from! {impl $name<i32>}
		impl_from! {$name<u16>}
	};
	($name:ident<u64>) => {
		impl_from! {impl $name<u64>}
		impl_from! {impl $name<i64>}
		impl_from! {$name<u32>}
	};
	($name:ident) => {
		impl_from! {$name<u32>}
	};

	(impl $name:ident<$t:ty>) => {
		impl From<$t> for $name {
			fn from(value: $t) -> Self { Self(value as _) }
		}
	};
}

macro_rules! impl_try_from {
	($name:ident<u8>) => {
		impl_try_from! {impl $name<u8, u16>}
		impl_try_from! {impl $name<u8, u32>}
		impl_try_from! {impl $name<u8, u64>}
	};
	($name:ident<u16>) => {
		impl_try_from! {impl $name<u8, u32>}
		impl_try_from! {impl $name<u8, u64>}
	};
	($name:ident<u32>) => {
		impl_try_from! {impl $name<u8, u64>}
	};
	($name:ident<u64>) => {};
	($name:ident) => {
		impl_try_from! {$name<u32>}
	};

	(impl $name:ident<$t:ty, $th:ty>) => {
		impl TryFrom<$th> for $name
			where $t: TryFrom<$th>,
			      Self: From<$t>
		{
			type Error = <$t as TryFrom<$th>>::Error;
			fn try_from(value: $th) -> Result<Self, Self::Error> {
				let value: $t = value.try_into()?;
				Ok(Self::from(value))
			}
		}
	};
}

macro_rules! impl_to {
	($name:ident<u8>) => {
		impl_to! {impl $name<u8>}
		impl_to! {impl $name<i8>}
		impl_to! {$name<u16>}
	};
	($name:ident<u16>) => {
		impl_to! {impl $name<u16>}
		impl_to! {impl $name<i16>}
		impl_to! {$name<u32>}
	};
	($name:ident<u32>) => {
		impl_to! {impl $name<u32>}
		impl_to! {impl $name<i32>}
		impl_to! {$name<u64>}
	};
	($name:ident<u64>) => {
		impl_to! {impl $name<u64>}
		impl_to! {impl $name<i64>}
	};
	($name:ident) => {
		impl_to! {$name<u32>}
	};

	(impl $name:ident<$t:ty>) => {
		impl From<$name> for $t {
			fn from(value: $name) -> Self { value.0 as _ }
		}
	};
}

macro_rules! impl_convert {
	($name:ident<u8>) => {
		impl_to! {$name<u8>}
		impl_from! {$name<u8>}
		impl_try_from! {$name<u8>}
	};
	($name:ident<u16>) => {
		impl_to! {$name<u16>}
		impl_from! {$name<u16>}
		impl_try_from! {$name<u16>}
	};
	($name:ident<u32>) => {
		impl_to! {$name<u32>}
		impl_from! {$name<u32>}
		impl_try_from! {$name<u32>}
	};
	($name:ident<u64>) => {
		impl_to! {$name<u64>}
		impl_from! {$name<u64>}
		impl_try_from! {$name<u64>}
	};
	($name:ident) => {
		impl_convert! {$name<u32>}
	};
}

macro_rules! impl_reg {
		($name:ident) => {
			impl_reg!{$name<u32>}
		};
		($name:ident<$t:ty>) => {
			impl_fmt!{$name}
		};

		($name:ident$(<$t:ty>)?, $($next:ident$(<$tn:ty>)?),+) => {
			impl_reg!($name$(<$t>)?);
         impl_reg!($($next$(<$tn>)?),+);
		}
	}

impl_reg! {PSR, IPSR<u16>, APSR<u16>, EPSR, CFSR, UFSR<u16>, BFSR<u8>, MMFSR<u8>, HSFR}
impl_convert! { PSR }
impl_convert! { IPSR<u16> }
impl_convert! { APSR<u16> }
impl_convert! { EPSR }
impl_convert! { CFSR }
impl_convert! { UFSR<u16> }
impl_convert! { BFSR<u8> }
impl_convert! { MMFSR<u8> }
impl_convert! { HSFR }

// _PSR entry in the crash log also includes the EPSR as well._
// ESPR - Execution Program Status Register
//
/// Program status register.
///
/// The Program Status Register (PSR) combines:
/// - Application Program Status Register (APSR).
/// - Interrupt Program Status Register (IPSR).
/// - Execution Program Status Register (EPSR).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PSR(u32);

impl PSR {
	pub fn apsr(&self) -> APSR { APSR(((self.0 & 0b11111000_00000111_00000000_00000000) >> 16) as u16) }
	pub fn ipsr(&self) -> IPSR { IPSR((self.0 & 0b00000000_00000000_00000001_11111111) as u16) }
	pub fn epsr(&self) -> EPSR { EPSR(self.0 & 0b00000111_00000000_11111100_00000000) }
}

impl Debug for PSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("PSR")
		 .field("APSR", &self.apsr())
		 .field("IPSR", &self.ipsr())
		 .field("EPSR", &self.epsr())
		 .finish()
	}
}


/// Application Program Status Register.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct APSR(u16);
bit! {0b1000_0000_00000000, APSR.N, r#"Negative"#}
bit! {0b0100_0000_00000000, APSR.Z, r#"Zero"#}
bit! {0b0010_0000_00000000, APSR.C, r#"Carry or borrow"#}
bit! {0b0001_0000_00000000, APSR.V, r#"Overflow"#}
bit! {0b0000_1000_00000000, APSR.Q, r#"DSP overflow and saturation"#}
bit! {0b0000_0000_00000111, APSR.GE, r#"Greater than or Equals"#}
impl Debug for APSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("APSR")
		 .field("N", &self.N())
		 .field("Z", &self.Z())
		 .field("C", &self.C())
		 .field("V", &self.V())
		 .field("Q", &self.Q())
		 .field("GE", &self.GE())
		 .finish()
	}
}
impl RegTags for APSR {
	fn is_empty(&self) -> bool { self.0 == 0 }

	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)> {
		if !self.is_empty() {
			[
			 self.N().then_some(("N", Self::DOC_N)),
			 self.Z().then_some(("Z", Self::DOC_Z)),
			 self.C().then_some(("C", Self::DOC_C)),
			 self.V().then_some(("V", Self::DOC_V)),
			 self.Q().then_some(("Q", Self::DOC_Q)),
			 self.GE().then_some(("GE", Self::DOC_GE)),
			].into_iter()
			.flatten()
		} else {
			<[_; 6]>::default().into_iter().flatten()
		}
	}
}

/// Interrupt Program Status Register.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct IPSR(u16);
bit! {0b0000_0000_00000001, IPSR.TM, r#"Thread mode"#}
bit! {0b0000_0000_00000100, IPSR.NMI, r#"NMI"#}
bit! {0b0000_0000_00001000, IPSR.HF, r#"HardFault"#}
bit! {0b0000_0000_00010000, IPSR.MM, r#"MemManage"#}
bit! {0b0000_0000_00100000, IPSR.BF, r#"BusFault"#}
bit! {0b0000_0000_01000000, IPSR.UF, r#"UsageFault"#}
impl Debug for IPSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("IPSR")
		 .field("TM", &self.TM())
		 .field("NMI", &self.NMI())
		 .field("HF", &self.HF())
		 .field("MM", &self.MM())
		 .field("BF", &self.BF())
		 .field("UF", &self.UF())
		 .finish()
	}
}
impl RegTags for IPSR {
	fn is_empty(&self) -> bool { self.0 == 0 }

	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)> {
		if !self.is_empty() {
			[
			 self.TM().then_some(("TM", Self::DOC_TM)),
			 self.NMI().then_some(("NMI", Self::DOC_NMI)),
			 self.HF().then_some(("HF", Self::DOC_HF)),
			 self.MM().then_some(("MM", Self::DOC_MM)),
			 self.BF().then_some(("BF", Self::DOC_BF)),
			 self.UF().then_some(("UF", Self::DOC_UF)),
			].into_iter()
			.flatten()
		} else {
			<[_; 6]>::default().into_iter().flatten()
		}
	}
}

/// Execution Program Status Register.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EPSR(u32);
bit! {0b00000110_00000000_00001100_00000000, EPSR.IS_NOT_ICI, r#"Interruptible-continuable instruction bits."#}
bit! {0b00000000_00000000_11110000_00000000, EPSR.ICI, r#"Interruptible-continuable instruction bits."#}
bit! {0b00000110_00000000_11111100_00000000, EPSR.IT, r#"If-Then block. Indicates the execution state bits of the IT instruction"#}
bit! {0b00000001_00000000_00000000_00000000, EPSR.T, r#"Thumb state"#}
impl Debug for EPSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("EPSR")
		 .field("IS_NOT_ICI", &self.IS_NOT_ICI())
		 .field("ICI", &self.ICI())
		 .field("IT", &self.IT())
		 .field("T", &self.T())
		 .finish()
	}
}
impl RegTags for EPSR {
	fn is_empty(&self) -> bool { self.0 == 0 }

	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)> {
		if !self.is_empty() {
			[
			 self.IS_NOT_ICI().then_some(("IS_NOT_ICI", Self::DOC_IS_NOT_ICI)),
			 self.ICI().then_some(("ICI", Self::DOC_ICI)),
			 self.IT().then_some(("IT", Self::DOC_IT)),
			 self.T().then_some(("T", Self::DOC_T)),
			].into_iter()
			.flatten()
		} else {
			<[_; 4]>::default().into_iter().flatten()
		}
	}
}


/// HardFault Status Register.
///
/// The Hard Fault status register indicates an incorrect usage of a CPU instruction.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct HSFR(u32);
impl HSFR {
	// When this bit is set, the PC value stacked for the exception return points to the instruction
	// that was preempted by the exception. This error is always a Hard Fault.
	bit! {0b00000000_00000000_00000000_00000010, VECTTBL, r#"Bus Fault on vector table read."#}
	bit! {0b01000000_00000000_00000000_00000000, FORCED, r#"Forced Hard Fault."#}
	bit! {0b10000000_00000000_00000000_00000000, DEBUGEVT, r#"Reserved for Debug use."#}
}
impl Debug for HSFR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("HSFR")
		 .field("VECTTBL", &self.VECTTBL())
		 .field("FORCED", &self.FORCED())
		 .field("DEBUGEVT", &self.DEBUGEVT())
		 .finish()
	}
}
impl RegTags for HSFR {
	fn is_empty(&self) -> bool { self.0 == 0 }

	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)> {
		if !self.is_empty() {
			[
			 self.VECTTBL().then_some(("VECTTBL", Self::DOC_VECTTBL)),
			 self.FORCED().then_some(("FORCED", Self::DOC_FORCED)),
			 self.DEBUGEVT().then_some(("DEBUGEVT", Self::DOC_DEBUGEVT)),
			].into_iter()
			.flatten()
		} else {
			<[_; 3]>::default().into_iter().flatten()
		}
	}
}


/// Configurable Fault Status
///
/// combines [`ufsr`][Self::ufsr] + [`bfsr`][Self::bfsr] + [`mmfsr`][Self::mmfsr].
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CFSR(u32);

impl CFSR {
	pub fn ufsr(&self) -> UFSR { UFSR(((self.0 & 0b11111111_11111111_00000000_00000000) >> 16) as u16) }
	pub fn bfsr(&self) -> BFSR { BFSR(((self.0 & 0b00000000_00000000_11111111_00000000) >> 8) as u8) }
	pub fn mmfsr(&self) -> MMFSR { MMFSR(self.0 as u8) }
}

impl Debug for CFSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CFSR")
		 .field("UFSR", &self.ufsr())
		 .field("BFSR", &self.bfsr())
		 .field("MMFSR", &self.mmfsr())
		 .finish()
	}
}


/// The UsageFault Status Register contains the status for some instruction execution faults, and for data access.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UFSR(u16);
impl UFSR {
	bit! {0b0000_0000_0000_0001, UNDEFINSTR, r#"Undefined instruction."#}
	bit! {0b0000_0000_0000_0010, INVSTATE, r#"Invalid state."#}
	bit! {0b0000_0000_0000_0100, INVPC, r#"Invalid `PC` load UsageFault, caused by an invalid `EXC_RETURN` value."#}
	bit! {0b0000_0000_0000_1000, NOCP, r#"No coprocessor."#}
	bit! {0b0000_0001_0000_0000, UNALIGNED, r#"Unaligned access UsageFault."#}
	bit! {0b0010_0000, DIVBYZERO, r#"Divide by zero UsageFault."#}
}


impl RegTags for UFSR {
	fn is_empty(&self) -> bool { self.0 == 0 }

	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)> {
		if self.is_empty() {
			return [None, None, None, None, None, None].into_iter().flatten();
		}

		[
		 self.UNDEFINSTR().then_some(("UNDEFINSTR", Self::DOC_UNDEFINSTR)),
		 self.INVSTATE().then_some(("INVSTATE", Self::DOC_INVSTATE)),
		 self.INVPC().then_some(("INVPC", Self::DOC_INVPC)),
		 self.NOCP().then_some(("NOCP", Self::DOC_NOCP)),
		 self.UNALIGNED().then_some(("UNALIGNED", Self::DOC_UNALIGNED)),
		 self.DIVBYZERO().then_some(("DIVBYZERO", Self::DOC_DIVBYZERO)),
		].into_iter()
		.flatten()
	}
}

impl Debug for UFSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("UFSR")
		 .field("UNDEFINSTR", &self.UNDEFINSTR())
		 .field("INVSTATE", &self.INVSTATE())
		 .field("INVPC", &self.INVPC())
		 .field("NOCP", &self.NOCP())
		 .field("UNALIGNED", &self.UNALIGNED())
		 .field("DIVBYZERO", &self.DIVBYZERO())
		 .finish()
	}
}


/// The BusFault Status Register shows the status of bus errors resulting from instruction fetches and data accesses
/// and indicates memory access faults detected during a bus operation.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BFSR(u8);
impl BFSR {
	bit! {0b0000_0001, IBUSERR, r#"Instruction bus error. Records whether a BusFault on an instruction prefetch has occurred."#}
	bit! {0b0000_0010, PRECISERR, r#"Precise data bus error."#}
	bit! {0b0000_0100, IMPRECISERR, r#"Imprecise data bus error."#}
	bit! {0b0000_1000, UNSTKERR, r#"BusFault on unstacking for a return from exception."#}
	bit! {0b0001_0000, STKERR, r#"BusFault on stacking for exception entry."#}
	bit! {0b0010_0000, LSPERR, r#"BusFault during floating point lazy state preservation (only when FPU present)."#}
	bit! {0b1000_0000, BFARVALID, r#"BusFault Address Register valid flag. BFAR holds a valid fault address."#}
}
impl Debug for BFSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("BFSR")
		 .field("IBUSERR", &self.IBUSERR())
		 .field("PRECISERR", &self.PRECISERR())
		 .field("IMPRECISERR", &self.IMPRECISERR())
		 .field("UNSTKERR", &self.UNSTKERR())
		 .field("STKERR", &self.STKERR())
		 .field("LSPERR", &self.LSPERR())
		 .field("BFARVALID", &self.BFARVALID())
		 .finish()
	}
}
impl RegTags for BFSR {
	fn is_empty(&self) -> bool { self.0 == 0 }

	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)> {
		if !self.is_empty() {
			[
			 self.IBUSERR().then_some(("IBUSERR", Self::DOC_IBUSERR)),
			 self.PRECISERR().then_some(("PRECISERR", Self::DOC_PRECISERR)),
			 self.IMPRECISERR()
			     .then_some(("IMPRECISERR", Self::DOC_IMPRECISERR)),
			 self.UNSTKERR().then_some(("UNSTKERR", Self::DOC_UNSTKERR)),
			 self.STKERR().then_some(("STKERR", Self::DOC_STKERR)),
			 self.LSPERR().then_some(("LSPERR", Self::DOC_LSPERR)),
			].into_iter()
			.flatten()
		} else {
			<[_; 6]>::default().into_iter().flatten()
		}
	}
}


/// The MemManage fault status register
/// indicates a memory access violation detected by the Memory Protection Unit (MPU).
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MMFSR(u8);
impl Debug for MMFSR {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MMFSR")
		 .field("IACCVIOL", &self.IACCVIOL())
		 .field("DACCVIOL", &self.DACCVIOL())
		 .field("MUNSTKERR", &self.MUNSTKERR())
		 .field("MSTKERR", &self.MSTKERR())
		 .field("MLSPERR", &self.MLSPERR())
		 .field("MMARVALID", &self.MMARVALID())
		 .finish()
	}
}
impl MMFSR {
	// Instruction access violation flag.
	// The processor attempted an instruction fetch from a location that does not permit execution
	//
	// The PC value stacked for the exception return points to the faulting instruction. The processor
	// has not written a fault address to the `MMFAR`. This fault condition occurs on any attempt of
	// instruction fetches to an XN (eXecute Never) region, even when the MPU is disabled or not
	// present. Potential reasons:
	// - Branch to regions that are not defined in the MPU or defined as non-executable.
	// - Invalid return due to corrupted stack content.
	// - Incorrect entry in the exception vector table.
	bit! {0b0000_0001, IACCVIOL, r#"Instruction access violation.
		The processor attempted an instruction fetch from a location that does not permit execution.
		Faulting instruction: see `PC`.
		"#}

	// Data access violation flag.
	// The processor attempted a load or store at a location that does not permit the operation.
	//
	// The PC value stacked for the exception return points to the faulting instruction.
	// The processor has loaded the `MMFAR` with the address of the attempted access.
	bit! {0b0000_0010, DACCVIOL, r#"Data access violation.
		The processor attempted a load or store at a location that does not permit the operation.
		Faulting instruction: see `PC`.
		Address of the attempted access: see `MMFAR`.
		"#}

	// MemManage fault on unstacking for a return from exception.
	//
	// Unstacking for an exception return has caused one or more access violations.
	//
	// This fault is chained to the handler which means that the original return stack is still present.
	// The processor has not adjusted the SP from the failing return, and has not performed a new
	// save. The processor has not written a fault address to the `MMFAR`. Potential reasons:
	// - Stack pointer is corrupted
	// - MPU region for the stack changed during execution of the exception handler.
	bit! {0b0000_1000, MUNSTKERR, r#"MemManage fault on unstacking for a return from exception.
		Fault address - see MMFAR.
		Potential reasons:
		- Stack pointer is corrupted
		- MPU region for the stack changed during execution of the exception handler
		"#}


	bit! {0b0001_0000, MSTKERR, r#"
			MemManage fault on stacking for exception entry.

			The SP is still adjusted but the values in the context area on the stack might be incorrect. The
			processor has not written a fault address to the MMFAR. Potential reasons:
			- Stack pointer is corrupted or not initialized
			- Stack is reaching a region not defined by the MPU as read/write memory.
			"#}

	bit! {0b0010_0000, MLSPERR, r#"MemManage fault during floating point lazy state preservation (only Cortex-M4 with FPU)."#}
	bit! {0b1000_0000, MMARVALID, r#"MemManage Fault Address Register (MMFAR) valid flag. SCB->MMFAR holds a valid fault address."#}
}

impl RegTags for MMFSR {
	fn is_empty(&self) -> bool { self.0 == 0 }

	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)> {
		if self.is_empty() {
			return [None, None, None, None, None].into_iter().flatten();
		}

		[
		 self.IACCVIOL().then_some(("IACCVIOL", Self::DOC_IACCVIOL)),
		 self.DACCVIOL().then_some(("DACCVIOL", Self::DOC_DACCVIOL)),
		 self.MUNSTKERR().then_some(("MUNSTKERR", Self::DOC_MUNSTKERR)),
		 self.MSTKERR().then_some(("MSTKERR", Self::DOC_MSTKERR)),
		 self.MLSPERR().then_some(("MLSPERR", Self::DOC_MLSPERR)),
		].into_iter()
		.flatten()
	}
}


pub trait RegTags {
	/// Has no tags
	fn is_empty(&self) -> bool;

	/// Returns `true` tags as (name + description)
	fn tags(&self) -> impl IntoIterator<Item = (&str, &str)>;
}


pub enum FaultKind {
	// HFSR:
	/// Bus error on a vector read error HardFault
	VECTTBL,
	/// Fault that is escalated to a hard fault
	FORCED,
	/// Fault on breakpoint escalation
	DEBUGEVT,

	// MMFSR:
	/// Fault on instruction access MemManage
	IACCVIOL,
	/// Fault on direct data access
	DACCVIOL,
	/// Context stacking, because of an MPU access violation
	MSTKERR,
	/// Context unstacking, because of an MPU access violation
	MUNSTKERR,
	/// During lazy floating-point state preservation
	MLSPERR,

	// BFSR:
	/// During exception stacking BusFault
	STKERR,
	/// During exception unstacking
	UNSTKERR,
	/// During instruction prefetching, precise
	IBUSERR,
	/// During lazy floating-point state preservation
	LSPERR,
	/// Precise data access error, precise
	PRECISERR,
	/// Imprecise data access error, imprecise
	IMPRECISERR,

	// UFSR:
	/// Undefined instruction UsageFault
	UNDEFINSTR,
	/// Attempt to enter an invalid instruction set state
	INVSTATE,
	/// Failed integrity check on exception return
	INVPC,
	/// Attempt to access a non-existing coprocessor
	NOCPC,
	/// Illegal unaligned load or store
	UNALIGNED,
	/// Stack overflow
	STKOF,
	/// Divide By 0
	DIVBYZERO,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpuAccessMode {
	/// FPU is not accessible
	Disabled,
	/// FPU is accessible in Privileged and User mode
	Enabled,
	/// FPU is accessible in Privileged mode only
	Privileged,
}


/// Active exception number
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub enum VectActive {
	/// Thread mode
	ThreadMode,

	/// Processor core exception (internal interrupts)
	Exception(Exception),

	/// Device specific exception (external interrupts)
	Interrupt {
		/// Interrupt number. This number is always within half open range `[0, 512)` (9 bit)
		irqn: u16,
	},
}

impl VectActive {
	/// Converts a vector number into `VectActive`
	#[inline]
	pub fn from(vect_active: u16) -> Option<Self> {
		Some(match vect_active {
			0 => VectActive::ThreadMode,
			2 => VectActive::Exception(Exception::NonMaskableInt),
			3 => VectActive::Exception(Exception::HardFault),
			4 => VectActive::Exception(Exception::MemoryManagement),
			5 => VectActive::Exception(Exception::BusFault),
			6 => VectActive::Exception(Exception::UsageFault),
			7 => VectActive::Exception(Exception::SecureFault),
			11 => VectActive::Exception(Exception::SVCall),
			12 => VectActive::Exception(Exception::DebugMonitor),
			14 => VectActive::Exception(Exception::PendSV),
			15 => VectActive::Exception(Exception::SysTick),
			irqn if (16..512).contains(&irqn) => VectActive::Interrupt { irqn: irqn - 16 },
			_ => return None,
		})
	}
}


/// Processor core exceptions (internal interrupts)
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub enum Exception {
	/// Non maskable interrupt
	NonMaskableInt,

	/// Hard fault interrupt
	HardFault,

	/// Memory management interrupt (not present on Cortex-M0 variants)
	MemoryManagement,

	/// Bus fault interrupt (not present on Cortex-M0 variants)
	BusFault,

	/// Usage fault interrupt (not present on Cortex-M0 variants)
	UsageFault,

	/// Secure fault interrupt (only on ARMv8-M)
	SecureFault,

	/// SV call interrupt
	SVCall,

	/// Debug monitor interrupt (not present on Cortex-M0 variants)
	DebugMonitor,

	/// Pend SV interrupt
	PendSV,

	/// System Tick interrupt
	SysTick,
}


#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn test_mmfsr_daccviol() {
		let reg = ExceptionFrame { r0: 0x00000004,
		                           r1: 0x00000008,
		                           r2: 0x00000008,
		                           r3: 0x00000008,
		                           r12: 0x00000008,
		                           lr: 0x6000d2e7.into(),
		                           pc: 0x6000d5e8.into(),
		                           psr: 0x010f0000.into(),
		                           cfsr: 0x00000082.into(),
		                           hfsr: 0x00000000.into(),
		                           mmfar: 0x00000004.into(),
		                           bfar: 0x00000004.into(),
		                           rcccsr: 0x00000000 };
		let mmfsr = reg.mmfsr();
		assert!(mmfsr.DACCVIOL());
		assert!(mmfsr.MMARVALID());
	}

	#[test]
	fn test_mmfsr_daccviol__() {
		let reg = ExceptionFrame { r0: 0x00000004,
		                           r1: 0x00000008,
		                           r2: 0x00000008,
		                           r3: 0x00000008,
		                           r12: 0x00000008,
		                           lr: 0x6000d2e7.into(),
		                           pc: 0x6000d5e8.into(),
		                           psr: 0x010f0000.into(),
		                           cfsr: 0x00000082.into(),
		                           hfsr: 0x00000000.into(),
		                           mmfar: 0x00000004.into(),
		                           bfar: 0x00000004.into(),
		                           rcccsr: 0x00000000 };
		let mmfsr = reg.mmfsr();
		assert!(mmfsr.DACCVIOL());
		assert!(mmfsr.MMARVALID());
	}
}
