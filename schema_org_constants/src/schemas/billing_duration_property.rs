/// <https://schema.org/billingDuration>
pub const BILLING_DURATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/billingDuration";
/// <https://schema.org/billingDuration>
pub const BILLING_DURATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/billingDuration";
/// <https://schema.org/billingDuration>
pub const BILLING_DURATION_PROPERTY_LABEL: &str = "billingDuration";
pub struct BillingDurationPropertyIri;
impl PartialEq<&str> for BillingDurationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BILLING_DURATION_PROPERTY_IRI_HTTP
			|| *other == BILLING_DURATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BillingDurationPropertyIri> for &str {
	fn eq(&self, other: &BillingDurationPropertyIri) -> bool {
		*self == BILLING_DURATION_PROPERTY_IRI_HTTP || *self == BILLING_DURATION_PROPERTY_IRI_HTTPS
	}
}
pub struct BillingDurationPropertyIriOrLabel;
impl PartialEq<&str> for BillingDurationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BillingDurationPropertyIri || *other == BILLING_DURATION_PROPERTY_LABEL
	}
}
impl PartialEq<BillingDurationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BillingDurationPropertyIriOrLabel) -> bool {
		*self == BillingDurationPropertyIri || *self == BILLING_DURATION_PROPERTY_LABEL
	}
}
