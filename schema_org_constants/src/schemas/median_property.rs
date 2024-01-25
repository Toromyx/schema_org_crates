/// <https://schema.org/median>
pub const MEDIAN_PROPERTY_IRI_HTTP: &str = "http://schema.org/median";
/// <https://schema.org/median>
pub const MEDIAN_PROPERTY_IRI_HTTPS: &str = "https://schema.org/median";
/// <https://schema.org/median>
pub const MEDIAN_PROPERTY_LABEL: &str = "median";
pub struct MedianPropertyIri;
impl PartialEq<&str> for MedianPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MEDIAN_PROPERTY_IRI_HTTP || *other == MEDIAN_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<MedianPropertyIri> for &str {
	fn eq(&self, other: &MedianPropertyIri) -> bool {
		*self == MEDIAN_PROPERTY_IRI_HTTP || *self == MEDIAN_PROPERTY_IRI_HTTPS
	}
}
pub struct MedianPropertyIriOrLabel;
impl PartialEq<&str> for MedianPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MedianPropertyIri || *other == MEDIAN_PROPERTY_LABEL
	}
}
impl PartialEq<MedianPropertyIriOrLabel> for &str {
	fn eq(&self, other: &MedianPropertyIriOrLabel) -> bool {
		*self == MedianPropertyIri || *self == MEDIAN_PROPERTY_LABEL
	}
}
