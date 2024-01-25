/// <https://schema.org/associatedAnatomy>
pub const ASSOCIATED_ANATOMY_PROPERTY_IRI_HTTP: &str = "http://schema.org/associatedAnatomy";
/// <https://schema.org/associatedAnatomy>
pub const ASSOCIATED_ANATOMY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/associatedAnatomy";
/// <https://schema.org/associatedAnatomy>
pub const ASSOCIATED_ANATOMY_PROPERTY_LABEL: &str = "associatedAnatomy";
pub struct AssociatedAnatomyPropertyIri;
impl PartialEq<&str> for AssociatedAnatomyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_ANATOMY_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_ANATOMY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedAnatomyPropertyIri> for &str {
	fn eq(&self, other: &AssociatedAnatomyPropertyIri) -> bool {
		*self == ASSOCIATED_ANATOMY_PROPERTY_IRI_HTTP
			|| *self == ASSOCIATED_ANATOMY_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedAnatomyPropertyIriOrLabel;
impl PartialEq<&str> for AssociatedAnatomyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedAnatomyPropertyIri || *other == ASSOCIATED_ANATOMY_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedAnatomyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedAnatomyPropertyIriOrLabel) -> bool {
		*self == AssociatedAnatomyPropertyIri || *self == ASSOCIATED_ANATOMY_PROPERTY_LABEL
	}
}
