/// <https://schema.org/prescribingInfo>
pub const PRESCRIBING_INFO_PROPERTY_IRI_HTTP: &str = "http://schema.org/prescribingInfo";
/// <https://schema.org/prescribingInfo>
pub const PRESCRIBING_INFO_PROPERTY_IRI_HTTPS: &str = "https://schema.org/prescribingInfo";
/// <https://schema.org/prescribingInfo>
pub const PRESCRIBING_INFO_PROPERTY_LABEL: &str = "prescribingInfo";
pub struct PrescribingInfoPropertyIri;
impl PartialEq<&str> for PrescribingInfoPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRESCRIBING_INFO_PROPERTY_IRI_HTTP
			|| *other == PRESCRIBING_INFO_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrescribingInfoPropertyIri> for &str {
	fn eq(&self, other: &PrescribingInfoPropertyIri) -> bool {
		*self == PRESCRIBING_INFO_PROPERTY_IRI_HTTP || *self == PRESCRIBING_INFO_PROPERTY_IRI_HTTPS
	}
}
pub struct PrescribingInfoPropertyIriOrLabel;
impl PartialEq<&str> for PrescribingInfoPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrescribingInfoPropertyIri || *other == PRESCRIBING_INFO_PROPERTY_LABEL
	}
}
impl PartialEq<PrescribingInfoPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrescribingInfoPropertyIriOrLabel) -> bool {
		*self == PrescribingInfoPropertyIri || *self == PRESCRIBING_INFO_PROPERTY_LABEL
	}
}
