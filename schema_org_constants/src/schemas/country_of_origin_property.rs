/// <https://schema.org/countryOfOrigin>
pub const COUNTRY_OF_ORIGIN_PROPERTY_IRI_HTTP: &str = "http://schema.org/countryOfOrigin";
/// <https://schema.org/countryOfOrigin>
pub const COUNTRY_OF_ORIGIN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/countryOfOrigin";
/// <https://schema.org/countryOfOrigin>
pub const COUNTRY_OF_ORIGIN_PROPERTY_LABEL: &str = "countryOfOrigin";
pub struct CountryOfOriginPropertyIri;
impl PartialEq<&str> for CountryOfOriginPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COUNTRY_OF_ORIGIN_PROPERTY_IRI_HTTP
			|| *other == COUNTRY_OF_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CountryOfOriginPropertyIri> for &str {
	fn eq(&self, other: &CountryOfOriginPropertyIri) -> bool {
		*self == COUNTRY_OF_ORIGIN_PROPERTY_IRI_HTTP
			|| *self == COUNTRY_OF_ORIGIN_PROPERTY_IRI_HTTPS
	}
}
pub struct CountryOfOriginPropertyIriOrLabel;
impl PartialEq<&str> for CountryOfOriginPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CountryOfOriginPropertyIri || *other == COUNTRY_OF_ORIGIN_PROPERTY_LABEL
	}
}
impl PartialEq<CountryOfOriginPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CountryOfOriginPropertyIriOrLabel) -> bool {
		*self == CountryOfOriginPropertyIri || *self == COUNTRY_OF_ORIGIN_PROPERTY_LABEL
	}
}
