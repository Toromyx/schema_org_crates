/// <https://schema.org/Museum>
pub const MUSEUM_IRI_HTTP: &str = "http://schema.org/Museum";
/// <https://schema.org/Museum>
pub const MUSEUM_IRI_HTTPS: &str = "https://schema.org/Museum";
/// <https://schema.org/Museum>
pub const MUSEUM_LABEL: &str = "Museum";
pub struct MuseumIri;
impl PartialEq<&str> for MuseumIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSEUM_IRI_HTTP || *other == MUSEUM_IRI_HTTPS
	}
}
impl PartialEq<MuseumIri> for &str {
	fn eq(&self, other: &MuseumIri) -> bool {
		*self == MUSEUM_IRI_HTTP || *self == MUSEUM_IRI_HTTPS
	}
}
pub struct MuseumIriOrLabel;
impl PartialEq<&str> for MuseumIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MuseumIri || *other == MUSEUM_LABEL
	}
}
impl PartialEq<MuseumIriOrLabel> for &str {
	fn eq(&self, other: &MuseumIriOrLabel) -> bool {
		*self == MuseumIri || *self == MUSEUM_LABEL
	}
}
