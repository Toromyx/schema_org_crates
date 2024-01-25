/// <https://schema.org/Surgical>
pub const SURGICAL_IRI_HTTP: &str = "http://schema.org/Surgical";
/// <https://schema.org/Surgical>
pub const SURGICAL_IRI_HTTPS: &str = "https://schema.org/Surgical";
/// <https://schema.org/Surgical>
pub const SURGICAL_LABEL: &str = "Surgical";
pub struct SurgicalIri;
impl PartialEq<&str> for SurgicalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SURGICAL_IRI_HTTP || *other == SURGICAL_IRI_HTTPS
	}
}
impl PartialEq<SurgicalIri> for &str {
	fn eq(&self, other: &SurgicalIri) -> bool {
		*self == SURGICAL_IRI_HTTP || *self == SURGICAL_IRI_HTTPS
	}
}
pub struct SurgicalIriOrLabel;
impl PartialEq<&str> for SurgicalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SurgicalIri || *other == SURGICAL_LABEL
	}
}
impl PartialEq<SurgicalIriOrLabel> for &str {
	fn eq(&self, other: &SurgicalIriOrLabel) -> bool {
		*self == SurgicalIri || *self == SURGICAL_LABEL
	}
}
