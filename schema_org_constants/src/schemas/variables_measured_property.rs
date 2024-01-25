/// <https://schema.org/variablesMeasured>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const VARIABLES_MEASURED_PROPERTY_IRI_HTTP: &str = "http://schema.org/variablesMeasured";
/// <https://schema.org/variablesMeasured>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const VARIABLES_MEASURED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/variablesMeasured";
/// <https://schema.org/variablesMeasured>
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub const VARIABLES_MEASURED_PROPERTY_LABEL: &str = "variablesMeasured";
pub struct VariablesMeasuredPropertyIri;
impl PartialEq<&str> for VariablesMeasuredPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VARIABLES_MEASURED_PROPERTY_IRI_HTTP
			|| *other == VARIABLES_MEASURED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<VariablesMeasuredPropertyIri> for &str {
	fn eq(&self, other: &VariablesMeasuredPropertyIri) -> bool {
		*self == VARIABLES_MEASURED_PROPERTY_IRI_HTTP
			|| *self == VARIABLES_MEASURED_PROPERTY_IRI_HTTPS
	}
}
pub struct VariablesMeasuredPropertyIriOrLabel;
impl PartialEq<&str> for VariablesMeasuredPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VariablesMeasuredPropertyIri || *other == VARIABLES_MEASURED_PROPERTY_LABEL
	}
}
impl PartialEq<VariablesMeasuredPropertyIriOrLabel> for &str {
	fn eq(&self, other: &VariablesMeasuredPropertyIriOrLabel) -> bool {
		*self == VariablesMeasuredPropertyIri || *self == VARIABLES_MEASURED_PROPERTY_LABEL
	}
}
