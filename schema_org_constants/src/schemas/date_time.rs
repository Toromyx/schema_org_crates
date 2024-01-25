/// <https://schema.org/DateTime>
pub const DATE_TIME_IRI_HTTP: &str = "http://schema.org/DateTime";
/// <https://schema.org/DateTime>
pub const DATE_TIME_IRI_HTTPS: &str = "https://schema.org/DateTime";
/// <https://schema.org/DateTime>
pub const DATE_TIME_LABEL: &str = "DateTime";
pub struct DateTimeIri;
impl PartialEq<&str> for DateTimeIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_TIME_IRI_HTTP || *other == DATE_TIME_IRI_HTTPS
	}
}
impl PartialEq<DateTimeIri> for &str {
	fn eq(&self, other: &DateTimeIri) -> bool {
		*self == DATE_TIME_IRI_HTTP || *self == DATE_TIME_IRI_HTTPS
	}
}
pub struct DateTimeIriOrLabel;
impl PartialEq<&str> for DateTimeIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateTimeIri || *other == DATE_TIME_LABEL
	}
}
impl PartialEq<DateTimeIriOrLabel> for &str {
	fn eq(&self, other: &DateTimeIriOrLabel) -> bool {
		*self == DateTimeIri || *self == DATE_TIME_LABEL
	}
}
