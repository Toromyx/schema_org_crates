/// <https://schema.org/arrivalBoatTerminal>
pub const ARRIVAL_BOAT_TERMINAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalBoatTerminal";
/// <https://schema.org/arrivalBoatTerminal>
pub const ARRIVAL_BOAT_TERMINAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalBoatTerminal";
/// <https://schema.org/arrivalBoatTerminal>
pub const ARRIVAL_BOAT_TERMINAL_PROPERTY_LABEL: &str = "arrivalBoatTerminal";
pub struct ArrivalBoatTerminalPropertyIri;
impl PartialEq<&str> for ArrivalBoatTerminalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_BOAT_TERMINAL_PROPERTY_IRI_HTTP
			|| *other == ARRIVAL_BOAT_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalBoatTerminalPropertyIri> for &str {
	fn eq(&self, other: &ArrivalBoatTerminalPropertyIri) -> bool {
		*self == ARRIVAL_BOAT_TERMINAL_PROPERTY_IRI_HTTP
			|| *self == ARRIVAL_BOAT_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalBoatTerminalPropertyIriOrLabel;
impl PartialEq<&str> for ArrivalBoatTerminalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalBoatTerminalPropertyIri || *other == ARRIVAL_BOAT_TERMINAL_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalBoatTerminalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalBoatTerminalPropertyIriOrLabel) -> bool {
		*self == ArrivalBoatTerminalPropertyIri || *self == ARRIVAL_BOAT_TERMINAL_PROPERTY_LABEL
	}
}
