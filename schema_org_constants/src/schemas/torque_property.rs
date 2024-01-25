/// <https://schema.org/torque>
pub const TORQUE_PROPERTY_IRI_HTTP: &str = "http://schema.org/torque";
/// <https://schema.org/torque>
pub const TORQUE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/torque";
/// <https://schema.org/torque>
pub const TORQUE_PROPERTY_LABEL: &str = "torque";
pub struct TorquePropertyIri;
impl PartialEq<&str> for TorquePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TORQUE_PROPERTY_IRI_HTTP || *other == TORQUE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TorquePropertyIri> for &str {
	fn eq(&self, other: &TorquePropertyIri) -> bool {
		*self == TORQUE_PROPERTY_IRI_HTTP || *self == TORQUE_PROPERTY_IRI_HTTPS
	}
}
pub struct TorquePropertyIriOrLabel;
impl PartialEq<&str> for TorquePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TorquePropertyIri || *other == TORQUE_PROPERTY_LABEL
	}
}
impl PartialEq<TorquePropertyIriOrLabel> for &str {
	fn eq(&self, other: &TorquePropertyIriOrLabel) -> bool {
		*self == TorquePropertyIri || *self == TORQUE_PROPERTY_LABEL
	}
}
