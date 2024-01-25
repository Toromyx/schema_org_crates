/// <https://schema.org/Otolaryngologic>
pub const OTOLARYNGOLOGIC_IRI_HTTP: &str = "http://schema.org/Otolaryngologic";
/// <https://schema.org/Otolaryngologic>
pub const OTOLARYNGOLOGIC_IRI_HTTPS: &str = "https://schema.org/Otolaryngologic";
/// <https://schema.org/Otolaryngologic>
pub const OTOLARYNGOLOGIC_LABEL: &str = "Otolaryngologic";
pub struct OtolaryngologicIri;
impl PartialEq<&str> for OtolaryngologicIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OTOLARYNGOLOGIC_IRI_HTTP || *other == OTOLARYNGOLOGIC_IRI_HTTPS
	}
}
impl PartialEq<OtolaryngologicIri> for &str {
	fn eq(&self, other: &OtolaryngologicIri) -> bool {
		*self == OTOLARYNGOLOGIC_IRI_HTTP || *self == OTOLARYNGOLOGIC_IRI_HTTPS
	}
}
pub struct OtolaryngologicIriOrLabel;
impl PartialEq<&str> for OtolaryngologicIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OtolaryngologicIri || *other == OTOLARYNGOLOGIC_LABEL
	}
}
impl PartialEq<OtolaryngologicIriOrLabel> for &str {
	fn eq(&self, other: &OtolaryngologicIriOrLabel) -> bool {
		*self == OtolaryngologicIri || *self == OTOLARYNGOLOGIC_LABEL
	}
}
