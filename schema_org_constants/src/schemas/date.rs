/// <https://schema.org/Date>
pub const DATE_IRI_HTTP: &str = "http://schema.org/Date";
/// <https://schema.org/Date>
pub const DATE_IRI_HTTPS: &str = "https://schema.org/Date";
/// <https://schema.org/Date>
pub const DATE_LABEL: &str = "Date";
pub struct DateIri;
impl PartialEq<&str> for DateIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_IRI_HTTP || *other == DATE_IRI_HTTPS
	}
}
impl PartialEq<DateIri> for &str {
	fn eq(&self, other: &DateIri) -> bool {
		*self == DATE_IRI_HTTP || *self == DATE_IRI_HTTPS
	}
}
pub struct DateIriOrLabel;
impl PartialEq<&str> for DateIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateIri || *other == DATE_LABEL
	}
}
impl PartialEq<DateIriOrLabel> for &str {
	fn eq(&self, other: &DateIriOrLabel) -> bool {
		*self == DateIri || *self == DATE_LABEL
	}
}
