/// <https://schema.org/variableMeasured>
pub const VARIABLE_MEASURED_PROPERTY_IRI_HTTP: &str = "http://schema.org/variableMeasured";
/// <https://schema.org/variableMeasured>
pub const VARIABLE_MEASURED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/variableMeasured";
/// <https://schema.org/variableMeasured>
pub const VARIABLE_MEASURED_PROPERTY_LABEL: &str = "variableMeasured";
pub struct VariableMeasuredPropertyIri;
impl PartialEq<&str> for VariableMeasuredPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VARIABLE_MEASURED_PROPERTY_IRI_HTTP
			|| *other == VARIABLE_MEASURED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VariableMeasuredPropertyIri> for &str {
	fn eq(&self, other: &VariableMeasuredPropertyIri) -> bool {
		*self == VARIABLE_MEASURED_PROPERTY_IRI_HTTP
			|| *self == VARIABLE_MEASURED_PROPERTY_IRI_HTTPS
	}
}
pub struct VariableMeasuredPropertyIriOrLabel;
impl PartialEq<&str> for VariableMeasuredPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VariableMeasuredPropertyIri || *other == VARIABLE_MEASURED_PROPERTY_LABEL
	}
}
impl PartialEq<VariableMeasuredPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VariableMeasuredPropertyIriOrLabel) -> bool {
		*self == VariableMeasuredPropertyIri || *self == VARIABLE_MEASURED_PROPERTY_LABEL
	}
}
