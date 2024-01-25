/// <https://schema.org/streetAddress>
pub const STREET_ADDRESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/streetAddress";
/// <https://schema.org/streetAddress>
pub const STREET_ADDRESS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/streetAddress";
/// <https://schema.org/streetAddress>
pub const STREET_ADDRESS_PROPERTY_LABEL: &str = "streetAddress";
pub struct StreetAddressPropertyIri;
impl PartialEq<&str> for StreetAddressPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STREET_ADDRESS_PROPERTY_IRI_HTTP || *other == STREET_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StreetAddressPropertyIri> for &str {
	fn eq(&self, other: &StreetAddressPropertyIri) -> bool {
		*self == STREET_ADDRESS_PROPERTY_IRI_HTTP || *self == STREET_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
pub struct StreetAddressPropertyIriOrLabel;
impl PartialEq<&str> for StreetAddressPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StreetAddressPropertyIri || *other == STREET_ADDRESS_PROPERTY_LABEL
	}
}
impl PartialEq<StreetAddressPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StreetAddressPropertyIriOrLabel) -> bool {
		*self == StreetAddressPropertyIri || *self == STREET_ADDRESS_PROPERTY_LABEL
	}
}
