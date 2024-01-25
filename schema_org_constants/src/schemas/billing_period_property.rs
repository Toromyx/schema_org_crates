/// <https://schema.org/billingPeriod>
pub const BILLING_PERIOD_PROPERTY_IRI_HTTP: &str = "http://schema.org/billingPeriod";
/// <https://schema.org/billingPeriod>
pub const BILLING_PERIOD_PROPERTY_IRI_HTTPS: &str = "https://schema.org/billingPeriod";
/// <https://schema.org/billingPeriod>
pub const BILLING_PERIOD_PROPERTY_LABEL: &str = "billingPeriod";
pub struct BillingPeriodPropertyIri;
impl PartialEq<&str> for BillingPeriodPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BILLING_PERIOD_PROPERTY_IRI_HTTP || *other == BILLING_PERIOD_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BillingPeriodPropertyIri> for &str {
	fn eq(&self, other: &BillingPeriodPropertyIri) -> bool {
		*self == BILLING_PERIOD_PROPERTY_IRI_HTTP || *self == BILLING_PERIOD_PROPERTY_IRI_HTTPS
	}
}
pub struct BillingPeriodPropertyIriOrLabel;
impl PartialEq<&str> for BillingPeriodPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BillingPeriodPropertyIri || *other == BILLING_PERIOD_PROPERTY_LABEL
	}
}
impl PartialEq<BillingPeriodPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BillingPeriodPropertyIriOrLabel) -> bool {
		*self == BillingPeriodPropertyIri || *self == BILLING_PERIOD_PROPERTY_LABEL
	}
}
