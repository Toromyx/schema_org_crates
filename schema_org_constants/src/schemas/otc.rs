/// <https://schema.org/OTC>
pub const OTC_IRI_HTTP: &str = "http://schema.org/OTC";
/// <https://schema.org/OTC>
pub const OTC_IRI_HTTPS: &str = "https://schema.org/OTC";
/// <https://schema.org/OTC>
pub const OTC_LABEL: &str = "OTC";
pub struct OtcIri;
impl PartialEq<&str> for OtcIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OTC_IRI_HTTP || *other == OTC_IRI_HTTPS
	}
}
impl PartialEq<OtcIri> for &str {
	fn eq(&self, other: &OtcIri) -> bool {
		*self == OTC_IRI_HTTP || *self == OTC_IRI_HTTPS
	}
}
pub struct OtcIriOrLabel;
impl PartialEq<&str> for OtcIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OtcIri || *other == OTC_LABEL
	}
}
impl PartialEq<OtcIriOrLabel> for &str {
	fn eq(&self, other: &OtcIriOrLabel) -> bool {
		*self == OtcIri || *self == OTC_LABEL
	}
}
