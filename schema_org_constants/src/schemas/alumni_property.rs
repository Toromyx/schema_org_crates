/// <https://schema.org/alumni>
pub const ALUMNI_PROPERTY_IRI_HTTP: &str = "http://schema.org/alumni";
/// <https://schema.org/alumni>
pub const ALUMNI_PROPERTY_IRI_HTTPS: &str = "https://schema.org/alumni";
/// <https://schema.org/alumni>
pub const ALUMNI_PROPERTY_LABEL: &str = "alumni";
pub struct AlumniPropertyIri;
impl PartialEq<&str> for AlumniPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALUMNI_PROPERTY_IRI_HTTP || *other == ALUMNI_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlumniPropertyIri> for &str {
	fn eq(&self, other: &AlumniPropertyIri) -> bool {
		*self == ALUMNI_PROPERTY_IRI_HTTP || *self == ALUMNI_PROPERTY_IRI_HTTPS
	}
}
pub struct AlumniPropertyIriOrLabel;
impl PartialEq<&str> for AlumniPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlumniPropertyIri || *other == ALUMNI_PROPERTY_LABEL
	}
}
impl PartialEq<AlumniPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlumniPropertyIriOrLabel) -> bool {
		*self == AlumniPropertyIri || *self == ALUMNI_PROPERTY_LABEL
	}
}
