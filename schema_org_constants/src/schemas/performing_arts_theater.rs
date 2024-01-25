/// <https://schema.org/PerformingArtsTheater>
pub const PERFORMING_ARTS_THEATER_IRI_HTTP: &str = "http://schema.org/PerformingArtsTheater";
/// <https://schema.org/PerformingArtsTheater>
pub const PERFORMING_ARTS_THEATER_IRI_HTTPS: &str = "https://schema.org/PerformingArtsTheater";
/// <https://schema.org/PerformingArtsTheater>
pub const PERFORMING_ARTS_THEATER_LABEL: &str = "PerformingArtsTheater";
pub struct PerformingArtsTheaterIri;
impl PartialEq<&str> for PerformingArtsTheaterIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERFORMING_ARTS_THEATER_IRI_HTTP || *other == PERFORMING_ARTS_THEATER_IRI_HTTPS
	}
}
impl PartialEq<PerformingArtsTheaterIri> for &str {
	fn eq(&self, other: &PerformingArtsTheaterIri) -> bool {
		*self == PERFORMING_ARTS_THEATER_IRI_HTTP || *self == PERFORMING_ARTS_THEATER_IRI_HTTPS
	}
}
pub struct PerformingArtsTheaterIriOrLabel;
impl PartialEq<&str> for PerformingArtsTheaterIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PerformingArtsTheaterIri || *other == PERFORMING_ARTS_THEATER_LABEL
	}
}
impl PartialEq<PerformingArtsTheaterIriOrLabel> for &str {
	fn eq(&self, other: &PerformingArtsTheaterIriOrLabel) -> bool {
		*self == PerformingArtsTheaterIri || *self == PERFORMING_ARTS_THEATER_LABEL
	}
}
