/// <https://schema.org/currentExchangeRate>
pub const CURRENT_EXCHANGE_RATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/currentExchangeRate";
/// <https://schema.org/currentExchangeRate>
pub const CURRENT_EXCHANGE_RATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/currentExchangeRate";
/// <https://schema.org/currentExchangeRate>
pub const CURRENT_EXCHANGE_RATE_PROPERTY_LABEL: &str = "currentExchangeRate";
pub struct CurrentExchangeRatePropertyIri;
impl PartialEq<&str> for CurrentExchangeRatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CURRENT_EXCHANGE_RATE_PROPERTY_IRI_HTTP
			|| *other == CURRENT_EXCHANGE_RATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CurrentExchangeRatePropertyIri> for &str {
	fn eq(&self, other: &CurrentExchangeRatePropertyIri) -> bool {
		*self == CURRENT_EXCHANGE_RATE_PROPERTY_IRI_HTTP
			|| *self == CURRENT_EXCHANGE_RATE_PROPERTY_IRI_HTTPS
	}
}
pub struct CurrentExchangeRatePropertyIriOrLabel;
impl PartialEq<&str> for CurrentExchangeRatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CurrentExchangeRatePropertyIri || *other == CURRENT_EXCHANGE_RATE_PROPERTY_LABEL
	}
}
impl PartialEq<CurrentExchangeRatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &CurrentExchangeRatePropertyIriOrLabel) -> bool {
		*self == CurrentExchangeRatePropertyIri || *self == CURRENT_EXCHANGE_RATE_PROPERTY_LABEL
	}
}
