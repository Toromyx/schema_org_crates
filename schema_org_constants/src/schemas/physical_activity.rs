/// <https://schema.org/PhysicalActivity>
pub const PHYSICAL_ACTIVITY_IRI_HTTP: &str = "http://schema.org/PhysicalActivity";
/// <https://schema.org/PhysicalActivity>
pub const PHYSICAL_ACTIVITY_IRI_HTTPS: &str = "https://schema.org/PhysicalActivity";
/// <https://schema.org/PhysicalActivity>
pub const PHYSICAL_ACTIVITY_LABEL: &str = "PhysicalActivity";
pub struct PhysicalActivityIri;
impl PartialEq<&str> for PhysicalActivityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PHYSICAL_ACTIVITY_IRI_HTTP || *other == PHYSICAL_ACTIVITY_IRI_HTTPS
	}
}
impl PartialEq<PhysicalActivityIri> for &str {
	fn eq(&self, other: &PhysicalActivityIri) -> bool {
		*self == PHYSICAL_ACTIVITY_IRI_HTTP || *self == PHYSICAL_ACTIVITY_IRI_HTTPS
	}
}
pub struct PhysicalActivityIriOrLabel;
impl PartialEq<&str> for PhysicalActivityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PhysicalActivityIri || *other == PHYSICAL_ACTIVITY_LABEL
	}
}
impl PartialEq<PhysicalActivityIriOrLabel> for &str {
	fn eq(&self, other: &PhysicalActivityIriOrLabel) -> bool {
		*self == PhysicalActivityIri || *self == PHYSICAL_ACTIVITY_LABEL
	}
}
