/// <https://schema.org/NarcoticConsideration>
pub const NARCOTIC_CONSIDERATION_IRI_HTTP: &str = "http://schema.org/NarcoticConsideration";
/// <https://schema.org/NarcoticConsideration>
pub const NARCOTIC_CONSIDERATION_IRI_HTTPS: &str = "https://schema.org/NarcoticConsideration";
/// <https://schema.org/NarcoticConsideration>
pub const NARCOTIC_CONSIDERATION_LABEL: &str = "NarcoticConsideration";
pub struct NarcoticConsiderationIri;
impl PartialEq<&str> for NarcoticConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NARCOTIC_CONSIDERATION_IRI_HTTP || *other == NARCOTIC_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<NarcoticConsiderationIri> for &str {
	fn eq(&self, other: &NarcoticConsiderationIri) -> bool {
		*self == NARCOTIC_CONSIDERATION_IRI_HTTP || *self == NARCOTIC_CONSIDERATION_IRI_HTTPS
	}
}
pub struct NarcoticConsiderationIriOrLabel;
impl PartialEq<&str> for NarcoticConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NarcoticConsiderationIri || *other == NARCOTIC_CONSIDERATION_LABEL
	}
}
impl PartialEq<NarcoticConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &NarcoticConsiderationIriOrLabel) -> bool {
		*self == NarcoticConsiderationIri || *self == NARCOTIC_CONSIDERATION_LABEL
	}
}
