/// <https://schema.org/yearsInOperation>
pub const YEARS_IN_OPERATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/yearsInOperation";
/// <https://schema.org/yearsInOperation>
pub const YEARS_IN_OPERATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/yearsInOperation";
/// <https://schema.org/yearsInOperation>
pub const YEARS_IN_OPERATION_PROPERTY_LABEL: &str = "yearsInOperation";
pub struct YearsInOperationPropertyIri;
impl PartialEq<&str> for YearsInOperationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == YEARS_IN_OPERATION_PROPERTY_IRI_HTTP
			|| *other == YEARS_IN_OPERATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<YearsInOperationPropertyIri> for &str {
	fn eq(&self, other: &YearsInOperationPropertyIri) -> bool {
		*self == YEARS_IN_OPERATION_PROPERTY_IRI_HTTP
			|| *self == YEARS_IN_OPERATION_PROPERTY_IRI_HTTPS
	}
}
pub struct YearsInOperationPropertyIriOrLabel;
impl PartialEq<&str> for YearsInOperationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == YearsInOperationPropertyIri || *other == YEARS_IN_OPERATION_PROPERTY_LABEL
	}
}
impl PartialEq<YearsInOperationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &YearsInOperationPropertyIriOrLabel) -> bool {
		*self == YearsInOperationPropertyIri || *self == YEARS_IN_OPERATION_PROPERTY_LABEL
	}
}
