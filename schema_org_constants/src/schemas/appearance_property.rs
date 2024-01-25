/// <https://schema.org/appearance>
pub const APPEARANCE_PROPERTY_IRI_HTTP: &str = "http://schema.org/appearance";
/// <https://schema.org/appearance>
pub const APPEARANCE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/appearance";
/// <https://schema.org/appearance>
pub const APPEARANCE_PROPERTY_LABEL: &str = "appearance";
pub struct AppearancePropertyIri;
impl PartialEq<&str> for AppearancePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPEARANCE_PROPERTY_IRI_HTTP || *other == APPEARANCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<AppearancePropertyIri> for &str {
	fn eq(&self, other: &AppearancePropertyIri) -> bool {
		*self == APPEARANCE_PROPERTY_IRI_HTTP || *self == APPEARANCE_PROPERTY_IRI_HTTPS
	}
}
pub struct AppearancePropertyIriOrLabel;
impl PartialEq<&str> for AppearancePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AppearancePropertyIri || *other == APPEARANCE_PROPERTY_LABEL
	}
}
impl PartialEq<AppearancePropertyIriOrLabel> for &str {
	fn eq(&self, other: &AppearancePropertyIriOrLabel) -> bool {
		*self == AppearancePropertyIri || *self == APPEARANCE_PROPERTY_LABEL
	}
}
