/// <https://schema.org/customer>
pub const CUSTOMER_PROPERTY_IRI_HTTP: &str = "http://schema.org/customer";
/// <https://schema.org/customer>
pub const CUSTOMER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/customer";
/// <https://schema.org/customer>
pub const CUSTOMER_PROPERTY_LABEL: &str = "customer";
pub struct CustomerPropertyIri;
impl PartialEq<&str> for CustomerPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CUSTOMER_PROPERTY_IRI_HTTP || *other == CUSTOMER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CustomerPropertyIri> for &str {
	fn eq(&self, other: &CustomerPropertyIri) -> bool {
		*self == CUSTOMER_PROPERTY_IRI_HTTP || *self == CUSTOMER_PROPERTY_IRI_HTTPS
	}
}
pub struct CustomerPropertyIriOrLabel;
impl PartialEq<&str> for CustomerPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CustomerPropertyIri || *other == CUSTOMER_PROPERTY_LABEL
	}
}
impl PartialEq<CustomerPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CustomerPropertyIriOrLabel) -> bool {
		*self == CustomerPropertyIri || *self == CUSTOMER_PROPERTY_LABEL
	}
}
