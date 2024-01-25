/// <https://schema.org/IPTCDigitalSourceEnumeration>
pub const IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTP: &str =
	"http://schema.org/IPTCDigitalSourceEnumeration";
/// <https://schema.org/IPTCDigitalSourceEnumeration>
pub const IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTPS: &str =
	"https://schema.org/IPTCDigitalSourceEnumeration";
/// <https://schema.org/IPTCDigitalSourceEnumeration>
pub const IPTC_DIGITAL_SOURCE_ENUMERATION_LABEL: &str = "IPTCDigitalSourceEnumeration";
pub struct IptcDigitalSourceEnumerationIri;
impl PartialEq<&str> for IptcDigitalSourceEnumerationIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTP
			|| *other == IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTPS
	}
}
impl PartialEq<IptcDigitalSourceEnumerationIri> for &str {
	fn eq(&self, other: &IptcDigitalSourceEnumerationIri) -> bool {
		*self == IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTP
			|| *self == IPTC_DIGITAL_SOURCE_ENUMERATION_IRI_HTTPS
	}
}
pub struct IptcDigitalSourceEnumerationIriOrLabel;
impl PartialEq<&str> for IptcDigitalSourceEnumerationIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IptcDigitalSourceEnumerationIri || *other == IPTC_DIGITAL_SOURCE_ENUMERATION_LABEL
	}
}
impl PartialEq<IptcDigitalSourceEnumerationIriOrLabel> for &str {
	fn eq(&self, other: &IptcDigitalSourceEnumerationIriOrLabel) -> bool {
		*self == IptcDigitalSourceEnumerationIri || *self == IPTC_DIGITAL_SOURCE_ENUMERATION_LABEL
	}
}
