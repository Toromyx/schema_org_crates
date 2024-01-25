/// <https://schema.org/SexualContentConsideration>
pub const SEXUAL_CONTENT_CONSIDERATION_IRI_HTTP: &str =
	"http://schema.org/SexualContentConsideration";
/// <https://schema.org/SexualContentConsideration>
pub const SEXUAL_CONTENT_CONSIDERATION_IRI_HTTPS: &str =
	"https://schema.org/SexualContentConsideration";
/// <https://schema.org/SexualContentConsideration>
pub const SEXUAL_CONTENT_CONSIDERATION_LABEL: &str = "SexualContentConsideration";
pub struct SexualContentConsiderationIri;
impl PartialEq<&str> for SexualContentConsiderationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SEXUAL_CONTENT_CONSIDERATION_IRI_HTTP
			|| *other == SEXUAL_CONTENT_CONSIDERATION_IRI_HTTPS
	}
}
impl PartialEq<SexualContentConsiderationIri> for &str {
	fn eq(&self, other: &SexualContentConsiderationIri) -> bool {
		*self == SEXUAL_CONTENT_CONSIDERATION_IRI_HTTP
			|| *self == SEXUAL_CONTENT_CONSIDERATION_IRI_HTTPS
	}
}
pub struct SexualContentConsiderationIriOrLabel;
impl PartialEq<&str> for SexualContentConsiderationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SexualContentConsiderationIri || *other == SEXUAL_CONTENT_CONSIDERATION_LABEL
	}
}
impl PartialEq<SexualContentConsiderationIriOrLabel> for &str {
	fn eq(&self, other: &SexualContentConsiderationIriOrLabel) -> bool {
		*self == SexualContentConsiderationIri || *self == SEXUAL_CONTENT_CONSIDERATION_LABEL
	}
}
