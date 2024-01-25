/// <https://schema.org/healthCondition>
pub const HEALTH_CONDITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/healthCondition";
/// <https://schema.org/healthCondition>
pub const HEALTH_CONDITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/healthCondition";
/// <https://schema.org/healthCondition>
pub const HEALTH_CONDITION_PROPERTY_LABEL: &str = "healthCondition";
pub struct HealthConditionPropertyIri;
impl PartialEq<&str> for HealthConditionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEALTH_CONDITION_PROPERTY_IRI_HTTP
			|| *other == HEALTH_CONDITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HealthConditionPropertyIri> for &str {
	fn eq(&self, other: &HealthConditionPropertyIri) -> bool {
		*self == HEALTH_CONDITION_PROPERTY_IRI_HTTP || *self == HEALTH_CONDITION_PROPERTY_IRI_HTTPS
	}
}
pub struct HealthConditionPropertyIriOrLabel;
impl PartialEq<&str> for HealthConditionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HealthConditionPropertyIri || *other == HEALTH_CONDITION_PROPERTY_LABEL
	}
}
impl PartialEq<HealthConditionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &HealthConditionPropertyIriOrLabel) -> bool {
		*self == HealthConditionPropertyIri || *self == HEALTH_CONDITION_PROPERTY_LABEL
	}
}
