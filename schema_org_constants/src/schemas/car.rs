/// <https://schema.org/Car>
pub const CAR_IRI_HTTP: &str = "http://schema.org/Car";
/// <https://schema.org/Car>
pub const CAR_IRI_HTTPS: &str = "https://schema.org/Car";
/// <https://schema.org/Car>
pub const CAR_LABEL: &str = "Car";
pub struct CarIri;
impl PartialEq<&str> for CarIri {
	fn eq(&self, other: &&str) -> bool {
		*other == CAR_IRI_HTTP || *other == CAR_IRI_HTTPS
	}
}
impl PartialEq<CarIri> for &str {
	fn eq(&self, other: &CarIri) -> bool {
		*self == CAR_IRI_HTTP || *self == CAR_IRI_HTTPS
	}
}
pub struct CarIriOrLabel;
impl PartialEq<&str> for CarIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CarIri || *other == CAR_LABEL
	}
}
impl PartialEq<CarIriOrLabel> for &str {
	fn eq(&self, other: &CarIriOrLabel) -> bool {
		*self == CarIri || *self == CAR_LABEL
	}
}
