/// <https://schema.org/Playground>
pub const PLAYGROUND_IRI_HTTP: &str = "http://schema.org/Playground";
/// <https://schema.org/Playground>
pub const PLAYGROUND_IRI_HTTPS: &str = "https://schema.org/Playground";
/// <https://schema.org/Playground>
pub const PLAYGROUND_LABEL: &str = "Playground";
pub struct PlaygroundIri;
impl PartialEq<&str> for PlaygroundIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PLAYGROUND_IRI_HTTP || *other == PLAYGROUND_IRI_HTTPS
	}
}
impl PartialEq<PlaygroundIri> for &str {
	fn eq(&self, other: &PlaygroundIri) -> bool {
		*self == PLAYGROUND_IRI_HTTP || *self == PLAYGROUND_IRI_HTTPS
	}
}
pub struct PlaygroundIriOrLabel;
impl PartialEq<&str> for PlaygroundIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PlaygroundIri || *other == PLAYGROUND_LABEL
	}
}
impl PartialEq<PlaygroundIriOrLabel> for &str {
	fn eq(&self, other: &PlaygroundIriOrLabel) -> bool {
		*self == PlaygroundIri || *self == PLAYGROUND_LABEL
	}
}
