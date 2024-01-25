/// <https://schema.org/EndorseAction>
pub const ENDORSE_ACTION_IRI_HTTP: &str = "http://schema.org/EndorseAction";
/// <https://schema.org/EndorseAction>
pub const ENDORSE_ACTION_IRI_HTTPS: &str = "https://schema.org/EndorseAction";
/// <https://schema.org/EndorseAction>
pub const ENDORSE_ACTION_LABEL: &str = "EndorseAction";
pub struct EndorseActionIri;
impl PartialEq<&str> for EndorseActionIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENDORSE_ACTION_IRI_HTTP || *other == ENDORSE_ACTION_IRI_HTTPS
	}
}
impl PartialEq<EndorseActionIri> for &str {
	fn eq(&self, other: &EndorseActionIri) -> bool {
		*self == ENDORSE_ACTION_IRI_HTTP || *self == ENDORSE_ACTION_IRI_HTTPS
	}
}
pub struct EndorseActionIriOrLabel;
impl PartialEq<&str> for EndorseActionIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndorseActionIri || *other == ENDORSE_ACTION_LABEL
	}
}
impl PartialEq<EndorseActionIriOrLabel> for &str {
	fn eq(&self, other: &EndorseActionIriOrLabel) -> bool {
		*self == EndorseActionIri || *self == ENDORSE_ACTION_LABEL
	}
}
