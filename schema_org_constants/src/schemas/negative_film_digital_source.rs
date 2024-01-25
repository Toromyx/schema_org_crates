/// <https://schema.org/NegativeFilmDigitalSource>
pub const NEGATIVE_FILM_DIGITAL_SOURCE_IRI_HTTP: &str =
	"http://schema.org/NegativeFilmDigitalSource";
/// <https://schema.org/NegativeFilmDigitalSource>
pub const NEGATIVE_FILM_DIGITAL_SOURCE_IRI_HTTPS: &str =
	"https://schema.org/NegativeFilmDigitalSource";
/// <https://schema.org/NegativeFilmDigitalSource>
pub const NEGATIVE_FILM_DIGITAL_SOURCE_LABEL: &str = "NegativeFilmDigitalSource";
pub struct NegativeFilmDigitalSourceIri;
impl PartialEq<&str> for NegativeFilmDigitalSourceIri {
	fn eq(&self, other: &&str) -> bool {
		*other == NEGATIVE_FILM_DIGITAL_SOURCE_IRI_HTTP
			|| *other == NEGATIVE_FILM_DIGITAL_SOURCE_IRI_HTTPS
	}
}
impl PartialEq<NegativeFilmDigitalSourceIri> for &str {
	fn eq(&self, other: &NegativeFilmDigitalSourceIri) -> bool {
		*self == NEGATIVE_FILM_DIGITAL_SOURCE_IRI_HTTP
			|| *self == NEGATIVE_FILM_DIGITAL_SOURCE_IRI_HTTPS
	}
}
pub struct NegativeFilmDigitalSourceIriOrLabel;
impl PartialEq<&str> for NegativeFilmDigitalSourceIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == NegativeFilmDigitalSourceIri || *other == NEGATIVE_FILM_DIGITAL_SOURCE_LABEL
	}
}
impl PartialEq<NegativeFilmDigitalSourceIriOrLabel> for &str {
	fn eq(&self, other: &NegativeFilmDigitalSourceIriOrLabel) -> bool {
		*self == NegativeFilmDigitalSourceIri || *self == NEGATIVE_FILM_DIGITAL_SOURCE_LABEL
	}
}
