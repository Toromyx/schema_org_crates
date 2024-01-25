/// <https://schema.org/releaseDate>
pub const RELEASE_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/releaseDate";
/// <https://schema.org/releaseDate>
pub const RELEASE_DATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/releaseDate";
/// <https://schema.org/releaseDate>
pub const RELEASE_DATE_PROPERTY_LABEL: &str = "releaseDate";
pub struct ReleaseDatePropertyIri;
impl PartialEq<&str> for ReleaseDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RELEASE_DATE_PROPERTY_IRI_HTTP || *other == RELEASE_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ReleaseDatePropertyIri> for &str {
	fn eq(&self, other: &ReleaseDatePropertyIri) -> bool {
		*self == RELEASE_DATE_PROPERTY_IRI_HTTP || *self == RELEASE_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ReleaseDatePropertyIriOrLabel;
impl PartialEq<&str> for ReleaseDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ReleaseDatePropertyIri || *other == RELEASE_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<ReleaseDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ReleaseDatePropertyIriOrLabel) -> bool {
		*self == ReleaseDatePropertyIri || *self == RELEASE_DATE_PROPERTY_LABEL
	}
}
