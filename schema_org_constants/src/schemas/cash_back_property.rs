/// <https://schema.org/cashBack>
pub const CASH_BACK_PROPERTY_IRI_HTTP: &str = "http://schema.org/cashBack";
/// <https://schema.org/cashBack>
pub const CASH_BACK_PROPERTY_IRI_HTTPS: &str = "https://schema.org/cashBack";
/// <https://schema.org/cashBack>
pub const CASH_BACK_PROPERTY_LABEL: &str = "cashBack";
pub struct CashBackPropertyIri;
impl PartialEq<&str> for CashBackPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CASH_BACK_PROPERTY_IRI_HTTP || *other == CASH_BACK_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CashBackPropertyIri> for &str {
	fn eq(&self, other: &CashBackPropertyIri) -> bool {
		*self == CASH_BACK_PROPERTY_IRI_HTTP || *self == CASH_BACK_PROPERTY_IRI_HTTPS
	}
}
pub struct CashBackPropertyIriOrLabel;
impl PartialEq<&str> for CashBackPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CashBackPropertyIri || *other == CASH_BACK_PROPERTY_LABEL
	}
}
impl PartialEq<CashBackPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CashBackPropertyIriOrLabel) -> bool {
		*self == CashBackPropertyIri || *self == CASH_BACK_PROPERTY_LABEL
	}
}
