/// <https://schema.org/MusicPlaylist>
pub const MUSIC_PLAYLIST_IRI_HTTP: &str = "http://schema.org/MusicPlaylist";
/// <https://schema.org/MusicPlaylist>
pub const MUSIC_PLAYLIST_IRI_HTTPS: &str = "https://schema.org/MusicPlaylist";
/// <https://schema.org/MusicPlaylist>
pub const MUSIC_PLAYLIST_LABEL: &str = "MusicPlaylist";
pub struct MusicPlaylistIri;
impl PartialEq<&str> for MusicPlaylistIri {
	fn eq(&self, other: &&str) -> bool {
		*other == MUSIC_PLAYLIST_IRI_HTTP || *other == MUSIC_PLAYLIST_IRI_HTTPS
	}
}
impl PartialEq<MusicPlaylistIri> for &str {
	fn eq(&self, other: &MusicPlaylistIri) -> bool {
		*self == MUSIC_PLAYLIST_IRI_HTTP || *self == MUSIC_PLAYLIST_IRI_HTTPS
	}
}
pub struct MusicPlaylistIriOrLabel;
impl PartialEq<&str> for MusicPlaylistIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == MusicPlaylistIri || *other == MUSIC_PLAYLIST_LABEL
	}
}
impl PartialEq<MusicPlaylistIriOrLabel> for &str {
	fn eq(&self, other: &MusicPlaylistIriOrLabel) -> bool {
		*self == MusicPlaylistIri || *self == MUSIC_PLAYLIST_LABEL
	}
}
