/// <https://schema.org/Balance>
pub const BALANCE_IRI_HTTP: &str = "http://schema.org/Balance";
/// <https://schema.org/Balance>
pub const BALANCE_IRI_HTTPS: &str = "https://schema.org/Balance";
/// <https://schema.org/Balance>
pub const BALANCE_LABEL: &str = "Balance";
pub struct BalanceIri;
impl PartialEq<&str> for BalanceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BALANCE_IRI_HTTP || *other == BALANCE_IRI_HTTPS
	}
}
impl PartialEq<BalanceIri> for &str {
	fn eq(&self, other: &BalanceIri) -> bool {
		*self == BALANCE_IRI_HTTP || *self == BALANCE_IRI_HTTPS
	}
}
pub struct BalanceIriOrLabel;
impl PartialEq<&str> for BalanceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BalanceIri || *other == BALANCE_LABEL
	}
}
impl PartialEq<BalanceIriOrLabel> for &str {
	fn eq(&self, other: &BalanceIriOrLabel) -> bool {
		*self == BalanceIri || *self == BALANCE_LABEL
	}
}
