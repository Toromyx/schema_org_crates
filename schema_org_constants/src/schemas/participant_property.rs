/// <https://schema.org/participant>
pub const PARTICIPANT_PROPERTY_IRI_HTTP: &str = "http://schema.org/participant";
/// <https://schema.org/participant>
pub const PARTICIPANT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/participant";
/// <https://schema.org/participant>
pub const PARTICIPANT_PROPERTY_LABEL: &str = "participant";
pub struct ParticipantPropertyIri;
impl PartialEq<&str> for ParticipantPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PARTICIPANT_PROPERTY_IRI_HTTP || *other == PARTICIPANT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ParticipantPropertyIri> for &str {
	fn eq(&self, other: &ParticipantPropertyIri) -> bool {
		*self == PARTICIPANT_PROPERTY_IRI_HTTP || *self == PARTICIPANT_PROPERTY_IRI_HTTPS
	}
}
pub struct ParticipantPropertyIriOrLabel;
impl PartialEq<&str> for ParticipantPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ParticipantPropertyIri || *other == PARTICIPANT_PROPERTY_LABEL
	}
}
impl PartialEq<ParticipantPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ParticipantPropertyIriOrLabel) -> bool {
		*self == ParticipantPropertyIri || *self == PARTICIPANT_PROPERTY_LABEL
	}
}
