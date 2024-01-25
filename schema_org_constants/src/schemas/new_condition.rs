/// <https://schema.org/NewCondition>
pub const NEW_CONDITION_IRI_HTTP: &str = "http://schema.org/NewCondition";
/// <https://schema.org/NewCondition>
pub const NEW_CONDITION_IRI_HTTPS: &str = "https://schema.org/NewCondition";
/// <https://schema.org/NewCondition>
pub const NEW_CONDITION_LABEL: &str = "NewCondition";
pub struct NewConditionIri;
impl PartialEq<&str> for NewConditionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEW_CONDITION_IRI_HTTP || *other == NEW_CONDITION_IRI_HTTPS
	}
}
impl PartialEq<NewConditionIri> for &str {
	fn eq(&self, other: &NewConditionIri) -> bool {
		*self == NEW_CONDITION_IRI_HTTP || *self == NEW_CONDITION_IRI_HTTPS
	}
}
pub struct NewConditionIriOrLabel;
impl PartialEq<&str> for NewConditionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NewConditionIri || *other == NEW_CONDITION_LABEL
	}
}
impl PartialEq<NewConditionIriOrLabel> for &str {
	fn eq(&self, other: &NewConditionIriOrLabel) -> bool {
		*self == NewConditionIri || *self == NEW_CONDITION_LABEL
	}
}
