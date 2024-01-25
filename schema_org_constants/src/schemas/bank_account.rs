/// <https://schema.org/BankAccount>
pub const BANK_ACCOUNT_IRI_HTTP: &str = "http://schema.org/BankAccount";
/// <https://schema.org/BankAccount>
pub const BANK_ACCOUNT_IRI_HTTPS: &str = "https://schema.org/BankAccount";
/// <https://schema.org/BankAccount>
pub const BANK_ACCOUNT_LABEL: &str = "BankAccount";
pub struct BankAccountIri;
impl PartialEq<&str> for BankAccountIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BANK_ACCOUNT_IRI_HTTP || *other == BANK_ACCOUNT_IRI_HTTPS
	}
}
impl PartialEq<BankAccountIri> for &str {
	fn eq(&self, other: &BankAccountIri) -> bool {
		*self == BANK_ACCOUNT_IRI_HTTP || *self == BANK_ACCOUNT_IRI_HTTPS
	}
}
pub struct BankAccountIriOrLabel;
impl PartialEq<&str> for BankAccountIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BankAccountIri || *other == BANK_ACCOUNT_LABEL
	}
}
impl PartialEq<BankAccountIriOrLabel> for &str {
	fn eq(&self, other: &BankAccountIriOrLabel) -> bool {
		*self == BankAccountIri || *self == BANK_ACCOUNT_LABEL
	}
}
