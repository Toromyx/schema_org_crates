/// <https://schema.org/serviceLocation>
pub const SERVICE_LOCATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceLocation";
/// <https://schema.org/serviceLocation>
pub const SERVICE_LOCATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceLocation";
/// <https://schema.org/serviceLocation>
pub const SERVICE_LOCATION_PROPERTY_LABEL: &str = "serviceLocation";
pub struct ServiceLocationPropertyIri;
impl PartialEq<&str> for ServiceLocationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_LOCATION_PROPERTY_IRI_HTTP
			|| *other == SERVICE_LOCATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceLocationPropertyIri> for &str {
	fn eq(&self, other: &ServiceLocationPropertyIri) -> bool {
		*self == SERVICE_LOCATION_PROPERTY_IRI_HTTP || *self == SERVICE_LOCATION_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceLocationPropertyIriOrLabel;
impl PartialEq<&str> for ServiceLocationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceLocationPropertyIri || *other == SERVICE_LOCATION_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceLocationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceLocationPropertyIriOrLabel) -> bool {
		*self == ServiceLocationPropertyIri || *self == SERVICE_LOCATION_PROPERTY_LABEL
	}
}
