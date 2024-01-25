/// <https://schema.org/trailerWeight>
pub const TRAILER_WEIGHT_PROPERTY_IRI_HTTP: &str = "http://schema.org/trailerWeight";
/// <https://schema.org/trailerWeight>
pub const TRAILER_WEIGHT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/trailerWeight";
/// <https://schema.org/trailerWeight>
pub const TRAILER_WEIGHT_PROPERTY_LABEL: &str = "trailerWeight";
pub struct TrailerWeightPropertyIri;
impl PartialEq<&str> for TrailerWeightPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == TRAILER_WEIGHT_PROPERTY_IRI_HTTP || *other == TRAILER_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<TrailerWeightPropertyIri> for &str {
	fn eq(&self, other: &TrailerWeightPropertyIri) -> bool {
		*self == TRAILER_WEIGHT_PROPERTY_IRI_HTTP || *self == TRAILER_WEIGHT_PROPERTY_IRI_HTTPS
	}
}
pub struct TrailerWeightPropertyIriOrLabel;
impl PartialEq<&str> for TrailerWeightPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == TrailerWeightPropertyIri || *other == TRAILER_WEIGHT_PROPERTY_LABEL
	}
}
impl PartialEq<TrailerWeightPropertyIriOrLabel> for &str {
	fn eq(&self, other: &TrailerWeightPropertyIriOrLabel) -> bool {
		*self == TrailerWeightPropertyIri || *self == TRAILER_WEIGHT_PROPERTY_LABEL
	}
}
