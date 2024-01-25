/// <https://schema.org/bankAccountType>
pub const BANK_ACCOUNT_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/bankAccountType";
/// <https://schema.org/bankAccountType>
pub const BANK_ACCOUNT_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/bankAccountType";
/// <https://schema.org/bankAccountType>
pub const BANK_ACCOUNT_TYPE_PROPERTY_LABEL: &str = "bankAccountType";
pub struct BankAccountTypePropertyIri;
impl PartialEq<&str> for BankAccountTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BANK_ACCOUNT_TYPE_PROPERTY_IRI_HTTP
			|| *other == BANK_ACCOUNT_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BankAccountTypePropertyIri> for &str {
	fn eq(&self, other: &BankAccountTypePropertyIri) -> bool {
		*self == BANK_ACCOUNT_TYPE_PROPERTY_IRI_HTTP
			|| *self == BANK_ACCOUNT_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct BankAccountTypePropertyIriOrLabel;
impl PartialEq<&str> for BankAccountTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BankAccountTypePropertyIri || *other == BANK_ACCOUNT_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<BankAccountTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &BankAccountTypePropertyIriOrLabel) -> bool {
		*self == BankAccountTypePropertyIri || *self == BANK_ACCOUNT_TYPE_PROPERTY_LABEL
	}
}
