/// <https://schema.org/DamagedCondition>
pub const DAMAGED_CONDITION_IRI_HTTP: &str = "http://schema.org/DamagedCondition";
/// <https://schema.org/DamagedCondition>
pub const DAMAGED_CONDITION_IRI_HTTPS: &str = "https://schema.org/DamagedCondition";
/// <https://schema.org/DamagedCondition>
pub const DAMAGED_CONDITION_LABEL: &str = "DamagedCondition";
pub struct DamagedConditionIri;
impl PartialEq<&str> for DamagedConditionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DAMAGED_CONDITION_IRI_HTTP || *other == DAMAGED_CONDITION_IRI_HTTPS
	}
}
impl PartialEq<DamagedConditionIri> for &str {
	fn eq(&self, other: &DamagedConditionIri) -> bool {
		*self == DAMAGED_CONDITION_IRI_HTTP || *self == DAMAGED_CONDITION_IRI_HTTPS
	}
}
pub struct DamagedConditionIriOrLabel;
impl PartialEq<&str> for DamagedConditionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DamagedConditionIri || *other == DAMAGED_CONDITION_LABEL
	}
}
impl PartialEq<DamagedConditionIriOrLabel> for &str {
	fn eq(&self, other: &DamagedConditionIriOrLabel) -> bool {
		*self == DamagedConditionIri || *self == DAMAGED_CONDITION_LABEL
	}
}
