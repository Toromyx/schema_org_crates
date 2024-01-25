/// <https://schema.org/ExchangeRateSpecification>
pub const EXCHANGE_RATE_SPECIFICATION_IRI_HTTP: &str =
	"http://schema.org/ExchangeRateSpecification";
/// <https://schema.org/ExchangeRateSpecification>
pub const EXCHANGE_RATE_SPECIFICATION_IRI_HTTPS: &str =
	"https://schema.org/ExchangeRateSpecification";
/// <https://schema.org/ExchangeRateSpecification>
pub const EXCHANGE_RATE_SPECIFICATION_LABEL: &str = "ExchangeRateSpecification";
pub struct ExchangeRateSpecificationIri;
impl PartialEq<&str> for ExchangeRateSpecificationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXCHANGE_RATE_SPECIFICATION_IRI_HTTP
			|| *other == EXCHANGE_RATE_SPECIFICATION_IRI_HTTPS
	}
}
impl PartialEq<ExchangeRateSpecificationIri> for &str {
	fn eq(&self, other: &ExchangeRateSpecificationIri) -> bool {
		*self == EXCHANGE_RATE_SPECIFICATION_IRI_HTTP
			|| *self == EXCHANGE_RATE_SPECIFICATION_IRI_HTTPS
	}
}
pub struct ExchangeRateSpecificationIriOrLabel;
impl PartialEq<&str> for ExchangeRateSpecificationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExchangeRateSpecificationIri || *other == EXCHANGE_RATE_SPECIFICATION_LABEL
	}
}
impl PartialEq<ExchangeRateSpecificationIriOrLabel> for &str {
	fn eq(&self, other: &ExchangeRateSpecificationIriOrLabel) -> bool {
		*self == ExchangeRateSpecificationIri || *self == EXCHANGE_RATE_SPECIFICATION_LABEL
	}
}
