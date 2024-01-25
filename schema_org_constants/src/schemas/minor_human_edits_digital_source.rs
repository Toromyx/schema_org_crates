/// <https://schema.org/MinorHumanEditsDigitalSource>
pub const MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/MinorHumanEditsDigitalSource";
/// <https://schema.org/MinorHumanEditsDigitalSource>
pub const MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/MinorHumanEditsDigitalSource";
/// <https://schema.org/MinorHumanEditsDigitalSource>
pub const MINOR_HUMAN_EDITS_DIGITAL_SOURCE_LABEL: &str = "MinorHumanEditsDigitalSource";
pub struct MinorHumanEditsDigitalSourceIri;
impl PartialEq<&str> for MinorHumanEditsDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTP
			|| *other == MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<MinorHumanEditsDigitalSourceIri> for &str {
	fn eq(&self, other: &MinorHumanEditsDigitalSourceIri) -> bool {
		*self == MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTP
			|| *self == MINOR_HUMAN_EDITS_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct MinorHumanEditsDigitalSourceIriOrLabel;
impl PartialEq<&str> for MinorHumanEditsDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MinorHumanEditsDigitalSourceIri
			|| *other == MINOR_HUMAN_EDITS_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<MinorHumanEditsDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &MinorHumanEditsDigitalSourceIriOrLabel) -> bool {
		*self == MinorHumanEditsDigitalSourceIri || *self == MINOR_HUMAN_EDITS_DIGITAL_SOURCE_LABEL
	}
}
