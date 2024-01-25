/// <https://schema.org/MultiPlayer>
pub const MULTI_PLAYER_IRI_HTTP: &str = "http://schema.org/MultiPlayer";
/// <https://schema.org/MultiPlayer>
pub const MULTI_PLAYER_IRI_HTTPS: &str = "https://schema.org/MultiPlayer";
/// <https://schema.org/MultiPlayer>
pub const MULTI_PLAYER_LABEL: &str = "MultiPlayer";
pub struct MultiPlayerIri;
impl PartialEq<&str> for MultiPlayerIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MULTI_PLAYER_IRI_HTTP || *other == MULTI_PLAYER_IRI_HTTPS
	}
}
impl PartialEq<MultiPlayerIri> for &str {
	fn eq(&self, other: &MultiPlayerIri) -> bool {
		*self == MULTI_PLAYER_IRI_HTTP || *self == MULTI_PLAYER_IRI_HTTPS
	}
}
pub struct MultiPlayerIriOrLabel;
impl PartialEq<&str> for MultiPlayerIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MultiPlayerIri || *other == MULTI_PLAYER_LABEL
	}
}
impl PartialEq<MultiPlayerIriOrLabel> for &str {
	fn eq(&self, other: &MultiPlayerIriOrLabel) -> bool {
		*self == MultiPlayerIri || *self == MULTI_PLAYER_LABEL
	}
}
