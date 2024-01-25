/// <https://schema.org/typicalAgeRange>
pub const TYPICAL_AGE_RANGE_PROPERTY_IRI_HTTP: &str = "http://schema.org/typicalAgeRange";
/// <https://schema.org/typicalAgeRange>
pub const TYPICAL_AGE_RANGE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/typicalAgeRange";
/// <https://schema.org/typicalAgeRange>
pub const TYPICAL_AGE_RANGE_PROPERTY_LABEL: &str = "typicalAgeRange";
pub struct TypicalAgeRangePropertyIri;
impl PartialEq<&str> for TypicalAgeRangePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TYPICAL_AGE_RANGE_PROPERTY_IRI_HTTP
			|| *other == TYPICAL_AGE_RANGE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TypicalAgeRangePropertyIri> for &str {
	fn eq(&self, other: &TypicalAgeRangePropertyIri) -> bool {
		*self == TYPICAL_AGE_RANGE_PROPERTY_IRI_HTTP
			|| *self == TYPICAL_AGE_RANGE_PROPERTY_IRI_HTTPS
	}
}
pub struct TypicalAgeRangePropertyIriOrLabel;
impl PartialEq<&str> for TypicalAgeRangePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TypicalAgeRangePropertyIri || *other == TYPICAL_AGE_RANGE_PROPERTY_LABEL
	}
}
impl PartialEq<TypicalAgeRangePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TypicalAgeRangePropertyIriOrLabel) -> bool {
		*self == TypicalAgeRangePropertyIri || *self == TYPICAL_AGE_RANGE_PROPERTY_LABEL
	}
}
