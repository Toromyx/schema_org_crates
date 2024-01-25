/// <https://schema.org/recommendedIntake>
pub const RECOMMENDED_INTAKE_PROPERTY_IRI_HTTP: &str = "http://schema.org/recommendedIntake";
/// <https://schema.org/recommendedIntake>
pub const RECOMMENDED_INTAKE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/recommendedIntake";
/// <https://schema.org/recommendedIntake>
pub const RECOMMENDED_INTAKE_PROPERTY_LABEL: &str = "recommendedIntake";
pub struct RecommendedIntakePropertyIri;
impl PartialEq<&str> for RecommendedIntakePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RECOMMENDED_INTAKE_PROPERTY_IRI_HTTP
			|| *other == RECOMMENDED_INTAKE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<RecommendedIntakePropertyIri> for &str {
	fn eq(&self, other: &RecommendedIntakePropertyIri) -> bool {
		*self == RECOMMENDED_INTAKE_PROPERTY_IRI_HTTP
			|| *self == RECOMMENDED_INTAKE_PROPERTY_IRI_HTTPS
	}
}
pub struct RecommendedIntakePropertyIriOrLabel;
impl PartialEq<&str> for RecommendedIntakePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RecommendedIntakePropertyIri || *other == RECOMMENDED_INTAKE_PROPERTY_LABEL
	}
}
impl PartialEq<RecommendedIntakePropertyIriOrLabel> for &str {
	fn eq(&self, other: &RecommendedIntakePropertyIriOrLabel) -> bool {
		*self == RecommendedIntakePropertyIri || *self == RECOMMENDED_INTAKE_PROPERTY_LABEL
	}
}
