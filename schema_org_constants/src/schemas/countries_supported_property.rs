/// <https://schema.org/countriesSupported>
pub const COUNTRIES_SUPPORTED_PROPERTY_IRI_HTTP: &str = "http://schema.org/countriesSupported";
/// <https://schema.org/countriesSupported>
pub const COUNTRIES_SUPPORTED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/countriesSupported";
/// <https://schema.org/countriesSupported>
pub const COUNTRIES_SUPPORTED_PROPERTY_LABEL: &str = "countriesSupported";
pub struct CountriesSupportedPropertyIri;
impl PartialEq<&str> for CountriesSupportedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COUNTRIES_SUPPORTED_PROPERTY_IRI_HTTP
			|| *other == COUNTRIES_SUPPORTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CountriesSupportedPropertyIri> for &str {
	fn eq(&self, other: &CountriesSupportedPropertyIri) -> bool {
		*self == COUNTRIES_SUPPORTED_PROPERTY_IRI_HTTP
			|| *self == COUNTRIES_SUPPORTED_PROPERTY_IRI_HTTPS
	}
}
pub struct CountriesSupportedPropertyIriOrLabel;
impl PartialEq<&str> for CountriesSupportedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CountriesSupportedPropertyIri || *other == COUNTRIES_SUPPORTED_PROPERTY_LABEL
	}
}
impl PartialEq<CountriesSupportedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CountriesSupportedPropertyIriOrLabel) -> bool {
		*self == CountriesSupportedPropertyIri || *self == COUNTRIES_SUPPORTED_PROPERTY_LABEL
	}
}
