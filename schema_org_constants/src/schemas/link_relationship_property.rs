/// <https://schema.org/linkRelationship>
pub const LINK_RELATIONSHIP_PROPERTY_IRI_HTTP: &str = "http://schema.org/linkRelationship";
/// <https://schema.org/linkRelationship>
pub const LINK_RELATIONSHIP_PROPERTY_IRI_HTTPS: &str = "https://schema.org/linkRelationship";
/// <https://schema.org/linkRelationship>
pub const LINK_RELATIONSHIP_PROPERTY_LABEL: &str = "linkRelationship";
pub struct LinkRelationshipPropertyIri;
impl PartialEq<&str> for LinkRelationshipPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LINK_RELATIONSHIP_PROPERTY_IRI_HTTP
			|| *other == LINK_RELATIONSHIP_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LinkRelationshipPropertyIri> for &str {
	fn eq(&self, other: &LinkRelationshipPropertyIri) -> bool {
		*self == LINK_RELATIONSHIP_PROPERTY_IRI_HTTP
			|| *self == LINK_RELATIONSHIP_PROPERTY_IRI_HTTPS
	}
}
pub struct LinkRelationshipPropertyIriOrLabel;
impl PartialEq<&str> for LinkRelationshipPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LinkRelationshipPropertyIri || *other == LINK_RELATIONSHIP_PROPERTY_LABEL
	}
}
impl PartialEq<LinkRelationshipPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LinkRelationshipPropertyIriOrLabel) -> bool {
		*self == LinkRelationshipPropertyIri || *self == LINK_RELATIONSHIP_PROPERTY_LABEL
	}
}
