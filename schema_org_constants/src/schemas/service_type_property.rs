/// <https://schema.org/serviceType>
pub const SERVICE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceType";
/// <https://schema.org/serviceType>
pub const SERVICE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceType";
/// <https://schema.org/serviceType>
pub const SERVICE_TYPE_PROPERTY_LABEL: &str = "serviceType";
pub struct ServiceTypePropertyIri;
impl PartialEq<&str> for ServiceTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_TYPE_PROPERTY_IRI_HTTP || *other == SERVICE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceTypePropertyIri> for &str {
	fn eq(&self, other: &ServiceTypePropertyIri) -> bool {
		*self == SERVICE_TYPE_PROPERTY_IRI_HTTP || *self == SERVICE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceTypePropertyIriOrLabel;
impl PartialEq<&str> for ServiceTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceTypePropertyIri || *other == SERVICE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceTypePropertyIriOrLabel) -> bool {
		*self == ServiceTypePropertyIri || *self == SERVICE_TYPE_PROPERTY_LABEL
	}
}
