/// <https://schema.org/associatedPathophysiology>
pub const ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/associatedPathophysiology";
/// <https://schema.org/associatedPathophysiology>
pub const ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/associatedPathophysiology";
/// <https://schema.org/associatedPathophysiology>
pub const ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_LABEL: &str = "associatedPathophysiology";
pub struct AssociatedPathophysiologyPropertyIri;
impl PartialEq<&str> for AssociatedPathophysiologyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedPathophysiologyPropertyIri> for &str {
	fn eq(&self, other: &AssociatedPathophysiologyPropertyIri) -> bool {
		*self == ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_IRI_HTTP
			|| *self == ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedPathophysiologyPropertyIriOrLabel;
impl PartialEq<&str> for AssociatedPathophysiologyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedPathophysiologyPropertyIri
			|| *other == ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedPathophysiologyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedPathophysiologyPropertyIriOrLabel) -> bool {
		*self == AssociatedPathophysiologyPropertyIri
			|| *self == ASSOCIATED_PATHOPHYSIOLOGY_PROPERTY_LABEL
	}
}
