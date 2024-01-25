/// <https://schema.org/DietarySupplement>
pub const DIETARY_SUPPLEMENT_IRI_HTTP: &str = "http://schema.org/DietarySupplement";
/// <https://schema.org/DietarySupplement>
pub const DIETARY_SUPPLEMENT_IRI_HTTPS: &str = "https://schema.org/DietarySupplement";
/// <https://schema.org/DietarySupplement>
pub const DIETARY_SUPPLEMENT_LABEL: &str = "DietarySupplement";
pub struct DietarySupplementIri;
impl PartialEq<&str> for DietarySupplementIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIETARY_SUPPLEMENT_IRI_HTTP || *other == DIETARY_SUPPLEMENT_IRI_HTTPS
	}
}
impl PartialEq<DietarySupplementIri> for &str {
	fn eq(&self, other: &DietarySupplementIri) -> bool {
		*self == DIETARY_SUPPLEMENT_IRI_HTTP || *self == DIETARY_SUPPLEMENT_IRI_HTTPS
	}
}
pub struct DietarySupplementIriOrLabel;
impl PartialEq<&str> for DietarySupplementIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DietarySupplementIri || *other == DIETARY_SUPPLEMENT_LABEL
	}
}
impl PartialEq<DietarySupplementIriOrLabel> for &str {
	fn eq(&self, other: &DietarySupplementIriOrLabel) -> bool {
		*self == DietarySupplementIri || *self == DIETARY_SUPPLEMENT_LABEL
	}
}
