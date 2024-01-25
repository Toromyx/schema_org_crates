/// <https://schema.org/actors>
#[deprecated = "This schema is superseded by <https://schema.org/actor>."]
pub const ACTORS_PROPERTY_IRI_HTTP: &str = "http://schema.org/actors";
/// <https://schema.org/actors>
#[deprecated = "This schema is superseded by <https://schema.org/actor>."]
pub const ACTORS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/actors";
/// <https://schema.org/actors>
#[deprecated = "This schema is superseded by <https://schema.org/actor>."]
pub const ACTORS_PROPERTY_LABEL: &str = "actors";
pub struct ActorsPropertyIri;
impl PartialEq<&str> for ActorsPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTORS_PROPERTY_IRI_HTTP || *other == ACTORS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActorsPropertyIri> for &str {
	fn eq(&self, other: &ActorsPropertyIri) -> bool {
		*self == ACTORS_PROPERTY_IRI_HTTP || *self == ACTORS_PROPERTY_IRI_HTTPS
	}
}
pub struct ActorsPropertyIriOrLabel;
impl PartialEq<&str> for ActorsPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActorsPropertyIri || *other == ACTORS_PROPERTY_LABEL
	}
}
impl PartialEq<ActorsPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActorsPropertyIriOrLabel) -> bool {
		*self == ActorsPropertyIri || *self == ACTORS_PROPERTY_LABEL
	}
}
