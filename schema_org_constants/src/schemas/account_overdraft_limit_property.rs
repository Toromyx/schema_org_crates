/// <https://schema.org/accountOverdraftLimit>
pub const ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/accountOverdraftLimit";
/// <https://schema.org/accountOverdraftLimit>
pub const ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accountOverdraftLimit";
/// <https://schema.org/accountOverdraftLimit>
pub const ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_LABEL: &str = "accountOverdraftLimit";
pub struct AccountOverdraftLimitPropertyIri;
impl PartialEq<&str> for AccountOverdraftLimitPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_IRI_HTTP
			|| *other == ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccountOverdraftLimitPropertyIri> for &str {
	fn eq(&self, other: &AccountOverdraftLimitPropertyIri) -> bool {
		*self == ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_IRI_HTTP
			|| *self == ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_IRI_HTTPS
	}
}
pub struct AccountOverdraftLimitPropertyIriOrLabel;
impl PartialEq<&str> for AccountOverdraftLimitPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccountOverdraftLimitPropertyIri
			|| *other == ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_LABEL
	}
}
impl PartialEq<AccountOverdraftLimitPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccountOverdraftLimitPropertyIriOrLabel) -> bool {
		*self == AccountOverdraftLimitPropertyIri || *self == ACCOUNT_OVERDRAFT_LIMIT_PROPERTY_LABEL
	}
}
