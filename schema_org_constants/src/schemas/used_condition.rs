/// <https://schema.org/UsedCondition>
pub const USED_CONDITION_IRI_HTTP: &str = "http://schema.org/UsedCondition";
/// <https://schema.org/UsedCondition>
pub const USED_CONDITION_IRI_HTTPS: &str = "https://schema.org/UsedCondition";
/// <https://schema.org/UsedCondition>
pub const USED_CONDITION_LABEL: &str = "UsedCondition";
pub struct UsedConditionIri;
impl PartialEq<&str> for UsedConditionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USED_CONDITION_IRI_HTTP || *other == USED_CONDITION_IRI_HTTPS
	}
}
impl PartialEq<UsedConditionIri> for &str {
	fn eq(&self, other: &UsedConditionIri) -> bool {
		*self == USED_CONDITION_IRI_HTTP || *self == USED_CONDITION_IRI_HTTPS
	}
}
pub struct UsedConditionIriOrLabel;
impl PartialEq<&str> for UsedConditionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UsedConditionIri || *other == USED_CONDITION_LABEL
	}
}
impl PartialEq<UsedConditionIriOrLabel> for &str {
	fn eq(&self, other: &UsedConditionIriOrLabel) -> bool {
		*self == UsedConditionIri || *self == USED_CONDITION_LABEL
	}
}
