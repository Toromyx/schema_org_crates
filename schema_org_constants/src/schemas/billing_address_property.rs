/// <https://schema.org/billingAddress>
pub const BILLING_ADDRESS_PROPERTY_IRI_HTTP: &str = "http://schema.org/billingAddress";
/// <https://schema.org/billingAddress>
pub const BILLING_ADDRESS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/billingAddress";
/// <https://schema.org/billingAddress>
pub const BILLING_ADDRESS_PROPERTY_LABEL: &str = "billingAddress";
pub struct BillingAddressPropertyIri;
impl PartialEq<&str> for BillingAddressPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BILLING_ADDRESS_PROPERTY_IRI_HTTP || *other == BILLING_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<BillingAddressPropertyIri> for &str {
	fn eq(&self, other: &BillingAddressPropertyIri) -> bool {
		*self == BILLING_ADDRESS_PROPERTY_IRI_HTTP || *self == BILLING_ADDRESS_PROPERTY_IRI_HTTPS
	}
}
pub struct BillingAddressPropertyIriOrLabel;
impl PartialEq<&str> for BillingAddressPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BillingAddressPropertyIri || *other == BILLING_ADDRESS_PROPERTY_LABEL
	}
}
impl PartialEq<BillingAddressPropertyIriOrLabel> for &str {
	fn eq(&self, other: &BillingAddressPropertyIriOrLabel) -> bool {
		*self == BillingAddressPropertyIri || *self == BILLING_ADDRESS_PROPERTY_LABEL
	}
}
