/// <https://schema.org/director>
pub const DIRECTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/director";
/// <https://schema.org/director>
pub const DIRECTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/director";
/// <https://schema.org/director>
pub const DIRECTOR_PROPERTY_LABEL: &str = "director";
pub struct DirectorPropertyIri;
impl PartialEq<&str> for DirectorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIRECTOR_PROPERTY_IRI_HTTP || *other == DIRECTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DirectorPropertyIri> for &str {
	fn eq(&self, other: &DirectorPropertyIri) -> bool {
		*self == DIRECTOR_PROPERTY_IRI_HTTP || *self == DIRECTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct DirectorPropertyIriOrLabel;
impl PartialEq<&str> for DirectorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DirectorPropertyIri || *other == DIRECTOR_PROPERTY_LABEL
	}
}
impl PartialEq<DirectorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DirectorPropertyIriOrLabel) -> bool {
		*self == DirectorPropertyIri || *self == DIRECTOR_PROPERTY_LABEL
	}
}
