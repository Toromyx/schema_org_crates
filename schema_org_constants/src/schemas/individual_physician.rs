/// <https://schema.org/IndividualPhysician>
pub const INDIVIDUAL_PHYSICIAN_IRI_HTTP: &str = "http://schema.org/IndividualPhysician";
/// <https://schema.org/IndividualPhysician>
pub const INDIVIDUAL_PHYSICIAN_IRI_HTTPS: &str = "https://schema.org/IndividualPhysician";
/// <https://schema.org/IndividualPhysician>
pub const INDIVIDUAL_PHYSICIAN_LABEL: &str = "IndividualPhysician";
pub struct IndividualPhysicianIri;
impl PartialEq<&str> for IndividualPhysicianIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INDIVIDUAL_PHYSICIAN_IRI_HTTP || *other == INDIVIDUAL_PHYSICIAN_IRI_HTTPS
	}
}
impl PartialEq<IndividualPhysicianIri> for &str {
	fn eq(&self, other: &IndividualPhysicianIri) -> bool {
		*self == INDIVIDUAL_PHYSICIAN_IRI_HTTP || *self == INDIVIDUAL_PHYSICIAN_IRI_HTTPS
	}
}
pub struct IndividualPhysicianIriOrLabel;
impl PartialEq<&str> for IndividualPhysicianIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IndividualPhysicianIri || *other == INDIVIDUAL_PHYSICIAN_LABEL
	}
}
impl PartialEq<IndividualPhysicianIriOrLabel> for &str {
	fn eq(&self, other: &IndividualPhysicianIriOrLabel) -> bool {
		*self == IndividualPhysicianIri || *self == INDIVIDUAL_PHYSICIAN_LABEL
	}
}
