/// <https://schema.org/constraintProperty>
pub const CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/constraintProperty";
/// <https://schema.org/constraintProperty>
pub const CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/constraintProperty";
/// <https://schema.org/constraintProperty>
pub const CONSTRAINT_PROPERTY_PROPERTY_LABEL: &str = "constraintProperty";
pub struct ConstraintPropertyPropertyIri;
impl PartialEq<&str> for ConstraintPropertyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTP
			|| *other == CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ConstraintPropertyPropertyIri> for &str {
	fn eq(&self, other: &ConstraintPropertyPropertyIri) -> bool {
		*self == CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTP
			|| *self == CONSTRAINT_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
pub struct ConstraintPropertyPropertyIriOrLabel;
impl PartialEq<&str> for ConstraintPropertyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ConstraintPropertyPropertyIri || *other == CONSTRAINT_PROPERTY_PROPERTY_LABEL
	}
}
impl PartialEq<ConstraintPropertyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ConstraintPropertyPropertyIriOrLabel) -> bool {
		*self == ConstraintPropertyPropertyIri || *self == CONSTRAINT_PROPERTY_PROPERTY_LABEL
	}
}
