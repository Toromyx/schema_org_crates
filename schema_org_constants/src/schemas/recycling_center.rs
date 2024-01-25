/// <https://schema.org/RecyclingCenter>
pub const RECYCLING_CENTER_IRI_HTTP: &str = "http://schema.org/RecyclingCenter";
/// <https://schema.org/RecyclingCenter>
pub const RECYCLING_CENTER_IRI_HTTPS: &str = "https://schema.org/RecyclingCenter";
/// <https://schema.org/RecyclingCenter>
pub const RECYCLING_CENTER_LABEL: &str = "RecyclingCenter";
pub struct RecyclingCenterIri;
impl PartialEq<&str> for RecyclingCenterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECYCLING_CENTER_IRI_HTTP || *other == RECYCLING_CENTER_IRI_HTTPS
	}
}
impl PartialEq<RecyclingCenterIri> for &str {
	fn eq(&self, other: &RecyclingCenterIri) -> bool {
		*self == RECYCLING_CENTER_IRI_HTTP || *self == RECYCLING_CENTER_IRI_HTTPS
	}
}
pub struct RecyclingCenterIriOrLabel;
impl PartialEq<&str> for RecyclingCenterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecyclingCenterIri || *other == RECYCLING_CENTER_LABEL
	}
}
impl PartialEq<RecyclingCenterIriOrLabel> for &str {
	fn eq(&self, other: &RecyclingCenterIriOrLabel) -> bool {
		*self == RecyclingCenterIri || *self == RECYCLING_CENTER_LABEL
	}
}
