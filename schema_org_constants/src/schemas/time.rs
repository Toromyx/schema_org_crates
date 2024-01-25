/// <https://schema.org/Time>
pub const TIME_IRI_HTTP: &str = "http://schema.org/Time";
/// <https://schema.org/Time>
pub const TIME_IRI_HTTPS: &str = "https://schema.org/Time";
/// <https://schema.org/Time>
pub const TIME_LABEL: &str = "Time";
pub struct TimeIri;
impl PartialEq<&str> for TimeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TIME_IRI_HTTP || *other == TIME_IRI_HTTPS
	}
}
impl PartialEq<TimeIri> for &str {
	fn eq(&self, other: &TimeIri) -> bool {
		*self == TIME_IRI_HTTP || *self == TIME_IRI_HTTPS
	}
}
pub struct TimeIriOrLabel;
impl PartialEq<&str> for TimeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TimeIri || *other == TIME_LABEL
	}
}
impl PartialEq<TimeIriOrLabel> for &str {
	fn eq(&self, other: &TimeIriOrLabel) -> bool {
		*self == TimeIri || *self == TIME_LABEL
	}
}
