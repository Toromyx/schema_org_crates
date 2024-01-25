/// <https://schema.org/dateReceived>
pub const DATE_RECEIVED_PROPERTY_IRI_HTTP: &str = "http://schema.org/dateReceived";
/// <https://schema.org/dateReceived>
pub const DATE_RECEIVED_PROPERTY_IRI_HTTPS: &str = "https://schema.org/dateReceived";
/// <https://schema.org/dateReceived>
pub const DATE_RECEIVED_PROPERTY_LABEL: &str = "dateReceived";
pub struct DateReceivedPropertyIri;
impl PartialEq<&str> for DateReceivedPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DATE_RECEIVED_PROPERTY_IRI_HTTP || *other == DATE_RECEIVED_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DateReceivedPropertyIri> for &str {
	fn eq(&self, other: &DateReceivedPropertyIri) -> bool {
		*self == DATE_RECEIVED_PROPERTY_IRI_HTTP || *self == DATE_RECEIVED_PROPERTY_IRI_HTTPS
	}
}
pub struct DateReceivedPropertyIriOrLabel;
impl PartialEq<&str> for DateReceivedPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DateReceivedPropertyIri || *other == DATE_RECEIVED_PROPERTY_LABEL
	}
}
impl PartialEq<DateReceivedPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DateReceivedPropertyIriOrLabel) -> bool {
		*self == DateReceivedPropertyIri || *self == DATE_RECEIVED_PROPERTY_LABEL
	}
}
