/// <https://schema.org/dateSent>
pub const DATE_SENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateSent";
/// <https://schema.org/dateSent>
pub const DATE_SENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateSent";
/// <https://schema.org/dateSent>
pub const DATE_SENT_PROPERTY_LABEL: &str = "dateSent";
pub struct DateSentPropertyIri;
impl PartialEq<&str> for DateSentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_SENT_PROPERTY_IRI_HTTP || *other == DATE_SENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateSentPropertyIri> for &str {
	fn eq(&self, other: &DateSentPropertyIri) -> bool {
		*self == DATE_SENT_PROPERTY_IRI_HTTP || *self == DATE_SENT_PROPERTY_IRI_HTTPS
	}
}
pub struct DateSentPropertyIriOrLabel;
impl PartialEq<&str> for DateSentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateSentPropertyIri || *other == DATE_SENT_PROPERTY_LABEL
	}
}
impl PartialEq<DateSentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateSentPropertyIriOrLabel) -> bool {
		*self == DateSentPropertyIri || *self == DATE_SENT_PROPERTY_LABEL
	}
}
