/// <https://schema.org/dateline>
pub const DATELINE_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateline";
/// <https://schema.org/dateline>
pub const DATELINE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateline";
/// <https://schema.org/dateline>
pub const DATELINE_PROPERTY_LABEL: &str = "dateline";
pub struct DatelinePropertyIri;
impl PartialEq<&str> for DatelinePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATELINE_PROPERTY_IRI_HTTP || *other == DATELINE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DatelinePropertyIri> for &str {
	fn eq(&self, other: &DatelinePropertyIri) -> bool {
		*self == DATELINE_PROPERTY_IRI_HTTP || *self == DATELINE_PROPERTY_IRI_HTTPS
	}
}
pub struct DatelinePropertyIriOrLabel;
impl PartialEq<&str> for DatelinePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DatelinePropertyIri || *other == DATELINE_PROPERTY_LABEL
	}
}
impl PartialEq<DatelinePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DatelinePropertyIriOrLabel) -> bool {
		*self == DatelinePropertyIri || *self == DATELINE_PROPERTY_LABEL
	}
}
