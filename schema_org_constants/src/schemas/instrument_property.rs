/// <https://schema.org/instrument>
pub const INSTRUMENT_PROPERTY_IRI_HTTP: &str = "http://schema.org/instrument";
/// <https://schema.org/instrument>
pub const INSTRUMENT_PROPERTY_IRI_HTTPS: &str = "https://schema.org/instrument";
/// <https://schema.org/instrument>
pub const INSTRUMENT_PROPERTY_LABEL: &str = "instrument";
pub struct InstrumentPropertyIri;
impl PartialEq<&str> for InstrumentPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == INSTRUMENT_PROPERTY_IRI_HTTP || *other == INSTRUMENT_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InstrumentPropertyIri> for &str {
	fn eq(&self, other: &InstrumentPropertyIri) -> bool {
		*self == INSTRUMENT_PROPERTY_IRI_HTTP || *self == INSTRUMENT_PROPERTY_IRI_HTTPS
	}
}
pub struct InstrumentPropertyIriOrLabel;
impl PartialEq<&str> for InstrumentPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InstrumentPropertyIri || *other == INSTRUMENT_PROPERTY_LABEL
	}
}
impl PartialEq<InstrumentPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InstrumentPropertyIriOrLabel) -> bool {
		*self == InstrumentPropertyIri || *self == INSTRUMENT_PROPERTY_LABEL
	}
}
