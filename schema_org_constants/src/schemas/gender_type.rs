/// <https://schema.org/GenderType>
pub const GENDER_TYPE_IRI_HTTP: &str = "http://schema.org/GenderType";
/// <https://schema.org/GenderType>
pub const GENDER_TYPE_IRI_HTTPS: &str = "https://schema.org/GenderType";
/// <https://schema.org/GenderType>
pub const GENDER_TYPE_LABEL: &str = "GenderType";
pub struct GenderTypeIri;
impl PartialEq<&str> for GenderTypeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENDER_TYPE_IRI_HTTP || *other == GENDER_TYPE_IRI_HTTPS
	}
}
impl PartialEq<GenderTypeIri> for &str {
	fn eq(&self, other: &GenderTypeIri) -> bool {
		*self == GENDER_TYPE_IRI_HTTP || *self == GENDER_TYPE_IRI_HTTPS
	}
}
pub struct GenderTypeIriOrLabel;
impl PartialEq<&str> for GenderTypeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GenderTypeIri || *other == GENDER_TYPE_LABEL
	}
}
impl PartialEq<GenderTypeIriOrLabel> for &str {
	fn eq(&self, other: &GenderTypeIriOrLabel) -> bool {
		*self == GenderTypeIri || *self == GENDER_TYPE_LABEL
	}
}
