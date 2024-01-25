/// <https://schema.org/serviceOperator>
pub const SERVICE_OPERATOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceOperator";
/// <https://schema.org/serviceOperator>
pub const SERVICE_OPERATOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceOperator";
/// <https://schema.org/serviceOperator>
pub const SERVICE_OPERATOR_PROPERTY_LABEL: &str = "serviceOperator";
pub struct ServiceOperatorPropertyIri;
impl PartialEq<&str> for ServiceOperatorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_OPERATOR_PROPERTY_IRI_HTTP
			|| *other == SERVICE_OPERATOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceOperatorPropertyIri> for &str {
	fn eq(&self, other: &ServiceOperatorPropertyIri) -> bool {
		*self == SERVICE_OPERATOR_PROPERTY_IRI_HTTP || *self == SERVICE_OPERATOR_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceOperatorPropertyIriOrLabel;
impl PartialEq<&str> for ServiceOperatorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceOperatorPropertyIri || *other == SERVICE_OPERATOR_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceOperatorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceOperatorPropertyIriOrLabel) -> bool {
		*self == ServiceOperatorPropertyIri || *self == SERVICE_OPERATOR_PROPERTY_LABEL
	}
}
