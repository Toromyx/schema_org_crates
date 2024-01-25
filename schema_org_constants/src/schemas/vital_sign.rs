/// <https://schema.org/VitalSign>
pub const VITAL_SIGN_IRI_HTTP: &str = "http://schema.org/VitalSign";
/// <https://schema.org/VitalSign>
pub const VITAL_SIGN_IRI_HTTPS: &str = "https://schema.org/VitalSign";
/// <https://schema.org/VitalSign>
pub const VITAL_SIGN_LABEL: &str = "VitalSign";
pub struct VitalSignIri;
impl PartialEq<&str> for VitalSignIri {
	fn eq(&self, other: &&str) -> bool {
		*other == VITAL_SIGN_IRI_HTTP || *other == VITAL_SIGN_IRI_HTTPS
	}
}
impl PartialEq<VitalSignIri> for &str {
	fn eq(&self, other: &VitalSignIri) -> bool {
		*self == VITAL_SIGN_IRI_HTTP || *self == VITAL_SIGN_IRI_HTTPS
	}
}
pub struct VitalSignIriOrLabel;
impl PartialEq<&str> for VitalSignIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == VitalSignIri || *other == VITAL_SIGN_LABEL
	}
}
impl PartialEq<VitalSignIriOrLabel> for &str {
	fn eq(&self, other: &VitalSignIriOrLabel) -> bool {
		*self == VitalSignIri || *self == VITAL_SIGN_LABEL
	}
}
