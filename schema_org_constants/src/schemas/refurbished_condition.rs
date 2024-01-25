/// <https://schema.org/RefurbishedCondition>
pub const REFURBISHED_CONDITION_IRI_HTTP: &str = "http://schema.org/RefurbishedCondition";
/// <https://schema.org/RefurbishedCondition>
pub const REFURBISHED_CONDITION_IRI_HTTPS: &str = "https://schema.org/RefurbishedCondition";
/// <https://schema.org/RefurbishedCondition>
pub const REFURBISHED_CONDITION_LABEL: &str = "RefurbishedCondition";
pub struct RefurbishedConditionIri;
impl PartialEq<&str> for RefurbishedConditionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == REFURBISHED_CONDITION_IRI_HTTP || *other == REFURBISHED_CONDITION_IRI_HTTPS
	}
}
impl PartialEq<RefurbishedConditionIri> for &str {
	fn eq(&self, other: &RefurbishedConditionIri) -> bool {
		*self == REFURBISHED_CONDITION_IRI_HTTP || *self == REFURBISHED_CONDITION_IRI_HTTPS
	}
}
pub struct RefurbishedConditionIriOrLabel;
impl PartialEq<&str> for RefurbishedConditionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RefurbishedConditionIri || *other == REFURBISHED_CONDITION_LABEL
	}
}
impl PartialEq<RefurbishedConditionIriOrLabel> for &str {
	fn eq(&self, other: &RefurbishedConditionIriOrLabel) -> bool {
		*self == RefurbishedConditionIri || *self == REFURBISHED_CONDITION_LABEL
	}
}
