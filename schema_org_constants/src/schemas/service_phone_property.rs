/// <https://schema.org/servicePhone>
pub const SERVICE_PHONE_PROPERTY_IRI_HTTP: &str = "http://schema.org/servicePhone";
/// <https://schema.org/servicePhone>
pub const SERVICE_PHONE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/servicePhone";
/// <https://schema.org/servicePhone>
pub const SERVICE_PHONE_PROPERTY_LABEL: &str = "servicePhone";
pub struct ServicePhonePropertyIri;
impl PartialEq<&str> for ServicePhonePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_PHONE_PROPERTY_IRI_HTTP || *other == SERVICE_PHONE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServicePhonePropertyIri> for &str {
	fn eq(&self, other: &ServicePhonePropertyIri) -> bool {
		*self == SERVICE_PHONE_PROPERTY_IRI_HTTP || *self == SERVICE_PHONE_PROPERTY_IRI_HTTPS
	}
}
pub struct ServicePhonePropertyIriOrLabel;
impl PartialEq<&str> for ServicePhonePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServicePhonePropertyIri || *other == SERVICE_PHONE_PROPERTY_LABEL
	}
}
impl PartialEq<ServicePhonePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServicePhonePropertyIriOrLabel) -> bool {
		*self == ServicePhonePropertyIri || *self == SERVICE_PHONE_PROPERTY_LABEL
	}
}
