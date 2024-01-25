/// <https://schema.org/ComputerLanguage>
pub const COMPUTER_LANGUAGE_IRI_HTTP: &str = "http://schema.org/ComputerLanguage";
/// <https://schema.org/ComputerLanguage>
pub const COMPUTER_LANGUAGE_IRI_HTTPS: &str = "https://schema.org/ComputerLanguage";
/// <https://schema.org/ComputerLanguage>
pub const COMPUTER_LANGUAGE_LABEL: &str = "ComputerLanguage";
pub struct ComputerLanguageIri;
impl PartialEq<&str> for ComputerLanguageIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPUTER_LANGUAGE_IRI_HTTP || *other == COMPUTER_LANGUAGE_IRI_HTTPS
	}
}
impl PartialEq<ComputerLanguageIri> for &str {
	fn eq(&self, other: &ComputerLanguageIri) -> bool {
		*self == COMPUTER_LANGUAGE_IRI_HTTP || *self == COMPUTER_LANGUAGE_IRI_HTTPS
	}
}
pub struct ComputerLanguageIriOrLabel;
impl PartialEq<&str> for ComputerLanguageIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ComputerLanguageIri || *other == COMPUTER_LANGUAGE_LABEL
	}
}
impl PartialEq<ComputerLanguageIriOrLabel> for &str {
	fn eq(&self, other: &ComputerLanguageIriOrLabel) -> bool {
		*self == ComputerLanguageIri || *self == COMPUTER_LANGUAGE_LABEL
	}
}
