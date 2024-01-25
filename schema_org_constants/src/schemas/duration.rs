/// <https://schema.org/Duration>
pub const DURATION_IRI_HTTP: &str = "http://schema.org/Duration";
/// <https://schema.org/Duration>
pub const DURATION_IRI_HTTPS: &str = "https://schema.org/Duration";
/// <https://schema.org/Duration>
pub const DURATION_LABEL: &str = "Duration";
pub struct DurationIri;
impl PartialEq<&str> for DurationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DURATION_IRI_HTTP || *other == DURATION_IRI_HTTPS
	}
}
impl PartialEq<DurationIri> for &str {
	fn eq(&self, other: &DurationIri) -> bool {
		*self == DURATION_IRI_HTTP || *self == DURATION_IRI_HTTPS
	}
}
pub struct DurationIriOrLabel;
impl PartialEq<&str> for DurationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DurationIri || *other == DURATION_LABEL
	}
}
impl PartialEq<DurationIriOrLabel> for &str {
	fn eq(&self, other: &DurationIriOrLabel) -> bool {
		*self == DurationIri || *self == DURATION_LABEL
	}
}
