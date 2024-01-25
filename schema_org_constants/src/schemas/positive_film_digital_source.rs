/// <https://schema.org/PositiveFilmDigitalSource>
pub const POSITIVE_FILM_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/PositiveFilmDigitalSource";
/// <https://schema.org/PositiveFilmDigitalSource>
pub const POSITIVE_FILM_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/PositiveFilmDigitalSource";
/// <https://schema.org/PositiveFilmDigitalSource>
pub const POSITIVE_FILM_DIGITAL_SOURCE_LABEL: &str = "PositiveFilmDigitalSource";
pub struct PositiveFilmDigitalSourceIri;
impl PartialEq<&str> for PositiveFilmDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == POSITIVE_FILM_DIGITAL_SOURCE_IRI_HTTP
			|| *other == POSITIVE_FILM_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<PositiveFilmDigitalSourceIri> for &str {
	fn eq(&self, other: &PositiveFilmDigitalSourceIri) -> bool {
		*self == POSITIVE_FILM_DIGITAL_SOURCE_IRI_HTTP
			|| *self == POSITIVE_FILM_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct PositiveFilmDigitalSourceIriOrLabel;
impl PartialEq<&str> for PositiveFilmDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PositiveFilmDigitalSourceIri || *other == POSITIVE_FILM_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<PositiveFilmDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &PositiveFilmDigitalSourceIriOrLabel) -> bool {
		*self == PositiveFilmDigitalSourceIri || *self == POSITIVE_FILM_DIGITAL_SOURCE_LABEL
	}
}
