/// <https://schema.org/servicePostalAddress>
pub const SERVICE_POSTAL_ADDRESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/servicePostalAddress";
/// <https://schema.org/servicePostalAddress>
pub const SERVICE_POSTAL_ADDRESS_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/servicePostalAddress";
/// <https://schema.org/servicePostalAddress>
pub const SERVICE_POSTAL_ADDRESS_PROPERTY_LABEL: &str = "servicePostalAddress";
pub struct ServicePostalAddressPropertyIri;
impl PartialEq<&str> for ServicePostalAddressPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_POSTAL_ADDRESS_PROPERTY_IRI_HTTP
			|| *other == SERVICE_POSTAL_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServicePostalAddressPropertyIri> for &str {
	fn eq(&self, other: &ServicePostalAddressPropertyIri) -> bool {
		*self == SERVICE_POSTAL_ADDRESS_PROPERTY_IRI_HTTP
			|| *self == SERVICE_POSTAL_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
pub struct ServicePostalAddressPropertyIriOrLabel;
impl PartialEq<&str> for ServicePostalAddressPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServicePostalAddressPropertyIri || *other == SERVICE_POSTAL_ADDRESS_PROPERTY_LABEL
	}
}
impl PartialEq<ServicePostalAddressPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServicePostalAddressPropertyIriOrLabel) -> bool {
		*self == ServicePostalAddressPropertyIri || *self == SERVICE_POSTAL_ADDRESS_PROPERTY_LABEL
	}
}
