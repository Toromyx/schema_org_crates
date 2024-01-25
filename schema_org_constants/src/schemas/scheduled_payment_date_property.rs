/// <https://schema.org/scheduledPaymentDate>
pub const SCHEDULED_PAYMENT_DATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/scheduledPaymentDate";
/// <https://schema.org/scheduledPaymentDate>
pub const SCHEDULED_PAYMENT_DATE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/scheduledPaymentDate";
/// <https://schema.org/scheduledPaymentDate>
pub const SCHEDULED_PAYMENT_DATE_PROPERTY_LABEL: &str = "scheduledPaymentDate";
pub struct ScheduledPaymentDatePropertyIri;
impl PartialEq<&str> for ScheduledPaymentDatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SCHEDULED_PAYMENT_DATE_PROPERTY_IRI_HTTP
			|| *other == SCHEDULED_PAYMENT_DATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ScheduledPaymentDatePropertyIri> for &str {
	fn eq(&self, other: &ScheduledPaymentDatePropertyIri) -> bool {
		*self == SCHEDULED_PAYMENT_DATE_PROPERTY_IRI_HTTP
			|| *self == SCHEDULED_PAYMENT_DATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ScheduledPaymentDatePropertyIriOrLabel;
impl PartialEq<&str> for ScheduledPaymentDatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ScheduledPaymentDatePropertyIri || *other == SCHEDULED_PAYMENT_DATE_PROPERTY_LABEL
	}
}
impl PartialEq<ScheduledPaymentDatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ScheduledPaymentDatePropertyIriOrLabel) -> bool {
		*self == ScheduledPaymentDatePropertyIri || *self == SCHEDULED_PAYMENT_DATE_PROPERTY_LABEL
	}
}
