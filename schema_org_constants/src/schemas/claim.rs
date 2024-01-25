/// <https://schema.org/Claim>
pub const CLAIM_IRI_HTTP: &str = "http://schema.org/Claim";
/// <https://schema.org/Claim>
pub const CLAIM_IRI_HTTPS: &str = "https://schema.org/Claim";
/// <https://schema.org/Claim>
pub const CLAIM_LABEL: &str = "Claim";
pub struct ClaimIri;
impl PartialEq<&str> for ClaimIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CLAIM_IRI_HTTP || *other == CLAIM_IRI_HTTPS
	}
}
impl PartialEq<ClaimIri> for &str {
	fn eq(&self, other: &ClaimIri) -> bool {
		*self == CLAIM_IRI_HTTP || *self == CLAIM_IRI_HTTPS
	}
}
pub struct ClaimIriOrLabel;
impl PartialEq<&str> for ClaimIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ClaimIri || *other == CLAIM_LABEL
	}
}
impl PartialEq<ClaimIriOrLabel> for &str {
	fn eq(&self, other: &ClaimIriOrLabel) -> bool {
		*self == ClaimIri || *self == CLAIM_LABEL
	}
}
