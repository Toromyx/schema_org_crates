/// <https://schema.org/qualifications>
pub const QUALIFICATIONS_PROPERTY_IRI_HTTP: &str = "http://schema.org/qualifications";
/// <https://schema.org/qualifications>
pub const QUALIFICATIONS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/qualifications";
/// <https://schema.org/qualifications>
pub const QUALIFICATIONS_PROPERTY_LABEL: &str = "qualifications";
pub struct QualificationsPropertyIri;
impl PartialEq<&str> for QualificationsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == QUALIFICATIONS_PROPERTY_IRI_HTTP || *other == QUALIFICATIONS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<QualificationsPropertyIri> for &str {
	fn eq(&self, other: &QualificationsPropertyIri) -> bool {
		*self == QUALIFICATIONS_PROPERTY_IRI_HTTP || *self == QUALIFICATIONS_PROPERTY_IRI_HTTPS
	}
}
pub struct QualificationsPropertyIriOrLabel;
impl PartialEq<&str> for QualificationsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == QualificationsPropertyIri || *other == QUALIFICATIONS_PROPERTY_LABEL
	}
}
impl PartialEq<QualificationsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &QualificationsPropertyIriOrLabel) -> bool {
		*self == QualificationsPropertyIri || *self == QUALIFICATIONS_PROPERTY_LABEL
	}
}
