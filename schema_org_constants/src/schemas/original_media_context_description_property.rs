/// <https://schema.org/originalMediaContextDescription>
pub const ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/originalMediaContextDescription";
/// <https://schema.org/originalMediaContextDescription>
pub const ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/originalMediaContextDescription";
/// <https://schema.org/originalMediaContextDescription>
pub const ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_LABEL: &str =
	"originalMediaContextDescription";
pub struct OriginalMediaContextDescriptionPropertyIri;
impl PartialEq<&str> for OriginalMediaContextDescriptionPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *other == ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OriginalMediaContextDescriptionPropertyIri> for &str {
	fn eq(&self, other: &OriginalMediaContextDescriptionPropertyIri) -> bool {
		*self == ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTP
			|| *self == ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_IRI_HTTPS
	}
}
pub struct OriginalMediaContextDescriptionPropertyIriOrLabel;
impl PartialEq<&str> for OriginalMediaContextDescriptionPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OriginalMediaContextDescriptionPropertyIri
			|| *other == ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_LABEL
	}
}
impl PartialEq<OriginalMediaContextDescriptionPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OriginalMediaContextDescriptionPropertyIriOrLabel) -> bool {
		*self == OriginalMediaContextDescriptionPropertyIri
			|| *self == ORIGINAL_MEDIA_CONTEXT_DESCRIPTION_PROPERTY_LABEL
	}
}
