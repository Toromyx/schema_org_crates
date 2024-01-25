/// <https://schema.org/departureTerminal>
pub const DEPARTURE_TERMINAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/departureTerminal";
/// <https://schema.org/departureTerminal>
pub const DEPARTURE_TERMINAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/departureTerminal";
/// <https://schema.org/departureTerminal>
pub const DEPARTURE_TERMINAL_PROPERTY_LABEL: &str = "departureTerminal";
pub struct DepartureTerminalPropertyIri;
impl PartialEq<&str> for DepartureTerminalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == DEPARTURE_TERMINAL_PROPERTY_IRI_HTTP
			|| *other == DEPARTURE_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<DepartureTerminalPropertyIri> for &str {
	fn eq(&self, other: &DepartureTerminalPropertyIri) -> bool {
		*self == DEPARTURE_TERMINAL_PROPERTY_IRI_HTTP
			|| *self == DEPARTURE_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
pub struct DepartureTerminalPropertyIriOrLabel;
impl PartialEq<&str> for DepartureTerminalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == DepartureTerminalPropertyIri || *other == DEPARTURE_TERMINAL_PROPERTY_LABEL
	}
}
impl PartialEq<DepartureTerminalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &DepartureTerminalPropertyIriOrLabel) -> bool {
		*self == DepartureTerminalPropertyIri || *self == DEPARTURE_TERMINAL_PROPERTY_LABEL
	}
}
