/// <https://schema.org/inPlaylist>
pub const IN_PLAYLIST_PROPERTY_IRI_HTTP: &str = "http://schema.org/inPlaylist";
/// <https://schema.org/inPlaylist>
pub const IN_PLAYLIST_PROPERTY_IRI_HTTPS: &str = "https://schema.org/inPlaylist";
/// <https://schema.org/inPlaylist>
pub const IN_PLAYLIST_PROPERTY_LABEL: &str = "inPlaylist";
pub struct InPlaylistPropertyIri;
impl PartialEq<&str> for InPlaylistPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_PLAYLIST_PROPERTY_IRI_HTTP || *other == IN_PLAYLIST_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InPlaylistPropertyIri> for &str {
	fn eq(&self, other: &InPlaylistPropertyIri) -> bool {
		*self == IN_PLAYLIST_PROPERTY_IRI_HTTP || *self == IN_PLAYLIST_PROPERTY_IRI_HTTPS
	}
}
pub struct InPlaylistPropertyIriOrLabel;
impl PartialEq<&str> for InPlaylistPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InPlaylistPropertyIri || *other == IN_PLAYLIST_PROPERTY_LABEL
	}
}
impl PartialEq<InPlaylistPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InPlaylistPropertyIriOrLabel) -> bool {
		*self == InPlaylistPropertyIri || *self == IN_PLAYLIST_PROPERTY_LABEL
	}
}
