/// <https://schema.org/interactionCount>
#[deprecated = "This schema is superseded by <https://schema.org/interactionStatistic>."]
pub const INTERACTION_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/interactionCount";
/// <https://schema.org/interactionCount>
#[deprecated = "This schema is superseded by <https://schema.org/interactionStatistic>."]
pub const INTERACTION_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/interactionCount";
/// <https://schema.org/interactionCount>
#[deprecated = "This schema is superseded by <https://schema.org/interactionStatistic>."]
pub const INTERACTION_COUNT_PROPERTY_LABEL: &str = "interactionCount";
pub struct InteractionCountPropertyIri;
impl PartialEq<&str> for InteractionCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACTION_COUNT_PROPERTY_IRI_HTTP
			|| *other == INTERACTION_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InteractionCountPropertyIri> for &str {
	fn eq(&self, other: &InteractionCountPropertyIri) -> bool {
		*self == INTERACTION_COUNT_PROPERTY_IRI_HTTP
			|| *self == INTERACTION_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct InteractionCountPropertyIriOrLabel;
impl PartialEq<&str> for InteractionCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractionCountPropertyIri || *other == INTERACTION_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<InteractionCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InteractionCountPropertyIriOrLabel) -> bool {
		*self == InteractionCountPropertyIri || *self == INTERACTION_COUNT_PROPERTY_LABEL
	}
}
