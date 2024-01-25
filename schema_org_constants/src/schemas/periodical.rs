/// <https://schema.org/Periodical>
pub const PERIODICAL_IRI_HTTP: &str = "http://schema.org/Periodical";
/// <https://schema.org/Periodical>
pub const PERIODICAL_IRI_HTTPS: &str = "https://schema.org/Periodical";
/// <https://schema.org/Periodical>
pub const PERIODICAL_LABEL: &str = "Periodical";
pub struct PeriodicalIri;
impl PartialEq<&str> for PeriodicalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PERIODICAL_IRI_HTTP || *other == PERIODICAL_IRI_HTTPS
	}
}
impl PartialEq<PeriodicalIri> for &str {
	fn eq(&self, other: &PeriodicalIri) -> bool {
		*self == PERIODICAL_IRI_HTTP || *self == PERIODICAL_IRI_HTTPS
	}
}
pub struct PeriodicalIriOrLabel;
impl PartialEq<&str> for PeriodicalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PeriodicalIri || *other == PERIODICAL_LABEL
	}
}
impl PartialEq<PeriodicalIriOrLabel> for &str {
	fn eq(&self, other: &PeriodicalIriOrLabel) -> bool {
		*self == PeriodicalIri || *self == PERIODICAL_LABEL
	}
}
