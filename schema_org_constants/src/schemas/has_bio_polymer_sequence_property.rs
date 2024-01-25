/// <https://schema.org/hasBioPolymerSequence>
pub const HAS_BIO_POLYMER_SEQUENCE_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/hasBioPolymerSequence";
/// <https://schema.org/hasBioPolymerSequence>
pub const HAS_BIO_POLYMER_SEQUENCE_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/hasBioPolymerSequence";
/// <https://schema.org/hasBioPolymerSequence>
pub const HAS_BIO_POLYMER_SEQUENCE_PROPERTY_LABEL: &str = "hasBioPolymerSequence";
pub struct HasBioPolymerSequencePropertyIri;
impl PartialEq<&str> for HasBioPolymerSequencePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == HAS_BIO_POLYMER_SEQUENCE_PROPERTY_IRI_HTTP
			|| *other == HAS_BIO_POLYMER_SEQUENCE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<HasBioPolymerSequencePropertyIri> for &str {
	fn eq(&self, other: &HasBioPolymerSequencePropertyIri) -> bool {
		*self == HAS_BIO_POLYMER_SEQUENCE_PROPERTY_IRI_HTTP
			|| *self == HAS_BIO_POLYMER_SEQUENCE_PROPERTY_IRI_HTTPS
	}
}
pub struct HasBioPolymerSequencePropertyIriOrLabel;
impl PartialEq<&str> for HasBioPolymerSequencePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == HasBioPolymerSequencePropertyIri
			|| *other == HAS_BIO_POLYMER_SEQUENCE_PROPERTY_LABEL
	}
}
impl PartialEq<HasBioPolymerSequencePropertyIriOrLabel> for &str {
	fn eq(&self, other: &HasBioPolymerSequencePropertyIriOrLabel) -> bool {
		*self == HasBioPolymerSequencePropertyIri
			|| *self == HAS_BIO_POLYMER_SEQUENCE_PROPERTY_LABEL
	}
}
