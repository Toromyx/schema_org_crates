/// <https://schema.org/BrokerageAccount>
pub const BROKERAGE_ACCOUNT_IRI_HTTP: &str = "http://schema.org/BrokerageAccount";
/// <https://schema.org/BrokerageAccount>
pub const BROKERAGE_ACCOUNT_IRI_HTTPS: &str = "https://schema.org/BrokerageAccount";
/// <https://schema.org/BrokerageAccount>
pub const BROKERAGE_ACCOUNT_LABEL: &str = "BrokerageAccount";
pub struct BrokerageAccountIri;
impl PartialEq<&str> for BrokerageAccountIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BROKERAGE_ACCOUNT_IRI_HTTP || *other == BROKERAGE_ACCOUNT_IRI_HTTPS
	}
}
impl PartialEq<BrokerageAccountIri> for &str {
	fn eq(&self, other: &BrokerageAccountIri) -> bool {
		*self == BROKERAGE_ACCOUNT_IRI_HTTP || *self == BROKERAGE_ACCOUNT_IRI_HTTPS
	}
}
pub struct BrokerageAccountIriOrLabel;
impl PartialEq<&str> for BrokerageAccountIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BrokerageAccountIri || *other == BROKERAGE_ACCOUNT_LABEL
	}
}
impl PartialEq<BrokerageAccountIriOrLabel> for &str {
	fn eq(&self, other: &BrokerageAccountIriOrLabel) -> bool {
		*self == BrokerageAccountIri || *self == BROKERAGE_ACCOUNT_LABEL
	}
}
