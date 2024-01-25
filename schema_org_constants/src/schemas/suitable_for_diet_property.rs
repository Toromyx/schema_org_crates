/// <https://schema.org/suitableForDiet>
pub const SUITABLE_FOR_DIET_PROPERTY_IRI_HTTP: &str = "http://schema.org/suitableForDiet";
/// <https://schema.org/suitableForDiet>
pub const SUITABLE_FOR_DIET_PROPERTY_IRI_HTTPS: &str = "https://schema.org/suitableForDiet";
/// <https://schema.org/suitableForDiet>
pub const SUITABLE_FOR_DIET_PROPERTY_LABEL: &str = "suitableForDiet";
pub struct SuitableForDietPropertyIri;
impl PartialEq<&str> for SuitableForDietPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUITABLE_FOR_DIET_PROPERTY_IRI_HTTP
			|| *other == SUITABLE_FOR_DIET_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SuitableForDietPropertyIri> for &str {
	fn eq(&self, other: &SuitableForDietPropertyIri) -> bool {
		*self == SUITABLE_FOR_DIET_PROPERTY_IRI_HTTP
			|| *self == SUITABLE_FOR_DIET_PROPERTY_IRI_HTTPS
	}
}
pub struct SuitableForDietPropertyIriOrLabel;
impl PartialEq<&str> for SuitableForDietPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SuitableForDietPropertyIri || *other == SUITABLE_FOR_DIET_PROPERTY_LABEL
	}
}
impl PartialEq<SuitableForDietPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SuitableForDietPropertyIriOrLabel) -> bool {
		*self == SuitableForDietPropertyIri || *self == SUITABLE_FOR_DIET_PROPERTY_LABEL
	}
}
