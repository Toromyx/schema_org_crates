/// <https://schema.org/Cardiovascular>
pub const CARDIOVASCULAR_IRI_HTTP: &str = "http://schema.org/Cardiovascular";
/// <https://schema.org/Cardiovascular>
pub const CARDIOVASCULAR_IRI_HTTPS: &str = "https://schema.org/Cardiovascular";
/// <https://schema.org/Cardiovascular>
pub const CARDIOVASCULAR_LABEL: &str = "Cardiovascular";
pub struct CardiovascularIri;
impl PartialEq<&str> for CardiovascularIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CARDIOVASCULAR_IRI_HTTP || *other == CARDIOVASCULAR_IRI_HTTPS
	}
}
impl PartialEq<CardiovascularIri> for &str {
	fn eq(&self, other: &CardiovascularIri) -> bool {
		*self == CARDIOVASCULAR_IRI_HTTP || *self == CARDIOVASCULAR_IRI_HTTPS
	}
}
pub struct CardiovascularIriOrLabel;
impl PartialEq<&str> for CardiovascularIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CardiovascularIri || *other == CARDIOVASCULAR_LABEL
	}
}
impl PartialEq<CardiovascularIriOrLabel> for &str {
	fn eq(&self, other: &CardiovascularIriOrLabel) -> bool {
		*self == CardiovascularIri || *self == CARDIOVASCULAR_LABEL
	}
}
