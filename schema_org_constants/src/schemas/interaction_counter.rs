/// <https://schema.org/InteractionCounter>
pub const INTERACTION_COUNTER_IRI_HTTP: &str = "http://schema.org/InteractionCounter";
/// <https://schema.org/InteractionCounter>
pub const INTERACTION_COUNTER_IRI_HTTPS: &str = "https://schema.org/InteractionCounter";
/// <https://schema.org/InteractionCounter>
pub const INTERACTION_COUNTER_LABEL: &str = "InteractionCounter";
pub struct InteractionCounterIri;
impl PartialEq<&str> for InteractionCounterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INTERACTION_COUNTER_IRI_HTTP || *other == INTERACTION_COUNTER_IRI_HTTPS
	}
}
impl PartialEq<InteractionCounterIri> for &str {
	fn eq(&self, other: &InteractionCounterIri) -> bool {
		*self == INTERACTION_COUNTER_IRI_HTTP || *self == INTERACTION_COUNTER_IRI_HTTPS
	}
}
pub struct InteractionCounterIriOrLabel;
impl PartialEq<&str> for InteractionCounterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InteractionCounterIri || *other == INTERACTION_COUNTER_LABEL
	}
}
impl PartialEq<InteractionCounterIriOrLabel> for &str {
	fn eq(&self, other: &InteractionCounterIriOrLabel) -> bool {
		*self == InteractionCounterIri || *self == INTERACTION_COUNTER_LABEL
	}
}
