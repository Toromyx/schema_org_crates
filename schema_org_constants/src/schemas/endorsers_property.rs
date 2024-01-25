/// <https://schema.org/endorsers>
pub const ENDORSERS_PROPERTY_IRI_HTTP: &str = "http://schema.org/endorsers";
/// <https://schema.org/endorsers>
pub const ENDORSERS_PROPERTY_IRI_HTTPS: &str = "https://schema.org/endorsers";
/// <https://schema.org/endorsers>
pub const ENDORSERS_PROPERTY_LABEL: &str = "endorsers";
pub struct EndorsersPropertyIri;
impl PartialEq<&str> for EndorsersPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ENDORSERS_PROPERTY_IRI_HTTP || *other == ENDORSERS_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<EndorsersPropertyIri> for &str {
	fn eq(&self, other: &EndorsersPropertyIri) -> bool {
		*self == ENDORSERS_PROPERTY_IRI_HTTP || *self == ENDORSERS_PROPERTY_IRI_HTTPS
	}
}
pub struct EndorsersPropertyIriOrLabel;
impl PartialEq<&str> for EndorsersPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == EndorsersPropertyIri || *other == ENDORSERS_PROPERTY_LABEL
	}
}
impl PartialEq<EndorsersPropertyIriOrLabel> for &str {
	fn eq(&self, other: &EndorsersPropertyIriOrLabel) -> bool {
		*self == EndorsersPropertyIri || *self == ENDORSERS_PROPERTY_LABEL
	}
}
