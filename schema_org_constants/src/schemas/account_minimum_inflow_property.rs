/// <https://schema.org/accountMinimumInflow>
pub const ACCOUNT_MINIMUM_INFLOW_PROPERTY_IRI_HTTP: &str = "http://schema.org/accountMinimumInflow";
/// <https://schema.org/accountMinimumInflow>
pub const ACCOUNT_MINIMUM_INFLOW_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/accountMinimumInflow";
/// <https://schema.org/accountMinimumInflow>
pub const ACCOUNT_MINIMUM_INFLOW_PROPERTY_LABEL: &str = "accountMinimumInflow";
pub struct AccountMinimumInflowPropertyIri;
impl PartialEq<&str> for AccountMinimumInflowPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOUNT_MINIMUM_INFLOW_PROPERTY_IRI_HTTP
			|| *other == ACCOUNT_MINIMUM_INFLOW_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AccountMinimumInflowPropertyIri> for &str {
	fn eq(&self, other: &AccountMinimumInflowPropertyIri) -> bool {
		*self == ACCOUNT_MINIMUM_INFLOW_PROPERTY_IRI_HTTP
			|| *self == ACCOUNT_MINIMUM_INFLOW_PROPERTY_IRI_HTTPS
	}
}
pub struct AccountMinimumInflowPropertyIriOrLabel;
impl PartialEq<&str> for AccountMinimumInflowPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccountMinimumInflowPropertyIri || *other == ACCOUNT_MINIMUM_INFLOW_PROPERTY_LABEL
	}
}
impl PartialEq<AccountMinimumInflowPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AccountMinimumInflowPropertyIriOrLabel) -> bool {
		*self == AccountMinimumInflowPropertyIri || *self == ACCOUNT_MINIMUM_INFLOW_PROPERTY_LABEL
	}
}
