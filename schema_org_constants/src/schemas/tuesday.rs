/// <https://schema.org/Tuesday>
pub const TUESDAY_IRI_HTTP: &str = "http://schema.org/Tuesday";
/// <https://schema.org/Tuesday>
pub const TUESDAY_IRI_HTTPS: &str = "https://schema.org/Tuesday";
/// <https://schema.org/Tuesday>
pub const TUESDAY_LABEL: &str = "Tuesday";
pub struct TuesdayIri;
impl PartialEq<&str> for TuesdayIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TUESDAY_IRI_HTTP || *other == TUESDAY_IRI_HTTPS
	}
}
impl PartialEq<TuesdayIri> for &str {
	fn eq(&self, other: &TuesdayIri) -> bool {
		*self == TUESDAY_IRI_HTTP || *self == TUESDAY_IRI_HTTPS
	}
}
pub struct TuesdayIriOrLabel;
impl PartialEq<&str> for TuesdayIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TuesdayIri || *other == TUESDAY_LABEL
	}
}
impl PartialEq<TuesdayIriOrLabel> for &str {
	fn eq(&self, other: &TuesdayIriOrLabel) -> bool {
		*self == TuesdayIri || *self == TUESDAY_LABEL
	}
}
