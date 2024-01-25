/// <https://schema.org/AlcoholConsideration>
pub const ALCOHOL_CONSIDERATION_IRI_HTTP: &str = "http://schema.org/AlcoholConsideration";
/// <https://schema.org/AlcoholConsideration>
pub const ALCOHOL_CONSIDERATION_IRI_HTTPS: &str = "https://schema.org/AlcoholConsideration";
/// <https://schema.org/AlcoholConsideration>
pub const ALCOHOL_CONSIDERATION_LABEL: &str = "AlcoholConsideration";
pub struct AlcoholConsiderationIri;
impl PartialEq<&str> for AlcoholConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALCOHOL_CONSIDERATION_IRI_HTTP || *other == ALCOHOL_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<AlcoholConsiderationIri> for &str {
	fn eq(&self, other: &AlcoholConsiderationIri) -> bool {
		*self == ALCOHOL_CONSIDERATION_IRI_HTTP || *self == ALCOHOL_CONSIDERATION_IRI_HTTPS
	}
}
pub struct AlcoholConsiderationIriOrLabel;
impl PartialEq<&str> for AlcoholConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlcoholConsiderationIri || *other == ALCOHOL_CONSIDERATION_LABEL
	}
}
impl PartialEq<AlcoholConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &AlcoholConsiderationIriOrLabel) -> bool {
		*self == AlcoholConsiderationIri || *self == ALCOHOL_CONSIDERATION_LABEL
	}
}
