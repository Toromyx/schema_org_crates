/// <https://schema.org/relatedCondition>
pub const RELATED_CONDITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/relatedCondition";
/// <https://schema.org/relatedCondition>
pub const RELATED_CONDITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/relatedCondition";
/// <https://schema.org/relatedCondition>
pub const RELATED_CONDITION_PROPERTY_LABEL: &str = "relatedCondition";
pub struct RelatedConditionPropertyIri;
impl PartialEq<&str> for RelatedConditionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELATED_CONDITION_PROPERTY_IRI_HTTP
			|| *other == RELATED_CONDITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RelatedConditionPropertyIri> for &str {
	fn eq(&self, other: &RelatedConditionPropertyIri) -> bool {
		*self == RELATED_CONDITION_PROPERTY_IRI_HTTP
			|| *self == RELATED_CONDITION_PROPERTY_IRI_HTTPS
	}
}
pub struct RelatedConditionPropertyIriOrLabel;
impl PartialEq<&str> for RelatedConditionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RelatedConditionPropertyIri || *other == RELATED_CONDITION_PROPERTY_LABEL
	}
}
impl PartialEq<RelatedConditionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &RelatedConditionPropertyIriOrLabel) -> bool {
		*self == RelatedConditionPropertyIri || *self == RELATED_CONDITION_PROPERTY_LABEL
	}
}
