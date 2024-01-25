/// <https://schema.org/pregnancyCategory>
pub const PREGNANCY_CATEGORY_PROPERTY_IRI_HTTP: &str = "http://schema.org/pregnancyCategory";
/// <https://schema.org/pregnancyCategory>
pub const PREGNANCY_CATEGORY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/pregnancyCategory";
/// <https://schema.org/pregnancyCategory>
pub const PREGNANCY_CATEGORY_PROPERTY_LABEL: &str = "pregnancyCategory";
pub struct PregnancyCategoryPropertyIri;
impl PartialEq<&str> for PregnancyCategoryPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREGNANCY_CATEGORY_PROPERTY_IRI_HTTP
			|| *other == PREGNANCY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PregnancyCategoryPropertyIri> for &str {
	fn eq(&self, other: &PregnancyCategoryPropertyIri) -> bool {
		*self == PREGNANCY_CATEGORY_PROPERTY_IRI_HTTP
			|| *self == PREGNANCY_CATEGORY_PROPERTY_IRI_HTTPS
	}
}
pub struct PregnancyCategoryPropertyIriOrLabel;
impl PartialEq<&str> for PregnancyCategoryPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PregnancyCategoryPropertyIri || *other == PREGNANCY_CATEGORY_PROPERTY_LABEL
	}
}
impl PartialEq<PregnancyCategoryPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PregnancyCategoryPropertyIriOrLabel) -> bool {
		*self == PregnancyCategoryPropertyIri || *self == PREGNANCY_CATEGORY_PROPERTY_LABEL
	}
}
