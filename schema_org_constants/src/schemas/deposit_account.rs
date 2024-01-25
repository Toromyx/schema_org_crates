/// <https://schema.org/DepositAccount>
pub const DEPOSIT_ACCOUNT_IRI_HTTP: &str = "http://schema.org/DepositAccount";
/// <https://schema.org/DepositAccount>
pub const DEPOSIT_ACCOUNT_IRI_HTTPS: &str = "https://schema.org/DepositAccount";
/// <https://schema.org/DepositAccount>
pub const DEPOSIT_ACCOUNT_LABEL: &str = "DepositAccount";
pub struct DepositAccountIri;
impl PartialEq<&str> for DepositAccountIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPOSIT_ACCOUNT_IRI_HTTP || *other == DEPOSIT_ACCOUNT_IRI_HTTPS
	}
}
impl PartialEq<DepositAccountIri> for &str {
	fn eq(&self, other: &DepositAccountIri) -> bool {
		*self == DEPOSIT_ACCOUNT_IRI_HTTP || *self == DEPOSIT_ACCOUNT_IRI_HTTPS
	}
}
pub struct DepositAccountIriOrLabel;
impl PartialEq<&str> for DepositAccountIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepositAccountIri || *other == DEPOSIT_ACCOUNT_LABEL
	}
}
impl PartialEq<DepositAccountIriOrLabel> for &str {
	fn eq(&self, other: &DepositAccountIriOrLabel) -> bool {
		*self == DepositAccountIri || *self == DEPOSIT_ACCOUNT_LABEL
	}
}
