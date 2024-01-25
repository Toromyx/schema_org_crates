/// <https://schema.org/Sunday>
pub const SUNDAY_IRI_HTTP: &str = "http://schema.org/Sunday";
/// <https://schema.org/Sunday>
pub const SUNDAY_IRI_HTTPS: &str = "https://schema.org/Sunday";
/// <https://schema.org/Sunday>
pub const SUNDAY_LABEL: &str = "Sunday";
pub struct SundayIri;
impl PartialEq<&str> for SundayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SUNDAY_IRI_HTTP || *other == SUNDAY_IRI_HTTPS
	}
}
impl PartialEq<SundayIri> for &str {
	fn eq(&self, other: &SundayIri) -> bool {
		*self == SUNDAY_IRI_HTTP || *self == SUNDAY_IRI_HTTPS
	}
}
pub struct SundayIriOrLabel;
impl PartialEq<&str> for SundayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SundayIri || *other == SUNDAY_LABEL
	}
}
impl PartialEq<SundayIriOrLabel> for &str {
	fn eq(&self, other: &SundayIriOrLabel) -> bool {
		*self == SundayIri || *self == SUNDAY_LABEL
	}
}
