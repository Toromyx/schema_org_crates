/// <https://schema.org/measuredProperty>
pub const MEASURED_PROPERTY_PROPERTY_IRI_HTTP: &str = "http://schema.org/measuredProperty";
/// <https://schema.org/measuredProperty>
pub const MEASURED_PROPERTY_PROPERTY_IRI_HTTPS: &str = "https://schema.org/measuredProperty";
/// <https://schema.org/measuredProperty>
pub const MEASURED_PROPERTY_PROPERTY_LABEL: &str = "measuredProperty";
pub struct MeasuredPropertyPropertyIri;
impl PartialEq<&str> for MeasuredPropertyPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEASURED_PROPERTY_PROPERTY_IRI_HTTP
			|| *other == MEASURED_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MeasuredPropertyPropertyIri> for &str {
	fn eq(&self, other: &MeasuredPropertyPropertyIri) -> bool {
		*self == MEASURED_PROPERTY_PROPERTY_IRI_HTTP
			|| *self == MEASURED_PROPERTY_PROPERTY_IRI_HTTPS
	}
}
pub struct MeasuredPropertyPropertyIriOrLabel;
impl PartialEq<&str> for MeasuredPropertyPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MeasuredPropertyPropertyIri || *other == MEASURED_PROPERTY_PROPERTY_LABEL
	}
}
impl PartialEq<MeasuredPropertyPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MeasuredPropertyPropertyIriOrLabel) -> bool {
		*self == MeasuredPropertyPropertyIri || *self == MEASURED_PROPERTY_PROPERTY_LABEL
	}
}
