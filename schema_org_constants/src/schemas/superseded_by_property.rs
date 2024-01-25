/// <https://schema.org/supersededBy>
pub const SUPERSEDED_BY_PROPERTY_IRI_HTTP: &str = "http://schema.org/supersededBy";
/// <https://schema.org/supersededBy>
pub const SUPERSEDED_BY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/supersededBy";
/// <https://schema.org/supersededBy>
pub const SUPERSEDED_BY_PROPERTY_LABEL: &str = "supersededBy";
pub struct SupersededByPropertyIri;
impl PartialEq<&str> for SupersededByPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUPERSEDED_BY_PROPERTY_IRI_HTTP || *other == SUPERSEDED_BY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SupersededByPropertyIri> for &str {
	fn eq(&self, other: &SupersededByPropertyIri) -> bool {
		*self == SUPERSEDED_BY_PROPERTY_IRI_HTTP || *self == SUPERSEDED_BY_PROPERTY_IRI_HTTPS
	}
}
pub struct SupersededByPropertyIriOrLabel;
impl PartialEq<&str> for SupersededByPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SupersededByPropertyIri || *other == SUPERSEDED_BY_PROPERTY_LABEL
	}
}
impl PartialEq<SupersededByPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SupersededByPropertyIriOrLabel) -> bool {
		*self == SupersededByPropertyIri || *self == SUPERSEDED_BY_PROPERTY_LABEL
	}
}
