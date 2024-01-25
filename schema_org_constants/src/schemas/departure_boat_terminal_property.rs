/// <https://schema.org/departureBoatTerminal>
pub const DEPARTURE_BOAT_TERMINAL_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/departureBoatTerminal";
/// <https://schema.org/departureBoatTerminal>
pub const DEPARTURE_BOAT_TERMINAL_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/departureBoatTerminal";
/// <https://schema.org/departureBoatTerminal>
pub const DEPARTURE_BOAT_TERMINAL_PROPERTY_LABEL: &str = "departureBoatTerminal";
pub struct DepartureBoatTerminalPropertyIri;
impl PartialEq<&str> for DepartureBoatTerminalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_BOAT_TERMINAL_PROPERTY_IRI_HTTP
			|| *other == DEPARTURE_BOAT_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartureBoatTerminalPropertyIri> for &str {
	fn eq(&self, other: &DepartureBoatTerminalPropertyIri) -> bool {
		*self == DEPARTURE_BOAT_TERMINAL_PROPERTY_IRI_HTTP
			|| *self == DEPARTURE_BOAT_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartureBoatTerminalPropertyIriOrLabel;
impl PartialEq<&str> for DepartureBoatTerminalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartureBoatTerminalPropertyIri
			|| *other == DEPARTURE_BOAT_TERMINAL_PROPERTY_LABEL
	}
}
impl PartialEq<DepartureBoatTerminalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartureBoatTerminalPropertyIriOrLabel) -> bool {
		*self == DepartureBoatTerminalPropertyIri || *self == DEPARTURE_BOAT_TERMINAL_PROPERTY_LABEL
	}
}
