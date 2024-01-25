/// <https://schema.org/screenCount>
pub const SCREEN_COUNT_PROPERTY_IRI_HTTP: &str = "http://schema.org/screenCount";
/// <https://schema.org/screenCount>
pub const SCREEN_COUNT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/screenCount";
/// <https://schema.org/screenCount>
pub const SCREEN_COUNT_PROPERTY_LABEL: &str = "screenCount";
pub struct ScreenCountPropertyIri;
impl PartialEq<&str> for ScreenCountPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCREEN_COUNT_PROPERTY_IRI_HTTP || *other == SCREEN_COUNT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ScreenCountPropertyIri> for &str {
	fn eq(&self, other: &ScreenCountPropertyIri) -> bool {
		*self == SCREEN_COUNT_PROPERTY_IRI_HTTP || *self == SCREEN_COUNT_PROPERTY_IRI_HTTPS
	}
}
pub struct ScreenCountPropertyIriOrLabel;
impl PartialEq<&str> for ScreenCountPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScreenCountPropertyIri || *other == SCREEN_COUNT_PROPERTY_LABEL
	}
}
impl PartialEq<ScreenCountPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ScreenCountPropertyIriOrLabel) -> bool {
		*self == ScreenCountPropertyIri || *self == SCREEN_COUNT_PROPERTY_LABEL
	}
}
