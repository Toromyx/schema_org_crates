/// <https://schema.org/Abdomen>
pub const ABDOMEN_IRI_HTTP: &str = "http://schema.org/Abdomen";
/// <https://schema.org/Abdomen>
pub const ABDOMEN_IRI_HTTPS: &str = "https://schema.org/Abdomen";
/// <https://schema.org/Abdomen>
pub const ABDOMEN_LABEL: &str = "Abdomen";
pub struct AbdomenIri;
impl PartialEq<&str> for AbdomenIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ABDOMEN_IRI_HTTP || *other == ABDOMEN_IRI_HTTPS
	}
}
impl PartialEq<AbdomenIri> for &str {
	fn eq(&self, other: &AbdomenIri) -> bool {
		*self == ABDOMEN_IRI_HTTP || *self == ABDOMEN_IRI_HTTPS
	}
}
pub struct AbdomenIriOrLabel;
impl PartialEq<&str> for AbdomenIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == AbdomenIri || *other == ABDOMEN_LABEL
	}
}
impl PartialEq<AbdomenIriOrLabel> for &str {
	fn eq(&self, other: &AbdomenIriOrLabel) -> bool {
		*self == AbdomenIri || *self == ABDOMEN_LABEL
	}
}
