/// <https://schema.org/addressLocality>
pub const ADDRESS_LOCALITY_PROPERTY_IRI_HTTP: &str = "http://schema.org/addressLocality";
/// <https://schema.org/addressLocality>
pub const ADDRESS_LOCALITY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/addressLocality";
/// <https://schema.org/addressLocality>
pub const ADDRESS_LOCALITY_PROPERTY_LABEL: &str = "addressLocality";
pub struct AddressLocalityPropertyIri;
impl PartialEq<&str> for AddressLocalityPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDRESS_LOCALITY_PROPERTY_IRI_HTTP
			|| *other == ADDRESS_LOCALITY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AddressLocalityPropertyIri> for &str {
	fn eq(&self, other: &AddressLocalityPropertyIri) -> bool {
		*self == ADDRESS_LOCALITY_PROPERTY_IRI_HTTP || *self == ADDRESS_LOCALITY_PROPERTY_IRI_HTTPS
	}
}
pub struct AddressLocalityPropertyIriOrLabel;
impl PartialEq<&str> for AddressLocalityPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AddressLocalityPropertyIri || *other == ADDRESS_LOCALITY_PROPERTY_LABEL
	}
}
impl PartialEq<AddressLocalityPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AddressLocalityPropertyIriOrLabel) -> bool {
		*self == AddressLocalityPropertyIri || *self == ADDRESS_LOCALITY_PROPERTY_LABEL
	}
}
