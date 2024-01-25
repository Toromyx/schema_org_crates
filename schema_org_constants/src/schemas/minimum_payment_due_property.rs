/// <https://schema.org/minimumPaymentDue>
pub const MINIMUM_PAYMENT_DUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/minimumPaymentDue";
/// <https://schema.org/minimumPaymentDue>
pub const MINIMUM_PAYMENT_DUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/minimumPaymentDue";
/// <https://schema.org/minimumPaymentDue>
pub const MINIMUM_PAYMENT_DUE_PROPERTY_LABEL: &str = "minimumPaymentDue";
pub struct MinimumPaymentDuePropertyIri;
impl PartialEq<&str> for MinimumPaymentDuePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MINIMUM_PAYMENT_DUE_PROPERTY_IRI_HTTP
			|| *other == MINIMUM_PAYMENT_DUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MinimumPaymentDuePropertyIri> for &str {
	fn eq(&self, other: &MinimumPaymentDuePropertyIri) -> bool {
		*self == MINIMUM_PAYMENT_DUE_PROPERTY_IRI_HTTP
			|| *self == MINIMUM_PAYMENT_DUE_PROPERTY_IRI_HTTPS
	}
}
pub struct MinimumPaymentDuePropertyIriOrLabel;
impl PartialEq<&str> for MinimumPaymentDuePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MinimumPaymentDuePropertyIri || *other == MINIMUM_PAYMENT_DUE_PROPERTY_LABEL
	}
}
impl PartialEq<MinimumPaymentDuePropertyIriOrLabel> for &str {
	fn eq(&self, other: &MinimumPaymentDuePropertyIriOrLabel) -> bool {
		*self == MinimumPaymentDuePropertyIri || *self == MINIMUM_PAYMENT_DUE_PROPERTY_LABEL
	}
}
