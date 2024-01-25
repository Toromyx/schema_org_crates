/// <https://schema.org/AdministrativeArea>
pub const ADMINISTRATIVE_AREA_IRI_HTTP: &str = "http://schema.org/AdministrativeArea";
/// <https://schema.org/AdministrativeArea>
pub const ADMINISTRATIVE_AREA_IRI_HTTPS: &str = "https://schema.org/AdministrativeArea";
/// <https://schema.org/AdministrativeArea>
pub const ADMINISTRATIVE_AREA_LABEL: &str = "AdministrativeArea";
pub struct AdministrativeAreaIri;
impl PartialEq<&str> for AdministrativeAreaIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ADMINISTRATIVE_AREA_IRI_HTTP || *other == ADMINISTRATIVE_AREA_IRI_HTTPS
	}
}
impl PartialEq<AdministrativeAreaIri> for &str {
	fn eq(&self, other: &AdministrativeAreaIri) -> bool {
		*self == ADMINISTRATIVE_AREA_IRI_HTTP || *self == ADMINISTRATIVE_AREA_IRI_HTTPS
	}
}
pub struct AdministrativeAreaIriOrLabel;
impl PartialEq<&str> for AdministrativeAreaIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AdministrativeAreaIri || *other == ADMINISTRATIVE_AREA_LABEL
	}
}
impl PartialEq<AdministrativeAreaIriOrLabel> for &str {
	fn eq(&self, other: &AdministrativeAreaIriOrLabel) -> bool {
		*self == AdministrativeAreaIri || *self == ADMINISTRATIVE_AREA_LABEL
	}
}
