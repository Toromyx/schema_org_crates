/// <https://schema.org/arrivalTerminal>
pub const ARRIVAL_TERMINAL_PROPERTY_IRI_HTTP: &str = "http://schema.org/arrivalTerminal";
/// <https://schema.org/arrivalTerminal>
pub const ARRIVAL_TERMINAL_PROPERTY_IRI_HTTPS: &str = "https://schema.org/arrivalTerminal";
/// <https://schema.org/arrivalTerminal>
pub const ARRIVAL_TERMINAL_PROPERTY_LABEL: &str = "arrivalTerminal";
pub struct ArrivalTerminalPropertyIri;
impl PartialEq<&str> for ArrivalTerminalPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == ARRIVAL_TERMINAL_PROPERTY_IRI_HTTP
			|| *other == ARRIVAL_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<ArrivalTerminalPropertyIri> for &str {
	fn eq(&self, other: &ArrivalTerminalPropertyIri) -> bool {
		*self == ARRIVAL_TERMINAL_PROPERTY_IRI_HTTP || *self == ARRIVAL_TERMINAL_PROPERTY_IRI_HTTPS
	}
}
pub struct ArrivalTerminalPropertyIriOrLabel;
impl PartialEq<&str> for ArrivalTerminalPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == ArrivalTerminalPropertyIri || *other == ARRIVAL_TERMINAL_PROPERTY_LABEL
	}
}
impl PartialEq<ArrivalTerminalPropertyIriOrLabel> for &str {
	fn eq(&self, other: &ArrivalTerminalPropertyIriOrLabel) -> bool {
		*self == ArrivalTerminalPropertyIri || *self == ARRIVAL_TERMINAL_PROPERTY_LABEL
	}
}
