/// <https://schema.org/preparation>
pub const PREPARATION_PROPERTY_IRI_HTTP: &str = "http://schema.org/preparation";
/// <https://schema.org/preparation>
pub const PREPARATION_PROPERTY_IRI_HTTPS: &str = "https://schema.org/preparation";
/// <https://schema.org/preparation>
pub const PREPARATION_PROPERTY_LABEL: &str = "preparation";
pub struct PreparationPropertyIri;
impl PartialEq<&str> for PreparationPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PREPARATION_PROPERTY_IRI_HTTP || *other == PREPARATION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PreparationPropertyIri> for &str {
	fn eq(&self, other: &PreparationPropertyIri) -> bool {
		*self == PREPARATION_PROPERTY_IRI_HTTP || *self == PREPARATION_PROPERTY_IRI_HTTPS
	}
}
pub struct PreparationPropertyIriOrLabel;
impl PartialEq<&str> for PreparationPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PreparationPropertyIri || *other == PREPARATION_PROPERTY_LABEL
	}
}
impl PartialEq<PreparationPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PreparationPropertyIriOrLabel) -> bool {
		*self == PreparationPropertyIri || *self == PREPARATION_PROPERTY_LABEL
	}
}
