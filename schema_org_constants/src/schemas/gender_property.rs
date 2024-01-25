/// <https://schema.org/gender>
pub const GENDER_PROPERTY_IRI_HTTP: &str = "http://schema.org/gender";
/// <https://schema.org/gender>
pub const GENDER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/gender";
/// <https://schema.org/gender>
pub const GENDER_PROPERTY_LABEL: &str = "gender";
pub struct GenderPropertyIri;
impl PartialEq<&str> for GenderPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == GENDER_PROPERTY_IRI_HTTP || *other == GENDER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<GenderPropertyIri> for &str {
	fn eq(&self, other: &GenderPropertyIri) -> bool {
		*self == GENDER_PROPERTY_IRI_HTTP || *self == GENDER_PROPERTY_IRI_HTTPS
	}
}
pub struct GenderPropertyIriOrLabel;
impl PartialEq<&str> for GenderPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == GenderPropertyIri || *other == GENDER_PROPERTY_LABEL
	}
}
impl PartialEq<GenderPropertyIriOrLabel> for &str {
	fn eq(&self, other: &GenderPropertyIriOrLabel) -> bool {
		*self == GenderPropertyIri || *self == GENDER_PROPERTY_LABEL
	}
}
