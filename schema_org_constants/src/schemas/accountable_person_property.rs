/// <https://schema.org/accountablePerson>
pub const ACCOUNTABLE_PERSON_PROPERTY_IRI_HTTP: &str = "http://schema.org/accountablePerson";
/// <https://schema.org/accountablePerson>
pub const ACCOUNTABLE_PERSON_PROPERTY_IRI_HTTPS: &str = "https://schema.org/accountablePerson";
/// <https://schema.org/accountablePerson>
pub const ACCOUNTABLE_PERSON_PROPERTY_LABEL: &str = "accountablePerson";
pub struct AccountablePersonPropertyIri;
impl PartialEq<&str> for AccountablePersonPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOUNTABLE_PERSON_PROPERTY_IRI_HTTP
			|| *other == ACCOUNTABLE_PERSON_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccountablePersonPropertyIri> for &str {
	fn eq(&self, other: &AccountablePersonPropertyIri) -> bool {
		*self == ACCOUNTABLE_PERSON_PROPERTY_IRI_HTTP
			|| *self == ACCOUNTABLE_PERSON_PROPERTY_IRI_HTTPS
	}
}
pub struct AccountablePersonPropertyIriOrLabel;
impl PartialEq<&str> for AccountablePersonPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccountablePersonPropertyIri || *other == ACCOUNTABLE_PERSON_PROPERTY_LABEL
	}
}
impl PartialEq<AccountablePersonPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccountablePersonPropertyIriOrLabel) -> bool {
		*self == AccountablePersonPropertyIri || *self == ACCOUNTABLE_PERSON_PROPERTY_LABEL
	}
}
