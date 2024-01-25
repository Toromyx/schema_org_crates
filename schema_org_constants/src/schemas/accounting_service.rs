/// <https://schema.org/AccountingService>
pub const ACCOUNTING_SERVICE_IRI_HTTP: &str = "http://schema.org/AccountingService";
/// <https://schema.org/AccountingService>
pub const ACCOUNTING_SERVICE_IRI_HTTPS: &str = "https://schema.org/AccountingService";
/// <https://schema.org/AccountingService>
pub const ACCOUNTING_SERVICE_LABEL: &str = "AccountingService";
pub struct AccountingServiceIri;
impl PartialEq<&str> for AccountingServiceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACCOUNTING_SERVICE_IRI_HTTP || *other == ACCOUNTING_SERVICE_IRI_HTTPS
	}
}
impl PartialEq<AccountingServiceIri> for &str {
	fn eq(&self, other: &AccountingServiceIri) -> bool {
		*self == ACCOUNTING_SERVICE_IRI_HTTP || *self == ACCOUNTING_SERVICE_IRI_HTTPS
	}
}
pub struct AccountingServiceIriOrLabel;
impl PartialEq<&str> for AccountingServiceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AccountingServiceIri || *other == ACCOUNTING_SERVICE_LABEL
	}
}
impl PartialEq<AccountingServiceIriOrLabel> for &str {
	fn eq(&self, other: &AccountingServiceIriOrLabel) -> bool {
		*self == AccountingServiceIri || *self == ACCOUNTING_SERVICE_LABEL
	}
}
