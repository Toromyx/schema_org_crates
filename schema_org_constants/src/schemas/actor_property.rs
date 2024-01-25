/// <https://schema.org/actor>
pub const ACTOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/actor";
/// <https://schema.org/actor>
pub const ACTOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/actor";
/// <https://schema.org/actor>
pub const ACTOR_PROPERTY_LABEL: &str = "actor";
pub struct ActorPropertyIri;
impl PartialEq<&str> for ActorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ACTOR_PROPERTY_IRI_HTTP || *other == ACTOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ActorPropertyIri> for &str {
	fn eq(&self, other: &ActorPropertyIri) -> bool {
		*self == ACTOR_PROPERTY_IRI_HTTP || *self == ACTOR_PROPERTY_IRI_HTTPS
	}
}
pub struct ActorPropertyIriOrLabel;
impl PartialEq<&str> for ActorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ActorPropertyIri || *other == ACTOR_PROPERTY_LABEL
	}
}
impl PartialEq<ActorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ActorPropertyIriOrLabel) -> bool {
		*self == ActorPropertyIri || *self == ACTOR_PROPERTY_LABEL
	}
}
