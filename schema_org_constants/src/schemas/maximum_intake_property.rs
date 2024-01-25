/// <https://schema.org/maximumIntake>
pub const MAXIMUM_INTAKE_PROPERTY_IRI_HTTP: &str = "http://schema.org/maximumIntake";
/// <https://schema.org/maximumIntake>
pub const MAXIMUM_INTAKE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/maximumIntake";
/// <https://schema.org/maximumIntake>
pub const MAXIMUM_INTAKE_PROPERTY_LABEL: &str = "maximumIntake";
pub struct MaximumIntakePropertyIri;
impl PartialEq<&str> for MaximumIntakePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MAXIMUM_INTAKE_PROPERTY_IRI_HTTP || *other == MAXIMUM_INTAKE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MaximumIntakePropertyIri> for &str {
	fn eq(&self, other: &MaximumIntakePropertyIri) -> bool {
		*self == MAXIMUM_INTAKE_PROPERTY_IRI_HTTP || *self == MAXIMUM_INTAKE_PROPERTY_IRI_HTTPS
	}
}
pub struct MaximumIntakePropertyIriOrLabel;
impl PartialEq<&str> for MaximumIntakePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MaximumIntakePropertyIri || *other == MAXIMUM_INTAKE_PROPERTY_LABEL
	}
}
impl PartialEq<MaximumIntakePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MaximumIntakePropertyIriOrLabel) -> bool {
		*self == MaximumIntakePropertyIri || *self == MAXIMUM_INTAKE_PROPERTY_LABEL
	}
}
