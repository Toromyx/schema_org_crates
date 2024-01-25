/// <https://schema.org/countryOfLastProcessing>
pub const COUNTRY_OF_LAST_PROCESSING_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/countryOfLastProcessing";
/// <https://schema.org/countryOfLastProcessing>
pub const COUNTRY_OF_LAST_PROCESSING_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/countryOfLastProcessing";
/// <https://schema.org/countryOfLastProcessing>
pub const COUNTRY_OF_LAST_PROCESSING_PROPERTY_LABEL: &str = "countryOfLastProcessing";
pub struct CountryOfLastProcessingPropertyIri;
impl PartialEq<&str> for CountryOfLastProcessingPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COUNTRY_OF_LAST_PROCESSING_PROPERTY_IRI_HTTP
			|| *other == COUNTRY_OF_LAST_PROCESSING_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CountryOfLastProcessingPropertyIri> for &str {
	fn eq(&self, other: &CountryOfLastProcessingPropertyIri) -> bool {
		*self == COUNTRY_OF_LAST_PROCESSING_PROPERTY_IRI_HTTP
			|| *self == COUNTRY_OF_LAST_PROCESSING_PROPERTY_IRI_HTTPS
	}
}
pub struct CountryOfLastProcessingPropertyIriOrLabel;
impl PartialEq<&str> for CountryOfLastProcessingPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CountryOfLastProcessingPropertyIri
			|| *other == COUNTRY_OF_LAST_PROCESSING_PROPERTY_LABEL
	}
}
impl PartialEq<CountryOfLastProcessingPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CountryOfLastProcessingPropertyIriOrLabel) -> bool {
		*self == CountryOfLastProcessingPropertyIri
			|| *self == COUNTRY_OF_LAST_PROCESSING_PROPERTY_LABEL
	}
}
