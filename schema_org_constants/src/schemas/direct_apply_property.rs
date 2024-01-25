/// <https://schema.org/directApply>
pub const DIRECT_APPLY_PROPERTY_IRI_HTTP: &str = "http://schema.org/directApply";
/// <https://schema.org/directApply>
pub const DIRECT_APPLY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/directApply";
/// <https://schema.org/directApply>
pub const DIRECT_APPLY_PROPERTY_LABEL: &str = "directApply";
pub struct DirectApplyPropertyIri;
impl PartialEq<&str> for DirectApplyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIRECT_APPLY_PROPERTY_IRI_HTTP || *other == DIRECT_APPLY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DirectApplyPropertyIri> for &str {
	fn eq(&self, other: &DirectApplyPropertyIri) -> bool {
		*self == DIRECT_APPLY_PROPERTY_IRI_HTTP || *self == DIRECT_APPLY_PROPERTY_IRI_HTTPS
	}
}
pub struct DirectApplyPropertyIriOrLabel;
impl PartialEq<&str> for DirectApplyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DirectApplyPropertyIri || *other == DIRECT_APPLY_PROPERTY_LABEL
	}
}
impl PartialEq<DirectApplyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DirectApplyPropertyIriOrLabel) -> bool {
		*self == DirectApplyPropertyIri || *self == DIRECT_APPLY_PROPERTY_LABEL
	}
}
