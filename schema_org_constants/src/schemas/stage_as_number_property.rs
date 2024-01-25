/// <https://schema.org/stageAsNumber>
pub const STAGE_AS_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/stageAsNumber";
/// <https://schema.org/stageAsNumber>
pub const STAGE_AS_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/stageAsNumber";
/// <https://schema.org/stageAsNumber>
pub const STAGE_AS_NUMBER_PROPERTY_LABEL: &str = "stageAsNumber";
pub struct StageAsNumberPropertyIri;
impl PartialEq<&str> for StageAsNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == STAGE_AS_NUMBER_PROPERTY_IRI_HTTP || *other == STAGE_AS_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<StageAsNumberPropertyIri> for &str {
	fn eq(&self, other: &StageAsNumberPropertyIri) -> bool {
		*self == STAGE_AS_NUMBER_PROPERTY_IRI_HTTP || *self == STAGE_AS_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct StageAsNumberPropertyIriOrLabel;
impl PartialEq<&str> for StageAsNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == StageAsNumberPropertyIri || *other == STAGE_AS_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<StageAsNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &StageAsNumberPropertyIriOrLabel) -> bool {
		*self == StageAsNumberPropertyIri || *self == STAGE_AS_NUMBER_PROPERTY_LABEL
	}
}
