/// <https://schema.org/address>
pub const ADDRESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/address";
/// <https://schema.org/address>
pub const ADDRESS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/address";
/// <https://schema.org/address>
pub const ADDRESS_PROPERTY_LABEL: &str = "address";
pub struct AddressPropertyIri;
impl PartialEq<&str> for AddressPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDRESS_PROPERTY_IRI_HTTP || *other == ADDRESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AddressPropertyIri> for &str {
	fn eq(&self, other: &AddressPropertyIri) -> bool {
		*self == ADDRESS_PROPERTY_IRI_HTTP || *self == ADDRESS_PROPERTY_IRI_HTTPS
	}
}
pub struct AddressPropertyIriOrLabel;
impl PartialEq<&str> for AddressPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AddressPropertyIri || *other == ADDRESS_PROPERTY_LABEL
	}
}
impl PartialEq<AddressPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AddressPropertyIriOrLabel) -> bool {
		*self == AddressPropertyIri || *self == ADDRESS_PROPERTY_LABEL
	}
}
