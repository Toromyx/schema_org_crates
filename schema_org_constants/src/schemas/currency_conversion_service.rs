/// <https://schema.org/CurrencyConversionService>
pub const CURRENCY_CONVERSION_SERVICE_IRI_HTTP: &str =
	"http://schema.org/CurrencyConversionService";
/// <https://schema.org/CurrencyConversionService>
pub const CURRENCY_CONVERSION_SERVICE_IRI_HTTPS: &str =
	"https://schema.org/CurrencyConversionService";
/// <https://schema.org/CurrencyConversionService>
pub const CURRENCY_CONVERSION_SERVICE_LABEL: &str = "CurrencyConversionService";
pub struct CurrencyConversionServiceIri;
impl PartialEq<&str> for CurrencyConversionServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CURRENCY_CONVERSION_SERVICE_IRI_HTTP
			|| *other == CURRENCY_CONVERSION_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<CurrencyConversionServiceIri> for &str {
	fn eq(&self, other: &CurrencyConversionServiceIri) -> bool {
		*self == CURRENCY_CONVERSION_SERVICE_IRI_HTTP
			|| *self == CURRENCY_CONVERSION_SERVICE_IRI_HTTPS
	}
}
pub struct CurrencyConversionServiceIriOrLabel;
impl PartialEq<&str> for CurrencyConversionServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CurrencyConversionServiceIri || *other == CURRENCY_CONVERSION_SERVICE_LABEL
	}
}
impl PartialEq<CurrencyConversionServiceIriOrLabel> for &str {
	fn eq(&self, other: &CurrencyConversionServiceIriOrLabel) -> bool {
		*self == CurrencyConversionServiceIri || *self == CURRENCY_CONVERSION_SERVICE_LABEL
	}
}
