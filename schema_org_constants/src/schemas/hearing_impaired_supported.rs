/// <https://schema.org/HearingImpairedSupported>
pub const HEARING_IMPAIRED_SUPPORTED_IRI_HTTP: &str = "http://schema.org/HearingImpairedSupported";
/// <https://schema.org/HearingImpairedSupported>
pub const HEARING_IMPAIRED_SUPPORTED_IRI_HTTPS: &str =
	"https://schema.org/HearingImpairedSupported";
/// <https://schema.org/HearingImpairedSupported>
pub const HEARING_IMPAIRED_SUPPORTED_LABEL: &str = "HearingImpairedSupported";
pub struct HearingImpairedSupportedIri;
impl PartialEq<&str> for HearingImpairedSupportedIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HEARING_IMPAIRED_SUPPORTED_IRI_HTTP
			|| *other == HEARING_IMPAIRED_SUPPORTED_IRI_HTTPS
	}
}
impl PartialEq<HearingImpairedSupportedIri> for &str {
	fn eq(&self, other: &HearingImpairedSupportedIri) -> bool {
		*self == HEARING_IMPAIRED_SUPPORTED_IRI_HTTP
			|| *self == HEARING_IMPAIRED_SUPPORTED_IRI_HTTPS
	}
}
pub struct HearingImpairedSupportedIriOrLabel;
impl PartialEq<&str> for HearingImpairedSupportedIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HearingImpairedSupportedIri || *other == HEARING_IMPAIRED_SUPPORTED_LABEL
	}
}
impl PartialEq<HearingImpairedSupportedIriOrLabel> for &str {
	fn eq(&self, other: &HearingImpairedSupportedIriOrLabel) -> bool {
		*self == HearingImpairedSupportedIri || *self == HEARING_IMPAIRED_SUPPORTED_LABEL
	}
}
