/// <https://schema.org/UnofficialLegalValue>
pub const UNOFFICIAL_LEGAL_VALUE_IRI_HTTP: &str = "http://schema.org/UnofficialLegalValue";
/// <https://schema.org/UnofficialLegalValue>
pub const UNOFFICIAL_LEGAL_VALUE_IRI_HTTPS: &str = "https://schema.org/UnofficialLegalValue";
/// <https://schema.org/UnofficialLegalValue>
pub const UNOFFICIAL_LEGAL_VALUE_LABEL: &str = "UnofficialLegalValue";
pub struct UnofficialLegalValueIri;
impl PartialEq<&str> for UnofficialLegalValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNOFFICIAL_LEGAL_VALUE_IRI_HTTP || *other == UNOFFICIAL_LEGAL_VALUE_IRI_HTTPS
	}
}
impl PartialEq<UnofficialLegalValueIri> for &str {
	fn eq(&self, other: &UnofficialLegalValueIri) -> bool {
		*self == UNOFFICIAL_LEGAL_VALUE_IRI_HTTP || *self == UNOFFICIAL_LEGAL_VALUE_IRI_HTTPS
	}
}
pub struct UnofficialLegalValueIriOrLabel;
impl PartialEq<&str> for UnofficialLegalValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnofficialLegalValueIri || *other == UNOFFICIAL_LEGAL_VALUE_LABEL
	}
}
impl PartialEq<UnofficialLegalValueIriOrLabel> for &str {
	fn eq(&self, other: &UnofficialLegalValueIriOrLabel) -> bool {
		*self == UnofficialLegalValueIri || *self == UNOFFICIAL_LEGAL_VALUE_LABEL
	}
}
