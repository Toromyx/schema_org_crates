/// <https://schema.org/associatedMedia>
pub const ASSOCIATED_MEDIA_PROPERTY_IRI_HTTP: &str = "http://schema.org/associatedMedia";
/// <https://schema.org/associatedMedia>
pub const ASSOCIATED_MEDIA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/associatedMedia";
/// <https://schema.org/associatedMedia>
pub const ASSOCIATED_MEDIA_PROPERTY_LABEL: &str = "associatedMedia";
pub struct AssociatedMediaPropertyIri;
impl PartialEq<&str> for AssociatedMediaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ASSOCIATED_MEDIA_PROPERTY_IRI_HTTP
			|| *other == ASSOCIATED_MEDIA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AssociatedMediaPropertyIri> for &str {
	fn eq(&self, other: &AssociatedMediaPropertyIri) -> bool {
		*self == ASSOCIATED_MEDIA_PROPERTY_IRI_HTTP || *self == ASSOCIATED_MEDIA_PROPERTY_IRI_HTTPS
	}
}
pub struct AssociatedMediaPropertyIriOrLabel;
impl PartialEq<&str> for AssociatedMediaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AssociatedMediaPropertyIri || *other == ASSOCIATED_MEDIA_PROPERTY_LABEL
	}
}
impl PartialEq<AssociatedMediaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &AssociatedMediaPropertyIriOrLabel) -> bool {
		*self == AssociatedMediaPropertyIri || *self == ASSOCIATED_MEDIA_PROPERTY_LABEL
	}
}
