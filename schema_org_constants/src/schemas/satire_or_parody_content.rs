/// <https://schema.org/SatireOrParodyContent>
pub const SATIRE_OR_PARODY_CONTENT_IRI_HTTP: &str = "http://schema.org/SatireOrParodyContent";
/// <https://schema.org/SatireOrParodyContent>
pub const SATIRE_OR_PARODY_CONTENT_IRI_HTTPS: &str = "https://schema.org/SatireOrParodyContent";
/// <https://schema.org/SatireOrParodyContent>
pub const SATIRE_OR_PARODY_CONTENT_LABEL: &str = "SatireOrParodyContent";
pub struct SatireOrParodyContentIri;
impl PartialEq<&str> for SatireOrParodyContentIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SATIRE_OR_PARODY_CONTENT_IRI_HTTP || *other == SATIRE_OR_PARODY_CONTENT_IRI_HTTPS
	}
}
impl PartialEq<SatireOrParodyContentIri> for &str {
	fn eq(&self, other: &SatireOrParodyContentIri) -> bool {
		*self == SATIRE_OR_PARODY_CONTENT_IRI_HTTP || *self == SATIRE_OR_PARODY_CONTENT_IRI_HTTPS
	}
}
pub struct SatireOrParodyContentIriOrLabel;
impl PartialEq<&str> for SatireOrParodyContentIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SatireOrParodyContentIri || *other == SATIRE_OR_PARODY_CONTENT_LABEL
	}
}
impl PartialEq<SatireOrParodyContentIriOrLabel> for &str {
	fn eq(&self, other: &SatireOrParodyContentIriOrLabel) -> bool {
		*self == SatireOrParodyContentIri || *self == SATIRE_OR_PARODY_CONTENT_LABEL
	}
}
