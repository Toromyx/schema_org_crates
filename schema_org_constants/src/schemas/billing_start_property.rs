/// <https://schema.org/billingStart>
pub const BILLING_START_PROPERTY_IRI_HTTP: &str = "http://schema.org/billingStart";
/// <https://schema.org/billingStart>
pub const BILLING_START_PROPERTY_IRI_HTTPS: &str = "https://schema.org/billingStart";
/// <https://schema.org/billingStart>
pub const BILLING_START_PROPERTY_LABEL: &str = "billingStart";
pub struct BillingStartPropertyIri;
impl PartialEq<&str> for BillingStartPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BILLING_START_PROPERTY_IRI_HTTP || *other == BILLING_START_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BillingStartPropertyIri> for &str {
	fn eq(&self, other: &BillingStartPropertyIri) -> bool {
		*self == BILLING_START_PROPERTY_IRI_HTTP || *self == BILLING_START_PROPERTY_IRI_HTTPS
	}
}
pub struct BillingStartPropertyIriOrLabel;
impl PartialEq<&str> for BillingStartPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BillingStartPropertyIri || *other == BILLING_START_PROPERTY_LABEL
	}
}
impl PartialEq<BillingStartPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BillingStartPropertyIriOrLabel) -> bool {
		*self == BillingStartPropertyIri || *self == BILLING_START_PROPERTY_LABEL
	}
}
