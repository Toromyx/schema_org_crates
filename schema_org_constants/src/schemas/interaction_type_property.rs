/// <https://schema.org/interactionType>
pub const INTERACTION_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/interactionType";
/// <https://schema.org/interactionType>
pub const INTERACTION_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/interactionType";
/// <https://schema.org/interactionType>
pub const INTERACTION_TYPE_PROPERTY_LABEL: &str = "interactionType";
pub struct InteractionTypePropertyIri;
impl PartialEq<&str> for InteractionTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACTION_TYPE_PROPERTY_IRI_HTTP
			|| *other == INTERACTION_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InteractionTypePropertyIri> for &str {
	fn eq(&self, other: &InteractionTypePropertyIri) -> bool {
		*self == INTERACTION_TYPE_PROPERTY_IRI_HTTP || *self == INTERACTION_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct InteractionTypePropertyIriOrLabel;
impl PartialEq<&str> for InteractionTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractionTypePropertyIri || *other == INTERACTION_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<InteractionTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &InteractionTypePropertyIriOrLabel) -> bool {
		*self == InteractionTypePropertyIri || *self == INTERACTION_TYPE_PROPERTY_LABEL
	}
}
