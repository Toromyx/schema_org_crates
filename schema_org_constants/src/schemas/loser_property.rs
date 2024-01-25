/// <https://schema.org/loser>
pub const LOSER_PROPERTY_IRI_HTTP: &str = "http://schema.org/loser";
/// <https://schema.org/loser>
pub const LOSER_PROPERTY_IRI_HTTPS: &str = "https://schema.org/loser";
/// <https://schema.org/loser>
pub const LOSER_PROPERTY_LABEL: &str = "loser";
pub struct LoserPropertyIri;
impl PartialEq<&str> for LoserPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == LOSER_PROPERTY_IRI_HTTP || *other == LOSER_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<LoserPropertyIri> for &str {
	fn eq(&self, other: &LoserPropertyIri) -> bool {
		*self == LOSER_PROPERTY_IRI_HTTP || *self == LOSER_PROPERTY_IRI_HTTPS
	}
}
pub struct LoserPropertyIriOrLabel;
impl PartialEq<&str> for LoserPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == LoserPropertyIri || *other == LOSER_PROPERTY_LABEL
	}
}
impl PartialEq<LoserPropertyIriOrLabel> for &str {
	fn eq(&self, other: &LoserPropertyIriOrLabel) -> bool {
		*self == LoserPropertyIri || *self == LOSER_PROPERTY_LABEL
	}
}
