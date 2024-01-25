/// <https://schema.org/totalPaymentDue>
pub const TOTAL_PAYMENT_DUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/totalPaymentDue";
/// <https://schema.org/totalPaymentDue>
pub const TOTAL_PAYMENT_DUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/totalPaymentDue";
/// <https://schema.org/totalPaymentDue>
pub const TOTAL_PAYMENT_DUE_PROPERTY_LABEL: &str = "totalPaymentDue";
pub struct TotalPaymentDuePropertyIri;
impl PartialEq<&str> for TotalPaymentDuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TOTAL_PAYMENT_DUE_PROPERTY_IRI_HTTP
			|| *other == TOTAL_PAYMENT_DUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TotalPaymentDuePropertyIri> for &str {
	fn eq(&self, other: &TotalPaymentDuePropertyIri) -> bool {
		*self == TOTAL_PAYMENT_DUE_PROPERTY_IRI_HTTP
			|| *self == TOTAL_PAYMENT_DUE_PROPERTY_IRI_HTTPS
	}
}
pub struct TotalPaymentDuePropertyIriOrLabel;
impl PartialEq<&str> for TotalPaymentDuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TotalPaymentDuePropertyIri || *other == TOTAL_PAYMENT_DUE_PROPERTY_LABEL
	}
}
impl PartialEq<TotalPaymentDuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TotalPaymentDuePropertyIriOrLabel) -> bool {
		*self == TotalPaymentDuePropertyIri || *self == TOTAL_PAYMENT_DUE_PROPERTY_LABEL
	}
}
