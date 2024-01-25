/// <https://schema.org/competitor>
pub const COMPETITOR_PROPERTY_IRI_HTTP: &str = "http://schema.org/competitor";
/// <https://schema.org/competitor>
pub const COMPETITOR_PROPERTY_IRI_HTTPS: &str = "https://schema.org/competitor";
/// <https://schema.org/competitor>
pub const COMPETITOR_PROPERTY_LABEL: &str = "competitor";
pub struct CompetitorPropertyIri;
impl PartialEq<&str> for CompetitorPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == COMPETITOR_PROPERTY_IRI_HTTP || *other == COMPETITOR_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<CompetitorPropertyIri> for &str {
	fn eq(&self, other: &CompetitorPropertyIri) -> bool {
		*self == COMPETITOR_PROPERTY_IRI_HTTP || *self == COMPETITOR_PROPERTY_IRI_HTTPS
	}
}
pub struct CompetitorPropertyIriOrLabel;
impl PartialEq<&str> for CompetitorPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == CompetitorPropertyIri || *other == COMPETITOR_PROPERTY_LABEL
	}
}
impl PartialEq<CompetitorPropertyIriOrLabel> for &str {
	fn eq(&self, other: &CompetitorPropertyIriOrLabel) -> bool {
		*self == CompetitorPropertyIri || *self == COMPETITOR_PROPERTY_LABEL
	}
}
