/// <https://schema.org/leaseLength>
pub const LEASE_LENGTH_PROPERTY_IRI_HTTP: &str = "http://schema.org/leaseLength";
/// <https://schema.org/leaseLength>
pub const LEASE_LENGTH_PROPERTY_IRI_HTTPS: &str = "https://schema.org/leaseLength";
/// <https://schema.org/leaseLength>
pub const LEASE_LENGTH_PROPERTY_LABEL: &str = "leaseLength";
pub struct LeaseLengthPropertyIri;
impl PartialEq<&str> for LeaseLengthPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LEASE_LENGTH_PROPERTY_IRI_HTTP || *other == LEASE_LENGTH_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LeaseLengthPropertyIri> for &str {
	fn eq(&self, other: &LeaseLengthPropertyIri) -> bool {
		*self == LEASE_LENGTH_PROPERTY_IRI_HTTP || *self == LEASE_LENGTH_PROPERTY_IRI_HTTPS
	}
}
pub struct LeaseLengthPropertyIriOrLabel;
impl PartialEq<&str> for LeaseLengthPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LeaseLengthPropertyIri || *other == LEASE_LENGTH_PROPERTY_LABEL
	}
}
impl PartialEq<LeaseLengthPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LeaseLengthPropertyIriOrLabel) -> bool {
		*self == LeaseLengthPropertyIri || *self == LEASE_LENGTH_PROPERTY_LABEL
	}
}
