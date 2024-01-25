/// <https://schema.org/Motorcycle>
pub const MOTORCYCLE_IRI_HTTP: &str = "http://schema.org/Motorcycle";
/// <https://schema.org/Motorcycle>
pub const MOTORCYCLE_IRI_HTTPS: &str = "https://schema.org/Motorcycle";
/// <https://schema.org/Motorcycle>
pub const MOTORCYCLE_LABEL: &str = "Motorcycle";
pub struct MotorcycleIri;
impl PartialEq<&str> for MotorcycleIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MOTORCYCLE_IRI_HTTP || *other == MOTORCYCLE_IRI_HTTPS
	}
}
impl PartialEq<MotorcycleIri> for &str {
	fn eq(&self, other: &MotorcycleIri) -> bool {
		*self == MOTORCYCLE_IRI_HTTP || *self == MOTORCYCLE_IRI_HTTPS
	}
}
pub struct MotorcycleIriOrLabel;
impl PartialEq<&str> for MotorcycleIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MotorcycleIri || *other == MOTORCYCLE_LABEL
	}
}
impl PartialEq<MotorcycleIriOrLabel> for &str {
	fn eq(&self, other: &MotorcycleIriOrLabel) -> bool {
		*self == MotorcycleIri || *self == MOTORCYCLE_LABEL
	}
}
