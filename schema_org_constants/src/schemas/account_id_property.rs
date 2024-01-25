/// <https://schema.org/accountId>
pub const ACCOUNT_ID_PROPERTY_IRI_HTTP: &str = "http://schema.org/accountId";
/// <https://schema.org/accountId>
pub const ACCOUNT_ID_PROPERTY_IRI_HTTPS: &str = "https://schema.org/accountId";
/// <https://schema.org/accountId>
pub const ACCOUNT_ID_PROPERTY_LABEL: &str = "accountId";
pub struct AccountIdPropertyIri;
impl PartialEq<&str> for AccountIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOUNT_ID_PROPERTY_IRI_HTTP || *other == ACCOUNT_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccountIdPropertyIri> for &str {
	fn eq(&self, other: &AccountIdPropertyIri) -> bool {
		*self == ACCOUNT_ID_PROPERTY_IRI_HTTP || *self == ACCOUNT_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct AccountIdPropertyIriOrLabel;
impl PartialEq<&str> for AccountIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccountIdPropertyIri || *other == ACCOUNT_ID_PROPERTY_LABEL
	}
}
impl PartialEq<AccountIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccountIdPropertyIriOrLabel) -> bool {
		*self == AccountIdPropertyIri || *self == ACCOUNT_ID_PROPERTY_LABEL
	}
}
