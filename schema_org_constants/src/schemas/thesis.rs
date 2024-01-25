/// <https://schema.org/Thesis>
pub const THESIS_IRI_HTTP: &str = "http://schema.org/Thesis";
/// <https://schema.org/Thesis>
pub const THESIS_IRI_HTTPS: &str = "https://schema.org/Thesis";
/// <https://schema.org/Thesis>
pub const THESIS_LABEL: &str = "Thesis";
pub struct ThesisIri;
impl PartialEq<&str> for ThesisIri {
	fn eq(&self, other: &&str) -> bool {
		*other == THESIS_IRI_HTTP || *other == THESIS_IRI_HTTPS
	}
}
impl PartialEq<ThesisIri> for &str {
	fn eq(&self, other: &ThesisIri) -> bool {
		*self == THESIS_IRI_HTTP || *self == THESIS_IRI_HTTPS
	}
}
pub struct ThesisIriOrLabel;
impl PartialEq<&str> for ThesisIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ThesisIri || *other == THESIS_LABEL
	}
}
impl PartialEq<ThesisIriOrLabel> for &str {
	fn eq(&self, other: &ThesisIriOrLabel) -> bool {
		*self == ThesisIri || *self == THESIS_LABEL
	}
}
