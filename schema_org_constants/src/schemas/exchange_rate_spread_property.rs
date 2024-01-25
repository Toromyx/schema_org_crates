/// <https://schema.org/exchangeRateSpread>
pub const EXCHANGE_RATE_SPREAD_PROPERTY_IRI_HTTP: &str = "http://schema.org/exchangeRateSpread";
/// <https://schema.org/exchangeRateSpread>
pub const EXCHANGE_RATE_SPREAD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/exchangeRateSpread";
/// <https://schema.org/exchangeRateSpread>
pub const EXCHANGE_RATE_SPREAD_PROPERTY_LABEL: &str = "exchangeRateSpread";
pub struct ExchangeRateSpreadPropertyIri;
impl PartialEq<&str> for ExchangeRateSpreadPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == EXCHANGE_RATE_SPREAD_PROPERTY_IRI_HTTP
			|| *other == EXCHANGE_RATE_SPREAD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ExchangeRateSpreadPropertyIri> for &str {
	fn eq(&self, other: &ExchangeRateSpreadPropertyIri) -> bool {
		*self == EXCHANGE_RATE_SPREAD_PROPERTY_IRI_HTTP
			|| *self == EXCHANGE_RATE_SPREAD_PROPERTY_IRI_HTTPS
	}
}
pub struct ExchangeRateSpreadPropertyIriOrLabel;
impl PartialEq<&str> for ExchangeRateSpreadPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ExchangeRateSpreadPropertyIri || *other == EXCHANGE_RATE_SPREAD_PROPERTY_LABEL
	}
}
impl PartialEq<ExchangeRateSpreadPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ExchangeRateSpreadPropertyIriOrLabel) -> bool {
		*self == ExchangeRateSpreadPropertyIri || *self == EXCHANGE_RATE_SPREAD_PROPERTY_LABEL
	}
}
