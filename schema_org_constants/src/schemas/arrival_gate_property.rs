/// <https://schema.org/arrivalGate>
pub const ARRIVAL_GATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalGate";
/// <https://schema.org/arrivalGate>
pub const ARRIVAL_GATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalGate";
/// <https://schema.org/arrivalGate>
pub const ARRIVAL_GATE_PROPERTY_LABEL: &str = "arrivalGate";
pub struct ArrivalGatePropertyIri;
impl PartialEq<&str> for ArrivalGatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_GATE_PROPERTY_IRI_HTTP || *other == ARRIVAL_GATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalGatePropertyIri> for &str {
	fn eq(&self, other: &ArrivalGatePropertyIri) -> bool {
		*self == ARRIVAL_GATE_PROPERTY_IRI_HTTP || *self == ARRIVAL_GATE_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalGatePropertyIriOrLabel;
impl PartialEq<&str> for ArrivalGatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalGatePropertyIri || *other == ARRIVAL_GATE_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalGatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalGatePropertyIriOrLabel) -> bool {
		*self == ArrivalGatePropertyIri || *self == ARRIVAL_GATE_PROPERTY_LABEL
	}
}
