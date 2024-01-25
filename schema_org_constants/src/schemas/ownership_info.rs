/// <https://schema.org/OwnershipInfo>
pub const OWNERSHIP_INFO_IRI_HTTP: &str = "http://schema.org/OwnershipInfo";
/// <https://schema.org/OwnershipInfo>
pub const OWNERSHIP_INFO_IRI_HTTPS: &str = "https://schema.org/OwnershipInfo";
/// <https://schema.org/OwnershipInfo>
pub const OWNERSHIP_INFO_LABEL: &str = "OwnershipInfo";
pub struct OwnershipInfoIri;
impl PartialEq<&str> for OwnershipInfoIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OWNERSHIP_INFO_IRI_HTTP || *other == OWNERSHIP_INFO_IRI_HTTPS
	}
}
impl PartialEq<OwnershipInfoIri> for &str {
	fn eq(&self, other: &OwnershipInfoIri) -> bool {
		*self == OWNERSHIP_INFO_IRI_HTTP || *self == OWNERSHIP_INFO_IRI_HTTPS
	}
}
pub struct OwnershipInfoIriOrLabel;
impl PartialEq<&str> for OwnershipInfoIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OwnershipInfoIri || *other == OWNERSHIP_INFO_LABEL
	}
}
impl PartialEq<OwnershipInfoIriOrLabel> for &str {
	fn eq(&self, other: &OwnershipInfoIriOrLabel) -> bool {
		*self == OwnershipInfoIri || *self == OWNERSHIP_INFO_LABEL
	}
}
