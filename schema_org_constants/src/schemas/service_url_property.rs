/// <https://schema.org/serviceUrl>
pub const SERVICE_URL_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceUrl";
/// <https://schema.org/serviceUrl>
pub const SERVICE_URL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceUrl";
/// <https://schema.org/serviceUrl>
pub const SERVICE_URL_PROPERTY_LABEL: &str = "serviceUrl";
pub struct ServiceUrlPropertyIri;
impl PartialEq<&str> for ServiceUrlPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_URL_PROPERTY_IRI_HTTP || *other == SERVICE_URL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceUrlPropertyIri> for &str {
	fn eq(&self, other: &ServiceUrlPropertyIri) -> bool {
		*self == SERVICE_URL_PROPERTY_IRI_HTTP || *self == SERVICE_URL_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceUrlPropertyIriOrLabel;
impl PartialEq<&str> for ServiceUrlPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceUrlPropertyIri || *other == SERVICE_URL_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceUrlPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceUrlPropertyIriOrLabel) -> bool {
		*self == ServiceUrlPropertyIri || *self == SERVICE_URL_PROPERTY_LABEL
	}
}
