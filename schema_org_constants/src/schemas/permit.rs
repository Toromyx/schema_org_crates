/// <https://schema.org/Permit>
pub const PERMIT_IRI_HTTP: &str = "http://schema.org/Permit";
/// <https://schema.org/Permit>
pub const PERMIT_IRI_HTTPS: &str = "https://schema.org/Permit";
/// <https://schema.org/Permit>
pub const PERMIT_LABEL: &str = "Permit";
pub struct PermitIri;
impl PartialEq<&str> for PermitIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERMIT_IRI_HTTP || *other == PERMIT_IRI_HTTPS
	}
}
impl PartialEq<PermitIri> for &str {
	fn eq(&self, other: &PermitIri) -> bool {
		*self == PERMIT_IRI_HTTP || *self == PERMIT_IRI_HTTPS
	}
}
pub struct PermitIriOrLabel;
impl PartialEq<&str> for PermitIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PermitIri || *other == PERMIT_LABEL
	}
}
impl PartialEq<PermitIriOrLabel> for &str {
	fn eq(&self, other: &PermitIriOrLabel) -> bool {
		*self == PermitIri || *self == PERMIT_LABEL
	}
}
