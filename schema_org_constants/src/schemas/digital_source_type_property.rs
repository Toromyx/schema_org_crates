/// <https://schema.org/digitalSourceType>
pub const DIGITAL_SOURCE_TYPE_PROPERTY_IRI_HTTP: &str = "http://schema.org/digitalSourceType";
/// <https://schema.org/digitalSourceType>
pub const DIGITAL_SOURCE_TYPE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/digitalSourceType";
/// <https://schema.org/digitalSourceType>
pub const DIGITAL_SOURCE_TYPE_PROPERTY_LABEL: &str = "digitalSourceType";
pub struct DigitalSourceTypePropertyIri;
impl PartialEq<&str> for DigitalSourceTypePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DIGITAL_SOURCE_TYPE_PROPERTY_IRI_HTTP
			|| *other == DIGITAL_SOURCE_TYPE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DigitalSourceTypePropertyIri> for &str {
	fn eq(&self, other: &DigitalSourceTypePropertyIri) -> bool {
		*self == DIGITAL_SOURCE_TYPE_PROPERTY_IRI_HTTP
			|| *self == DIGITAL_SOURCE_TYPE_PROPERTY_IRI_HTTPS
	}
}
pub struct DigitalSourceTypePropertyIriOrLabel;
impl PartialEq<&str> for DigitalSourceTypePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DigitalSourceTypePropertyIri || *other == DIGITAL_SOURCE_TYPE_PROPERTY_LABEL
	}
}
impl PartialEq<DigitalSourceTypePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DigitalSourceTypePropertyIriOrLabel) -> bool {
		*self == DigitalSourceTypePropertyIri || *self == DIGITAL_SOURCE_TYPE_PROPERTY_LABEL
	}
}
