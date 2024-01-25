/// <https://schema.org/alumniOf>
pub const ALUMNI_OF_PROPERTY_IRI_HTTP: &str = "http://schema.org/alumniOf";
/// <https://schema.org/alumniOf>
pub const ALUMNI_OF_PROPERTY_IRI_HTTPS: &str = "https://schema.org/alumniOf";
/// <https://schema.org/alumniOf>
pub const ALUMNI_OF_PROPERTY_LABEL: &str = "alumniOf";
pub struct AlumniOfPropertyIri;
impl PartialEq<&str> for AlumniOfPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ALUMNI_OF_PROPERTY_IRI_HTTP || *other == ALUMNI_OF_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AlumniOfPropertyIri> for &str {
	fn eq(&self, other: &AlumniOfPropertyIri) -> bool {
		*self == ALUMNI_OF_PROPERTY_IRI_HTTP || *self == ALUMNI_OF_PROPERTY_IRI_HTTPS
	}
}
pub struct AlumniOfPropertyIriOrLabel;
impl PartialEq<&str> for AlumniOfPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AlumniOfPropertyIri || *other == ALUMNI_OF_PROPERTY_LABEL
	}
}
impl PartialEq<AlumniOfPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AlumniOfPropertyIriOrLabel) -> bool {
		*self == AlumniOfPropertyIri || *self == ALUMNI_OF_PROPERTY_LABEL
	}
}
