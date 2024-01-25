/// <https://schema.org/duringMedia>
pub const DURING_MEDIA_PROPERTY_IRI_HTTP: &str = "http://schema.org/duringMedia";
/// <https://schema.org/duringMedia>
pub const DURING_MEDIA_PROPERTY_IRI_HTTPS: &str = "https://schema.org/duringMedia";
/// <https://schema.org/duringMedia>
pub const DURING_MEDIA_PROPERTY_LABEL: &str = "duringMedia";
pub struct DuringMediaPropertyIri;
impl PartialEq<&str> for DuringMediaPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DURING_MEDIA_PROPERTY_IRI_HTTP || *other == DURING_MEDIA_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DuringMediaPropertyIri> for &str {
	fn eq(&self, other: &DuringMediaPropertyIri) -> bool {
		*self == DURING_MEDIA_PROPERTY_IRI_HTTP || *self == DURING_MEDIA_PROPERTY_IRI_HTTPS
	}
}
pub struct DuringMediaPropertyIriOrLabel;
impl PartialEq<&str> for DuringMediaPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DuringMediaPropertyIri || *other == DURING_MEDIA_PROPERTY_LABEL
	}
}
impl PartialEq<DuringMediaPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DuringMediaPropertyIriOrLabel) -> bool {
		*self == DuringMediaPropertyIri || *self == DURING_MEDIA_PROPERTY_LABEL
	}
}
