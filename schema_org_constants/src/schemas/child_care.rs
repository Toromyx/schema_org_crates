/// <https://schema.org/ChildCare>
pub const CHILD_CARE_IRI_HTTP: &str = "http://schema.org/ChildCare";
/// <https://schema.org/ChildCare>
pub const CHILD_CARE_IRI_HTTPS: &str = "https://schema.org/ChildCare";
/// <https://schema.org/ChildCare>
pub const CHILD_CARE_LABEL: &str = "ChildCare";
pub struct ChildCareIri;
impl PartialEq<&str> for ChildCareIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CHILD_CARE_IRI_HTTP || *other == CHILD_CARE_IRI_HTTPS
	}
}
impl PartialEq<ChildCareIri> for &str {
	fn eq(&self, other: &ChildCareIri) -> bool {
		*self == CHILD_CARE_IRI_HTTP || *self == CHILD_CARE_IRI_HTTPS
	}
}
pub struct ChildCareIriOrLabel;
impl PartialEq<&str> for ChildCareIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ChildCareIri || *other == CHILD_CARE_LABEL
	}
}
impl PartialEq<ChildCareIriOrLabel> for &str {
	fn eq(&self, other: &ChildCareIriOrLabel) -> bool {
		*self == ChildCareIri || *self == CHILD_CARE_LABEL
	}
}
