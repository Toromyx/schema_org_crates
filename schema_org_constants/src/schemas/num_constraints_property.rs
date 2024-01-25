/// <https://schema.org/numConstraints>
pub const NUM_CONSTRAINTS_PROPERTY_IRI_HTTP: &str = "http://schema.org/numConstraints";
/// <https://schema.org/numConstraints>
pub const NUM_CONSTRAINTS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/numConstraints";
/// <https://schema.org/numConstraints>
pub const NUM_CONSTRAINTS_PROPERTY_LABEL: &str = "numConstraints";
pub struct NumConstraintsPropertyIri;
impl PartialEq<&str> for NumConstraintsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NUM_CONSTRAINTS_PROPERTY_IRI_HTTP || *other == NUM_CONSTRAINTS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NumConstraintsPropertyIri> for &str {
	fn eq(&self, other: &NumConstraintsPropertyIri) -> bool {
		*self == NUM_CONSTRAINTS_PROPERTY_IRI_HTTP || *self == NUM_CONSTRAINTS_PROPERTY_IRI_HTTPS
	}
}
pub struct NumConstraintsPropertyIriOrLabel;
impl PartialEq<&str> for NumConstraintsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NumConstraintsPropertyIri || *other == NUM_CONSTRAINTS_PROPERTY_LABEL
	}
}
impl PartialEq<NumConstraintsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NumConstraintsPropertyIriOrLabel) -> bool {
		*self == NumConstraintsPropertyIri || *self == NUM_CONSTRAINTS_PROPERTY_LABEL
	}
}
