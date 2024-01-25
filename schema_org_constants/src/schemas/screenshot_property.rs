/// <https://schema.org/screenshot>
pub const SCREENSHOT_PROPERTY_IRI_HTTP: &str = "http://schema.org/screenshot";
/// <https://schema.org/screenshot>
pub const SCREENSHOT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/screenshot";
/// <https://schema.org/screenshot>
pub const SCREENSHOT_PROPERTY_LABEL: &str = "screenshot";
pub struct ScreenshotPropertyIri;
impl PartialEq<&str> for ScreenshotPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCREENSHOT_PROPERTY_IRI_HTTP || *other == SCREENSHOT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ScreenshotPropertyIri> for &str {
	fn eq(&self, other: &ScreenshotPropertyIri) -> bool {
		*self == SCREENSHOT_PROPERTY_IRI_HTTP || *self == SCREENSHOT_PROPERTY_IRI_HTTPS
	}
}
pub struct ScreenshotPropertyIriOrLabel;
impl PartialEq<&str> for ScreenshotPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScreenshotPropertyIri || *other == SCREENSHOT_PROPERTY_LABEL
	}
}
impl PartialEq<ScreenshotPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ScreenshotPropertyIriOrLabel) -> bool {
		*self == ScreenshotPropertyIri || *self == SCREENSHOT_PROPERTY_LABEL
	}
}
