/// <https://schema.org/billingIncrement>
pub const BILLING_INCREMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/billingIncrement";
/// <https://schema.org/billingIncrement>
pub const BILLING_INCREMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/billingIncrement";
/// <https://schema.org/billingIncrement>
pub const BILLING_INCREMENT_PROPERTY_LABEL: &str = "billingIncrement";
pub struct BillingIncrementPropertyIri;
impl PartialEq<&str> for BillingIncrementPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BILLING_INCREMENT_PROPERTY_IRI_HTTP
			|| *other == BILLING_INCREMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BillingIncrementPropertyIri> for &str {
	fn eq(&self, other: &BillingIncrementPropertyIri) -> bool {
		*self == BILLING_INCREMENT_PROPERTY_IRI_HTTP
			|| *self == BILLING_INCREMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct BillingIncrementPropertyIriOrLabel;
impl PartialEq<&str> for BillingIncrementPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BillingIncrementPropertyIri || *other == BILLING_INCREMENT_PROPERTY_LABEL
	}
}
impl PartialEq<BillingIncrementPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BillingIncrementPropertyIriOrLabel) -> bool {
		*self == BillingIncrementPropertyIri || *self == BILLING_INCREMENT_PROPERTY_LABEL
	}
}
