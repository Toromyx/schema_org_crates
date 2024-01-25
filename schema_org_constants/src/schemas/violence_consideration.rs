/// <https://schema.org/ViolenceConsideration>
pub const VIOLENCE_CONSIDERATION_IRI_HTTP: &str = "http://schema.org/ViolenceConsideration";
/// <https://schema.org/ViolenceConsideration>
pub const VIOLENCE_CONSIDERATION_IRI_HTTPS: &str = "https://schema.org/ViolenceConsideration";
/// <https://schema.org/ViolenceConsideration>
pub const VIOLENCE_CONSIDERATION_LABEL: &str = "ViolenceConsideration";
pub struct ViolenceConsiderationIri;
impl PartialEq<&str> for ViolenceConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VIOLENCE_CONSIDERATION_IRI_HTTP || *other == VIOLENCE_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<ViolenceConsiderationIri> for &str {
	fn eq(&self, other: &ViolenceConsiderationIri) -> bool {
		*self == VIOLENCE_CONSIDERATION_IRI_HTTP || *self == VIOLENCE_CONSIDERATION_IRI_HTTPS
	}
}
pub struct ViolenceConsiderationIriOrLabel;
impl PartialEq<&str> for ViolenceConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ViolenceConsiderationIri || *other == VIOLENCE_CONSIDERATION_LABEL
	}
}
impl PartialEq<ViolenceConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &ViolenceConsiderationIriOrLabel) -> bool {
		*self == ViolenceConsiderationIri || *self == VIOLENCE_CONSIDERATION_LABEL
	}
}
