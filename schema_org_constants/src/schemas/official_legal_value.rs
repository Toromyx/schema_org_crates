/// <https://schema.org/OfficialLegalValue>
pub const OFFICIAL_LEGAL_VALUE_IRI_HTTP: &str = "http://schema.org/OfficialLegalValue";
/// <https://schema.org/OfficialLegalValue>
pub const OFFICIAL_LEGAL_VALUE_IRI_HTTPS: &str = "https://schema.org/OfficialLegalValue";
/// <https://schema.org/OfficialLegalValue>
pub const OFFICIAL_LEGAL_VALUE_LABEL: &str = "OfficialLegalValue";
pub struct OfficialLegalValueIri;
impl PartialEq<&str> for OfficialLegalValueIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFICIAL_LEGAL_VALUE_IRI_HTTP || *other == OFFICIAL_LEGAL_VALUE_IRI_HTTPS
	}
}
impl PartialEq<OfficialLegalValueIri> for &str {
	fn eq(&self, other: &OfficialLegalValueIri) -> bool {
		*self == OFFICIAL_LEGAL_VALUE_IRI_HTTP || *self == OFFICIAL_LEGAL_VALUE_IRI_HTTPS
	}
}
pub struct OfficialLegalValueIriOrLabel;
impl PartialEq<&str> for OfficialLegalValueIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OfficialLegalValueIri || *other == OFFICIAL_LEGAL_VALUE_LABEL
	}
}
impl PartialEq<OfficialLegalValueIriOrLabel> for &str {
	fn eq(&self, other: &OfficialLegalValueIriOrLabel) -> bool {
		*self == OfficialLegalValueIri || *self == OFFICIAL_LEGAL_VALUE_LABEL
	}
}
