/// <https://schema.org/SinglePlayer>
pub const SINGLE_PLAYER_IRI_HTTP: &str = "http://schema.org/SinglePlayer";
/// <https://schema.org/SinglePlayer>
pub const SINGLE_PLAYER_IRI_HTTPS: &str = "https://schema.org/SinglePlayer";
/// <https://schema.org/SinglePlayer>
pub const SINGLE_PLAYER_LABEL: &str = "SinglePlayer";
pub struct SinglePlayerIri;
impl PartialEq<&str> for SinglePlayerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == SINGLE_PLAYER_IRI_HTTP || *other == SINGLE_PLAYER_IRI_HTTPS
	}
}
impl PartialEq<SinglePlayerIri> for &str {
	fn eq(&self, other: &SinglePlayerIri) -> bool {
		*self == SINGLE_PLAYER_IRI_HTTP || *self == SINGLE_PLAYER_IRI_HTTPS
	}
}
pub struct SinglePlayerIriOrLabel;
impl PartialEq<&str> for SinglePlayerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == SinglePlayerIri || *other == SINGLE_PLAYER_LABEL
	}
}
impl PartialEq<SinglePlayerIriOrLabel> for &str {
	fn eq(&self, other: &SinglePlayerIriOrLabel) -> bool {
		*self == SinglePlayerIri || *self == SINGLE_PLAYER_LABEL
	}
}
