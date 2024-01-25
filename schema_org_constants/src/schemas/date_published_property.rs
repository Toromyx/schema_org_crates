/// <https://schema.org/datePublished>
pub const DATE_PUBLISHED_PROPERTY_IRI_HTTP: &str = "http://schema.org/datePublished";
/// <https://schema.org/datePublished>
pub const DATE_PUBLISHED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/datePublished";
/// <https://schema.org/datePublished>
pub const DATE_PUBLISHED_PROPERTY_LABEL: &str = "datePublished";
pub struct DatePublishedPropertyIri;
impl PartialEq<&str> for DatePublishedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_PUBLISHED_PROPERTY_IRI_HTTP || *other == DATE_PUBLISHED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DatePublishedPropertyIri> for &str {
	fn eq(&self, other: &DatePublishedPropertyIri) -> bool {
		*self == DATE_PUBLISHED_PROPERTY_IRI_HTTP || *self == DATE_PUBLISHED_PROPERTY_IRI_HTTPS
	}
}
pub struct DatePublishedPropertyIriOrLabel;
impl PartialEq<&str> for DatePublishedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DatePublishedPropertyIri || *other == DATE_PUBLISHED_PROPERTY_LABEL
	}
}
impl PartialEq<DatePublishedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DatePublishedPropertyIriOrLabel) -> bool {
		*self == DatePublishedPropertyIri || *self == DATE_PUBLISHED_PROPERTY_LABEL
	}
}
