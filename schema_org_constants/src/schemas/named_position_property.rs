/// <https://schema.org/namedPosition>
#[deprecated = "This schema is superseded by <https://schema.org/roleName>."]
pub const NAMED_POSITION_PROPERTY_IRI_HTTP: &str = "http://schema.org/namedPosition";
/// <https://schema.org/namedPosition>
#[deprecated = "This schema is superseded by <https://schema.org/roleName>."]
pub const NAMED_POSITION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/namedPosition";
/// <https://schema.org/namedPosition>
#[deprecated = "This schema is superseded by <https://schema.org/roleName>."]
pub const NAMED_POSITION_PROPERTY_LABEL: &str = "namedPosition";
pub struct NamedPositionPropertyIri;
impl PartialEq<&str> for NamedPositionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NAMED_POSITION_PROPERTY_IRI_HTTP || *other == NAMED_POSITION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<NamedPositionPropertyIri> for &str {
	fn eq(&self, other: &NamedPositionPropertyIri) -> bool {
		*self == NAMED_POSITION_PROPERTY_IRI_HTTP || *self == NAMED_POSITION_PROPERTY_IRI_HTTPS
	}
}
pub struct NamedPositionPropertyIriOrLabel;
impl PartialEq<&str> for NamedPositionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NamedPositionPropertyIri || *other == NAMED_POSITION_PROPERTY_LABEL
	}
}
impl PartialEq<NamedPositionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &NamedPositionPropertyIriOrLabel) -> bool {
		*self == NamedPositionPropertyIri || *self == NAMED_POSITION_PROPERTY_LABEL
	}
}
