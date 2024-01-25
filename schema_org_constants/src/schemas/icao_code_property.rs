/// <https://schema.org/icaoCode>
pub const ICAO_CODE_PROPERTY_IRI_HTTP: &str = "http://schema.org/icaoCode";
/// <https://schema.org/icaoCode>
pub const ICAO_CODE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/icaoCode";
/// <https://schema.org/icaoCode>
pub const ICAO_CODE_PROPERTY_LABEL: &str = "icaoCode";
pub struct IcaoCodePropertyIri;
impl PartialEq<&str> for IcaoCodePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ICAO_CODE_PROPERTY_IRI_HTTP || *other == ICAO_CODE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<IcaoCodePropertyIri> for &str {
	fn eq(&self, other: &IcaoCodePropertyIri) -> bool {
		*self == ICAO_CODE_PROPERTY_IRI_HTTP || *self == ICAO_CODE_PROPERTY_IRI_HTTPS
	}
}
pub struct IcaoCodePropertyIriOrLabel;
impl PartialEq<&str> for IcaoCodePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == IcaoCodePropertyIri || *other == ICAO_CODE_PROPERTY_LABEL
	}
}
impl PartialEq<IcaoCodePropertyIriOrLabel> for &str {
	fn eq(&self, other: &IcaoCodePropertyIriOrLabel) -> bool {
		*self == IcaoCodePropertyIri || *self == ICAO_CODE_PROPERTY_LABEL
	}
}
