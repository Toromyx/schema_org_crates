/// <https://schema.org/Physician>
pub const PHYSICIAN_IRI_HTTP: &str = "http://schema.org/Physician";
/// <https://schema.org/Physician>
pub const PHYSICIAN_IRI_HTTPS: &str = "https://schema.org/Physician";
/// <https://schema.org/Physician>
pub const PHYSICIAN_LABEL: &str = "Physician";
pub struct PhysicianIri;
impl PartialEq<&str> for PhysicianIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSICIAN_IRI_HTTP || *other == PHYSICIAN_IRI_HTTPS
	}
}
impl PartialEq<PhysicianIri> for &str {
	fn eq(&self, other: &PhysicianIri) -> bool {
		*self == PHYSICIAN_IRI_HTTP || *self == PHYSICIAN_IRI_HTTPS
	}
}
pub struct PhysicianIriOrLabel;
impl PartialEq<&str> for PhysicianIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysicianIri || *other == PHYSICIAN_LABEL
	}
}
impl PartialEq<PhysicianIriOrLabel> for &str {
	fn eq(&self, other: &PhysicianIriOrLabel) -> bool {
		*self == PhysicianIri || *self == PHYSICIAN_LABEL
	}
}
