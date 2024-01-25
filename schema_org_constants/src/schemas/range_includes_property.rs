/// <https://schema.org/rangeIncludes>
pub const RANGE_INCLUDES_PROPERTY_IRI_HTTP: &str = "http://schema.org/rangeIncludes";
/// <https://schema.org/rangeIncludes>
pub const RANGE_INCLUDES_PROPERTY_IRI_HTTPS: &str = "https://schema.org/rangeIncludes";
/// <https://schema.org/rangeIncludes>
pub const RANGE_INCLUDES_PROPERTY_LABEL: &str = "rangeIncludes";
pub struct RangeIncludesPropertyIri;
impl PartialEq<&str> for RangeIncludesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RANGE_INCLUDES_PROPERTY_IRI_HTTP || *other == RANGE_INCLUDES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RangeIncludesPropertyIri> for &str {
	fn eq(&self, other: &RangeIncludesPropertyIri) -> bool {
		*self == RANGE_INCLUDES_PROPERTY_IRI_HTTP || *self == RANGE_INCLUDES_PROPERTY_IRI_HTTPS
	}
}
pub struct RangeIncludesPropertyIriOrLabel;
impl PartialEq<&str> for RangeIncludesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RangeIncludesPropertyIri || *other == RANGE_INCLUDES_PROPERTY_LABEL
	}
}
impl PartialEq<RangeIncludesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RangeIncludesPropertyIriOrLabel) -> bool {
		*self == RangeIncludesPropertyIri || *self == RANGE_INCLUDES_PROPERTY_LABEL
	}
}
