/// <https://schema.org/Service>
pub const SERVICE_IRI_HTTP: &str = "http://schema.org/Service";
/// <https://schema.org/Service>
pub const SERVICE_IRI_HTTPS: &str = "https://schema.org/Service";
/// <https://schema.org/Service>
pub const SERVICE_LABEL: &str = "Service";
pub struct ServiceIri;
impl PartialEq<&str> for ServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_IRI_HTTP || *other == SERVICE_IRI_HTTPS
	}
}
impl PartialEq<ServiceIri> for &str {
	fn eq(&self, other: &ServiceIri) -> bool {
		*self == SERVICE_IRI_HTTP || *self == SERVICE_IRI_HTTPS
	}
}
pub struct ServiceIriOrLabel;
impl PartialEq<&str> for ServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceIri || *other == SERVICE_LABEL
	}
}
impl PartialEq<ServiceIriOrLabel> for &str {
	fn eq(&self, other: &ServiceIriOrLabel) -> bool {
		*self == ServiceIri || *self == SERVICE_LABEL
	}
}
