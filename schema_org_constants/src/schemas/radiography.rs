/// <https://schema.org/Radiography>
pub const RADIOGRAPHY_IRI_HTTP: &str = "http://schema.org/Radiography";
/// <https://schema.org/Radiography>
pub const RADIOGRAPHY_IRI_HTTPS: &str = "https://schema.org/Radiography";
/// <https://schema.org/Radiography>
pub const RADIOGRAPHY_LABEL: &str = "Radiography";
pub struct RadiographyIri;
impl PartialEq<&str> for RadiographyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == RADIOGRAPHY_IRI_HTTP || *other == RADIOGRAPHY_IRI_HTTPS
	}
}
impl PartialEq<RadiographyIri> for &str {
	fn eq(&self, other: &RadiographyIri) -> bool {
		*self == RADIOGRAPHY_IRI_HTTP || *self == RADIOGRAPHY_IRI_HTTPS
	}
}
pub struct RadiographyIriOrLabel;
impl PartialEq<&str> for RadiographyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == RadiographyIri || *other == RADIOGRAPHY_LABEL
	}
}
impl PartialEq<RadiographyIriOrLabel> for &str {
	fn eq(&self, other: &RadiographyIriOrLabel) -> bool {
		*self == RadiographyIri || *self == RADIOGRAPHY_LABEL
	}
}
