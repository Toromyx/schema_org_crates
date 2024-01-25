/// <https://schema.org/addressCountry>
pub const ADDRESS_COUNTRY_PROPERTY_IRI_HTTP: &str = "http://schema.org/addressCountry";
/// <https://schema.org/addressCountry>
pub const ADDRESS_COUNTRY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/addressCountry";
/// <https://schema.org/addressCountry>
pub const ADDRESS_COUNTRY_PROPERTY_LABEL: &str = "addressCountry";
pub struct AddressCountryPropertyIri;
impl PartialEq<&str> for AddressCountryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADDRESS_COUNTRY_PROPERTY_IRI_HTTP || *other == ADDRESS_COUNTRY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AddressCountryPropertyIri> for &str {
	fn eq(&self, other: &AddressCountryPropertyIri) -> bool {
		*self == ADDRESS_COUNTRY_PROPERTY_IRI_HTTP || *self == ADDRESS_COUNTRY_PROPERTY_IRI_HTTPS
	}
}
pub struct AddressCountryPropertyIriOrLabel;
impl PartialEq<&str> for AddressCountryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AddressCountryPropertyIri || *other == ADDRESS_COUNTRY_PROPERTY_LABEL
	}
}
impl PartialEq<AddressCountryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AddressCountryPropertyIriOrLabel) -> bool {
		*self == AddressCountryPropertyIri || *self == ADDRESS_COUNTRY_PROPERTY_LABEL
	}
}
