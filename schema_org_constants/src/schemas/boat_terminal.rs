/// <https://schema.org/BoatTerminal>
pub const BOAT_TERMINAL_IRI_HTTP: &str = "http://schema.org/BoatTerminal";
/// <https://schema.org/BoatTerminal>
pub const BOAT_TERMINAL_IRI_HTTPS: &str = "https://schema.org/BoatTerminal";
/// <https://schema.org/BoatTerminal>
pub const BOAT_TERMINAL_LABEL: &str = "BoatTerminal";
pub struct BoatTerminalIri;
impl PartialEq<&str> for BoatTerminalIri {
	fn eq(&self, other: &&str) -> bool {
		*other == BOAT_TERMINAL_IRI_HTTP || *other == BOAT_TERMINAL_IRI_HTTPS
	}
}
impl PartialEq<BoatTerminalIri> for &str {
	fn eq(&self, other: &BoatTerminalIri) -> bool {
		*self == BOAT_TERMINAL_IRI_HTTP || *self == BOAT_TERMINAL_IRI_HTTPS
	}
}
pub struct BoatTerminalIriOrLabel;
impl PartialEq<&str> for BoatTerminalIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == BoatTerminalIri || *other == BOAT_TERMINAL_LABEL
	}
}
impl PartialEq<BoatTerminalIriOrLabel> for &str {
	fn eq(&self, other: &BoatTerminalIriOrLabel) -> bool {
		*self == BoatTerminalIri || *self == BOAT_TERMINAL_LABEL
	}
}
