/// <https://schema.org/countriesNotSupported>
pub const COUNTRIES_NOT_SUPPORTED_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/countriesNotSupported";
/// <https://schema.org/countriesNotSupported>
pub const COUNTRIES_NOT_SUPPORTED_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/countriesNotSupported";
/// <https://schema.org/countriesNotSupported>
pub const COUNTRIES_NOT_SUPPORTED_PROPERTY_LABEL: &str = "countriesNotSupported";
pub struct CountriesNotSupportedPropertyIri;
impl PartialEq<&str> for CountriesNotSupportedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COUNTRIES_NOT_SUPPORTED_PROPERTY_IRI_HTTP
			|| *other == COUNTRIES_NOT_SUPPORTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CountriesNotSupportedPropertyIri> for &str {
	fn eq(&self, other: &CountriesNotSupportedPropertyIri) -> bool {
		*self == COUNTRIES_NOT_SUPPORTED_PROPERTY_IRI_HTTP
			|| *self == COUNTRIES_NOT_SUPPORTED_PROPERTY_IRI_HTTPS
	}
}
pub struct CountriesNotSupportedPropertyIriOrLabel;
impl PartialEq<&str> for CountriesNotSupportedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CountriesNotSupportedPropertyIri
			|| *other == COUNTRIES_NOT_SUPPORTED_PROPERTY_LABEL
	}
}
impl PartialEq<CountriesNotSupportedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CountriesNotSupportedPropertyIriOrLabel) -> bool {
		*self == CountriesNotSupportedPropertyIri || *self == COUNTRIES_NOT_SUPPORTED_PROPERTY_LABEL
	}
}
