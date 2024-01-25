/// <https://schema.org/syllabusSections>
pub const SYLLABUS_SECTIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/syllabusSections";
/// <https://schema.org/syllabusSections>
pub const SYLLABUS_SECTIONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/syllabusSections";
/// <https://schema.org/syllabusSections>
pub const SYLLABUS_SECTIONS_PROPERTY_LABEL: &str = "syllabusSections";
pub struct SyllabusSectionsPropertyIri;
impl PartialEq<&str> for SyllabusSectionsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SYLLABUS_SECTIONS_PROPERTY_IRI_HTTP
			|| *other == SYLLABUS_SECTIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SyllabusSectionsPropertyIri> for &str {
	fn eq(&self, other: &SyllabusSectionsPropertyIri) -> bool {
		*self == SYLLABUS_SECTIONS_PROPERTY_IRI_HTTP
			|| *self == SYLLABUS_SECTIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct SyllabusSectionsPropertyIriOrLabel;
impl PartialEq<&str> for SyllabusSectionsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SyllabusSectionsPropertyIri || *other == SYLLABUS_SECTIONS_PROPERTY_LABEL
	}
}
impl PartialEq<SyllabusSectionsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SyllabusSectionsPropertyIriOrLabel) -> bool {
		*self == SyllabusSectionsPropertyIri || *self == SYLLABUS_SECTIONS_PROPERTY_LABEL
	}
}
