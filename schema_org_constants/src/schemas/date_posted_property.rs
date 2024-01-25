/// <https://schema.org/datePosted>
pub const DATE_POSTED_PROPERTY_IRI_HTTP: &str = "http://schema.org/datePosted";
/// <https://schema.org/datePosted>
pub const DATE_POSTED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/datePosted";
/// <https://schema.org/datePosted>
pub const DATE_POSTED_PROPERTY_LABEL: &str = "datePosted";
pub struct DatePostedPropertyIri;
impl PartialEq<&str> for DatePostedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_POSTED_PROPERTY_IRI_HTTP || *other == DATE_POSTED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DatePostedPropertyIri> for &str {
	fn eq(&self, other: &DatePostedPropertyIri) -> bool {
		*self == DATE_POSTED_PROPERTY_IRI_HTTP || *self == DATE_POSTED_PROPERTY_IRI_HTTPS
	}
}
pub struct DatePostedPropertyIriOrLabel;
impl PartialEq<&str> for DatePostedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DatePostedPropertyIri || *other == DATE_POSTED_PROPERTY_LABEL
	}
}
impl PartialEq<DatePostedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DatePostedPropertyIriOrLabel) -> bool {
		*self == DatePostedPropertyIri || *self == DATE_POSTED_PROPERTY_LABEL
	}
}
