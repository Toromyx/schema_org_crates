/// <https://schema.org/performerIn>
pub const PERFORMER_IN_PROPERTY_IRI_HTTP: &str = "http://schema.org/performerIn";
/// <https://schema.org/performerIn>
pub const PERFORMER_IN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/performerIn";
/// <https://schema.org/performerIn>
pub const PERFORMER_IN_PROPERTY_LABEL: &str = "performerIn";
pub struct PerformerInPropertyIri;
impl PartialEq<&str> for PerformerInPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORMER_IN_PROPERTY_IRI_HTTP || *other == PERFORMER_IN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PerformerInPropertyIri> for &str {
	fn eq(&self, other: &PerformerInPropertyIri) -> bool {
		*self == PERFORMER_IN_PROPERTY_IRI_HTTP || *self == PERFORMER_IN_PROPERTY_IRI_HTTPS
	}
}
pub struct PerformerInPropertyIriOrLabel;
impl PartialEq<&str> for PerformerInPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformerInPropertyIri || *other == PERFORMER_IN_PROPERTY_LABEL
	}
}
impl PartialEq<PerformerInPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PerformerInPropertyIriOrLabel) -> bool {
		*self == PerformerInPropertyIri || *self == PERFORMER_IN_PROPERTY_LABEL
	}
}
