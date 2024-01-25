/// <https://schema.org/interactionService>
pub const INTERACTION_SERVICE_PROPERTY_IRI_HTTP: &str = "http://schema.org/interactionService";
/// <https://schema.org/interactionService>
pub const INTERACTION_SERVICE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/interactionService";
/// <https://schema.org/interactionService>
pub const INTERACTION_SERVICE_PROPERTY_LABEL: &str = "interactionService";
pub struct InteractionServicePropertyIri;
impl PartialEq<&str> for InteractionServicePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACTION_SERVICE_PROPERTY_IRI_HTTP
			|| *other == INTERACTION_SERVICE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InteractionServicePropertyIri> for &str {
	fn eq(&self, other: &InteractionServicePropertyIri) -> bool {
		*self == INTERACTION_SERVICE_PROPERTY_IRI_HTTP
			|| *self == INTERACTION_SERVICE_PROPERTY_IRI_HTTPS
	}
}
pub struct InteractionServicePropertyIriOrLabel;
impl PartialEq<&str> for InteractionServicePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractionServicePropertyIri || *other == INTERACTION_SERVICE_PROPERTY_LABEL
	}
}
impl PartialEq<InteractionServicePropertyIriOrLabel> for &str {
	fn eq(&self, other: &InteractionServicePropertyIriOrLabel) -> bool {
		*self == InteractionServicePropertyIri || *self == INTERACTION_SERVICE_PROPERTY_LABEL
	}
}
