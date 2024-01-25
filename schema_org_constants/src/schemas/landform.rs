/// <https://schema.org/Landform>
pub const LANDFORM_IRI_HTTP: &str = "http://schema.org/Landform";
/// <https://schema.org/Landform>
pub const LANDFORM_IRI_HTTPS: &str = "https://schema.org/Landform";
/// <https://schema.org/Landform>
pub const LANDFORM_LABEL: &str = "Landform";
pub struct LandformIri;
impl PartialEq<&str> for LandformIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LANDFORM_IRI_HTTP || *other == LANDFORM_IRI_HTTPS
	}
}
impl PartialEq<LandformIri> for &str {
	fn eq(&self, other: &LandformIri) -> bool {
		*self == LANDFORM_IRI_HTTP || *self == LANDFORM_IRI_HTTPS
	}
}
pub struct LandformIriOrLabel;
impl PartialEq<&str> for LandformIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LandformIri || *other == LANDFORM_LABEL
	}
}
impl PartialEq<LandformIriOrLabel> for &str {
	fn eq(&self, other: &LandformIriOrLabel) -> bool {
		*self == LandformIri || *self == LANDFORM_LABEL
	}
}
