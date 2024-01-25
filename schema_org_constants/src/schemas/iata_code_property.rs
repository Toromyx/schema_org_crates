/// <https://schema.org/iataCode>
pub const IATA_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/iataCode";
/// <https://schema.org/iataCode>
pub const IATA_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/iataCode";
/// <https://schema.org/iataCode>
pub const IATA_CODE_PROPERTY_LABEL: &str = "iataCode";
pub struct IataCodePropertyIri;
impl PartialEq<&str> for IataCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IATA_CODE_PROPERTY_IRI_HTTP || *other == IATA_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IataCodePropertyIri> for &str {
	fn eq(&self, other: &IataCodePropertyIri) -> bool {
		*self == IATA_CODE_PROPERTY_IRI_HTTP || *self == IATA_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct IataCodePropertyIriOrLabel;
impl PartialEq<&str> for IataCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IataCodePropertyIri || *other == IATA_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<IataCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &IataCodePropertyIriOrLabel) -> bool {
		*self == IataCodePropertyIri || *self == IATA_CODE_PROPERTY_LABEL
	}
}
