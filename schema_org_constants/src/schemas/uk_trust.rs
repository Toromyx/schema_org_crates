/// <https://schema.org/UKTrust>
pub const UK_TRUST_IRI_HTTP: &str = "http://schema.org/UKTrust";
/// <https://schema.org/UKTrust>
pub const UK_TRUST_IRI_HTTPS: &str = "https://schema.org/UKTrust";
/// <https://schema.org/UKTrust>
pub const UK_TRUST_LABEL: &str = "UKTrust";
pub struct UkTrustIri;
impl PartialEq<&str> for UkTrustIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UK_TRUST_IRI_HTTP || *other == UK_TRUST_IRI_HTTPS
	}
}
impl PartialEq<UkTrustIri> for &str {
	fn eq(&self, other: &UkTrustIri) -> bool {
		*self == UK_TRUST_IRI_HTTP || *self == UK_TRUST_IRI_HTTPS
	}
}
pub struct UkTrustIriOrLabel;
impl PartialEq<&str> for UkTrustIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UkTrustIri || *other == UK_TRUST_LABEL
	}
}
impl PartialEq<UkTrustIriOrLabel> for &str {
	fn eq(&self, other: &UkTrustIriOrLabel) -> bool {
		*self == UkTrustIri || *self == UK_TRUST_LABEL
	}
}
