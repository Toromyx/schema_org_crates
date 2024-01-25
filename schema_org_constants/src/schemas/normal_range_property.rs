/// <https://schema.org/normalRange>
pub const NORMAL_RANGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/normalRange";
/// <https://schema.org/normalRange>
pub const NORMAL_RANGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/normalRange";
/// <https://schema.org/normalRange>
pub const NORMAL_RANGE_PROPERTY_LABEL: &str = "normalRange";
pub struct NormalRangePropertyIri;
impl PartialEq<&str> for NormalRangePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NORMAL_RANGE_PROPERTY_IRI_HTTP || *other == NORMAL_RANGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NormalRangePropertyIri> for &str {
	fn eq(&self, other: &NormalRangePropertyIri) -> bool {
		*self == NORMAL_RANGE_PROPERTY_IRI_HTTP || *self == NORMAL_RANGE_PROPERTY_IRI_HTTPS
	}
}
pub struct NormalRangePropertyIriOrLabel;
impl PartialEq<&str> for NormalRangePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NormalRangePropertyIri || *other == NORMAL_RANGE_PROPERTY_LABEL
	}
}
impl PartialEq<NormalRangePropertyIriOrLabel> for &str {
	fn eq(&self, other: &NormalRangePropertyIriOrLabel) -> bool {
		*self == NormalRangePropertyIri || *self == NORMAL_RANGE_PROPERTY_LABEL
	}
}
