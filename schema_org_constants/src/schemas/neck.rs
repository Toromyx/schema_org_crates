/// <https://schema.org/Neck>
pub const NECK_IRI_HTTP: &str = "http://schema.org/Neck";
/// <https://schema.org/Neck>
pub const NECK_IRI_HTTPS: &str = "https://schema.org/Neck";
/// <https://schema.org/Neck>
pub const NECK_LABEL: &str = "Neck";
pub struct NeckIri;
impl PartialEq<&str> for NeckIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NECK_IRI_HTTP || *other == NECK_IRI_HTTPS
	}
}
impl PartialEq<NeckIri> for &str {
	fn eq(&self, other: &NeckIri) -> bool {
		*self == NECK_IRI_HTTP || *self == NECK_IRI_HTTPS
	}
}
pub struct NeckIriOrLabel;
impl PartialEq<&str> for NeckIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NeckIri || *other == NECK_LABEL
	}
}
impl PartialEq<NeckIriOrLabel> for &str {
	fn eq(&self, other: &NeckIriOrLabel) -> bool {
		*self == NeckIri || *self == NECK_LABEL
	}
}
