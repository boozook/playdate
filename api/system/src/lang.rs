pub use sys::ffi::PDLanguage;

pub trait PDLanguageExt {
	#![allow(non_upper_case_globals)]
	const English: PDLanguage = PDLanguage::kPDLanguageEnglish;
	const Japanese: PDLanguage = PDLanguage::kPDLanguageJapanese;
	const Unknown: PDLanguage = PDLanguage::kPDLanguageUnknown;
}


impl PDLanguageExt for PDLanguage {}
