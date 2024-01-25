/// <https://schema.org/prescriptionStatus>
pub const PRESCRIPTION_STATUS_PROPERTY_IRI_HTTP: &str = "http://schema.org/prescriptionStatus";
/// <https://schema.org/prescriptionStatus>
pub const PRESCRIPTION_STATUS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/prescriptionStatus";
/// <https://schema.org/prescriptionStatus>
pub const PRESCRIPTION_STATUS_PROPERTY_LABEL: &str = "prescriptionStatus";
pub struct PrescriptionStatusPropertyIri;
impl PartialEq<&str> for PrescriptionStatusPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PRESCRIPTION_STATUS_PROPERTY_IRI_HTTP
			|| *other == PRESCRIPTION_STATUS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PrescriptionStatusPropertyIri> for &str {
	fn eq(&self, other: &PrescriptionStatusPropertyIri) -> bool {
		*self == PRESCRIPTION_STATUS_PROPERTY_IRI_HTTP
			|| *self == PRESCRIPTION_STATUS_PROPERTY_IRI_HTTPS
	}
}
pub struct PrescriptionStatusPropertyIriOrLabel;
impl PartialEq<&str> for PrescriptionStatusPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PrescriptionStatusPropertyIri || *other == PRESCRIPTION_STATUS_PROPERTY_LABEL
	}
}
impl PartialEq<PrescriptionStatusPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PrescriptionStatusPropertyIriOrLabel) -> bool {
		*self == PrescriptionStatusPropertyIri || *self == PRESCRIPTION_STATUS_PROPERTY_LABEL
	}
}
