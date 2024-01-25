/// <https://schema.org/author>
pub const AUTHOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/author";
/// <https://schema.org/author>
pub const AUTHOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/author";
/// <https://schema.org/author>
pub const AUTHOR_PROPERTY_LABEL: &str = "author";
pub struct AuthorPropertyIri;
impl PartialEq<&str> for AuthorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == AUTHOR_PROPERTY_IRI_HTTP || *other == AUTHOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AuthorPropertyIri> for &str {
	fn eq(&self, other: &AuthorPropertyIri) -> bool {
		*self == AUTHOR_PROPERTY_IRI_HTTP || *self == AUTHOR_PROPERTY_IRI_HTTPS
	}
}
pub struct AuthorPropertyIriOrLabel;
impl PartialEq<&str> for AuthorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AuthorPropertyIri || *other == AUTHOR_PROPERTY_LABEL
	}
}
impl PartialEq<AuthorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AuthorPropertyIriOrLabel) -> bool {
		*self == AuthorPropertyIri || *self == AUTHOR_PROPERTY_LABEL
	}
}
