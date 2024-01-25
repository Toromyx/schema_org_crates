/// <https://schema.org/serviceSmsNumber>
pub const SERVICE_SMS_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/serviceSmsNumber";
/// <https://schema.org/serviceSmsNumber>
pub const SERVICE_SMS_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serviceSmsNumber";
/// <https://schema.org/serviceSmsNumber>
pub const SERVICE_SMS_NUMBER_PROPERTY_LABEL: &str = "serviceSmsNumber";
pub struct ServiceSmsNumberPropertyIri;
impl PartialEq<&str> for ServiceSmsNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERVICE_SMS_NUMBER_PROPERTY_IRI_HTTP
			|| *other == SERVICE_SMS_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ServiceSmsNumberPropertyIri> for &str {
	fn eq(&self, other: &ServiceSmsNumberPropertyIri) -> bool {
		*self == SERVICE_SMS_NUMBER_PROPERTY_IRI_HTTP
			|| *self == SERVICE_SMS_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct ServiceSmsNumberPropertyIriOrLabel;
impl PartialEq<&str> for ServiceSmsNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ServiceSmsNumberPropertyIri || *other == SERVICE_SMS_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<ServiceSmsNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ServiceSmsNumberPropertyIriOrLabel) -> bool {
		*self == ServiceSmsNumberPropertyIri || *self == SERVICE_SMS_NUMBER_PROPERTY_LABEL
	}
}
