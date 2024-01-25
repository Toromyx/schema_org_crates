/// <https://schema.org/Appearance>
pub const APPEARANCE_IRI_HTTP: &str = "http://schema.org/Appearance";
/// <https://schema.org/Appearance>
pub const APPEARANCE_IRI_HTTPS: &str = "https://schema.org/Appearance";
/// <https://schema.org/Appearance>
pub const APPEARANCE_LABEL: &str = "Appearance";
pub struct AppearanceIri;
impl PartialEq<&str> for AppearanceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == APPEARANCE_IRI_HTTP || *other == APPEARANCE_IRI_HTTPS
	}
}
impl PartialEq<AppearanceIri> for &str {
	fn eq(&self, other: &AppearanceIri) -> bool {
		*self == APPEARANCE_IRI_HTTP || *self == APPEARANCE_IRI_HTTPS
	}
}
pub struct AppearanceIriOrLabel;
impl PartialEq<&str> for AppearanceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AppearanceIri || *other == APPEARANCE_LABEL
	}
}
impl PartialEq<AppearanceIriOrLabel> for &str {
	fn eq(&self, other: &AppearanceIriOrLabel) -> bool {
		*self == AppearanceIri || *self == APPEARANCE_LABEL
	}
}
