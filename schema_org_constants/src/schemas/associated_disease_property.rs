/// <https://schema.org/associatedDisease>
pub const ASSOCIATED_DISEASE_PROPERTY_IRI_HTTP: &str = "http://schema.org/associatedDisease";
/// <https://schema.org/associatedDisease>
pub const ASSOCIATED_DISEASE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/associatedDisease";
/// <https://schema.org/associatedDisease>
pub const ASSOCIATED_DISEASE_PROPERTY_LABEL: &str = "associatedDisease";
pub struct AssociatedDiseasePropertyIri;
impl PartialEq<&str> for AssociatedDiseasePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_DISEASE_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_DISEASE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedDiseasePropertyIri> for &str {
	fn eq(&self, other: &AssociatedDiseasePropertyIri) -> bool {
		*self == ASSOCIATED_DISEASE_PROPERTY_IRI_HTTP
			|| *self == ASSOCIATED_DISEASE_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedDiseasePropertyIriOrLabel;
impl PartialEq<&str> for AssociatedDiseasePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedDiseasePropertyIri || *other == ASSOCIATED_DISEASE_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedDiseasePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedDiseasePropertyIriOrLabel) -> bool {
		*self == AssociatedDiseasePropertyIri || *self == ASSOCIATED_DISEASE_PROPERTY_LABEL
	}
}
