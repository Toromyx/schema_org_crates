/// <https://schema.org/serialNumber>
pub const SERIAL_NUMBER_PROPERTY_IRI_HTTP: &str = "http://schema.org/serialNumber";
/// <https://schema.org/serialNumber>
pub const SERIAL_NUMBER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/serialNumber";
/// <https://schema.org/serialNumber>
pub const SERIAL_NUMBER_PROPERTY_LABEL: &str = "serialNumber";
pub struct SerialNumberPropertyIri;
impl PartialEq<&str> for SerialNumberPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SERIAL_NUMBER_PROPERTY_IRI_HTTP || *other == SERIAL_NUMBER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<SerialNumberPropertyIri> for &str {
	fn eq(&self, other: &SerialNumberPropertyIri) -> bool {
		*self == SERIAL_NUMBER_PROPERTY_IRI_HTTP || *self == SERIAL_NUMBER_PROPERTY_IRI_HTTPS
	}
}
pub struct SerialNumberPropertyIriOrLabel;
impl PartialEq<&str> for SerialNumberPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SerialNumberPropertyIri || *other == SERIAL_NUMBER_PROPERTY_LABEL
	}
}
impl PartialEq<SerialNumberPropertyIriOrLabel> for &str {
	fn eq(&self, other: &SerialNumberPropertyIriOrLabel) -> bool {
		*self == SerialNumberPropertyIri || *self == SERIAL_NUMBER_PROPERTY_LABEL
	}
}
