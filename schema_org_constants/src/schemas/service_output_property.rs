/// <https://schema.org/serviceOutput>
pub const SERVICE_OUTPUT_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceOutput";
/// <https://schema.org/serviceOutput>
pub const SERVICE_OUTPUT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceOutput";
/// <https://schema.org/serviceOutput>
pub const SERVICE_OUTPUT_PROPERTY_LABEL: &str = "serviceOutput";
pub struct ServiceOutputPropertyIri;
impl PartialEq<&str> for ServiceOutputPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_OUTPUT_PROPERTY_IRI_HTTP || *other == SERVICE_OUTPUT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceOutputPropertyIri> for &str {
	fn eq(&self, other: &ServiceOutputPropertyIri) -> bool {
		*self == SERVICE_OUTPUT_PROPERTY_IRI_HTTP || *self == SERVICE_OUTPUT_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceOutputPropertyIriOrLabel;
impl PartialEq<&str> for ServiceOutputPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceOutputPropertyIri || *other == SERVICE_OUTPUT_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceOutputPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceOutputPropertyIriOrLabel) -> bool {
		*self == ServiceOutputPropertyIri || *self == SERVICE_OUTPUT_PROPERTY_LABEL
	}
}
