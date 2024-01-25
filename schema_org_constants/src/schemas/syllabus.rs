/// <https://schema.org/Syllabus>
pub const SYLLABUS_IRI_HTTP: &str = "http://schema.org/Syllabus";
/// <https://schema.org/Syllabus>
pub const SYLLABUS_IRI_HTTPS: &str = "https://schema.org/Syllabus";
/// <https://schema.org/Syllabus>
pub const SYLLABUS_LABEL: &str = "Syllabus";
pub struct SyllabusIri;
impl PartialEq<&str> for SyllabusIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SYLLABUS_IRI_HTTP || *other == SYLLABUS_IRI_HTTPS
	}
}
impl PartialEq<SyllabusIri> for &str {
	fn eq(&self, other: &SyllabusIri) -> bool {
		*self == SYLLABUS_IRI_HTTP || *self == SYLLABUS_IRI_HTTPS
	}
}
pub struct SyllabusIriOrLabel;
impl PartialEq<&str> for SyllabusIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SyllabusIri || *other == SYLLABUS_LABEL
	}
}
impl PartialEq<SyllabusIriOrLabel> for &str {
	fn eq(&self, other: &SyllabusIriOrLabel) -> bool {
		*self == SyllabusIri || *self == SYLLABUS_LABEL
	}
}
