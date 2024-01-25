/// <https://schema.org/departureGate>
pub const DEPARTURE_GATE_PROPERTY_IRI_HTTP: &str = "http://schema.org/departureGate";
/// <https://schema.org/departureGate>
pub const DEPARTURE_GATE_PROPERTY_IRI_HTTPS: &str = "https://schema.org/departureGate";
/// <https://schema.org/departureGate>
pub const DEPARTURE_GATE_PROPERTY_LABEL: &str = "departureGate";
pub struct DepartureGatePropertyIri;
impl PartialEq<&str> for DepartureGatePropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_GATE_PROPERTY_IRI_HTTP || *other == DEPARTURE_GATE_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartureGatePropertyIri> for &str {
	fn eq(&self, other: &DepartureGatePropertyIri) -> bool {
		*self == DEPARTURE_GATE_PROPERTY_IRI_HTTP || *self == DEPARTURE_GATE_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartureGatePropertyIriOrLabel;
impl PartialEq<&str> for DepartureGatePropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartureGatePropertyIri || *other == DEPARTURE_GATE_PROPERTY_LABEL
	}
}
impl PartialEq<DepartureGatePropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartureGatePropertyIriOrLabel) -> bool {
		*self == DepartureGatePropertyIri || *self == DEPARTURE_GATE_PROPERTY_LABEL
	}
}
