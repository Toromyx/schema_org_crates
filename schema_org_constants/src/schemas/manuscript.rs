/// <https://schema.org/Manuscript>
pub const MANUSCRIPT_IRI_HTTP: &str = "http://schema.org/Manuscript";
/// <https://schema.org/Manuscript>
pub const MANUSCRIPT_IRI_HTTPS: &str = "https://schema.org/Manuscript";
/// <https://schema.org/Manuscript>
pub const MANUSCRIPT_LABEL: &str = "Manuscript";
pub struct ManuscriptIri;
impl PartialEq<&str> for ManuscriptIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MANUSCRIPT_IRI_HTTP || *other == MANUSCRIPT_IRI_HTTPS
	}
}
impl PartialEq<ManuscriptIri> for &str {
	fn eq(&self, other: &ManuscriptIri) -> bool {
		*self == MANUSCRIPT_IRI_HTTP || *self == MANUSCRIPT_IRI_HTTPS
	}
}
pub struct ManuscriptIriOrLabel;
impl PartialEq<&str> for ManuscriptIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ManuscriptIri || *other == MANUSCRIPT_LABEL
	}
}
impl PartialEq<ManuscriptIriOrLabel> for &str {
	fn eq(&self, other: &ManuscriptIriOrLabel) -> bool {
		*self == ManuscriptIri || *self == MANUSCRIPT_LABEL
	}
}
