/// <https://schema.org/currenciesAccepted>
pub const CURRENCIES_ACCEPTED_PROPERTY_IRI_HTTP: &str = "http://schema.org/currenciesAccepted";
/// <https://schema.org/currenciesAccepted>
pub const CURRENCIES_ACCEPTED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/currenciesAccepted";
/// <https://schema.org/currenciesAccepted>
pub const CURRENCIES_ACCEPTED_PROPERTY_LABEL: &str = "currenciesAccepted";
pub struct CurrenciesAcceptedPropertyIri;
impl PartialEq<&str> for CurrenciesAcceptedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CURRENCIES_ACCEPTED_PROPERTY_IRI_HTTP
			|| *other == CURRENCIES_ACCEPTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CurrenciesAcceptedPropertyIri> for &str {
	fn eq(&self, other: &CurrenciesAcceptedPropertyIri) -> bool {
		*self == CURRENCIES_ACCEPTED_PROPERTY_IRI_HTTP
			|| *self == CURRENCIES_ACCEPTED_PROPERTY_IRI_HTTPS
	}
}
pub struct CurrenciesAcceptedPropertyIriOrLabel;
impl PartialEq<&str> for CurrenciesAcceptedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CurrenciesAcceptedPropertyIri || *other == CURRENCIES_ACCEPTED_PROPERTY_LABEL
	}
}
impl PartialEq<CurrenciesAcceptedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CurrenciesAcceptedPropertyIriOrLabel) -> bool {
		*self == CurrenciesAcceptedPropertyIri || *self == CURRENCIES_ACCEPTED_PROPERTY_LABEL
	}
}
