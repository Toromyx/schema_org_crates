/// <https://schema.org/UserBlocks>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_BLOCKS_IRI_HTTP: &str = "http://schema.org/UserBlocks";
/// <https://schema.org/UserBlocks>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_BLOCKS_IRI_HTTPS: &str = "https://schema.org/UserBlocks";
/// <https://schema.org/UserBlocks>
#[deprecated = "This schema is superseded by <https://schema.org/InteractionCounter>."]
pub const USER_BLOCKS_LABEL: &str = "UserBlocks";
pub struct UserBlocksIri;
impl PartialEq<&str> for UserBlocksIri {
	fn eq(&self, other: &&str) -> bool {
		*other == USER_BLOCKS_IRI_HTTP || *other == USER_BLOCKS_IRI_HTTPS
	}
}
impl PartialEq<UserBlocksIri> for &str {
	fn eq(&self, other: &UserBlocksIri) -> bool {
		*self == USER_BLOCKS_IRI_HTTP || *self == USER_BLOCKS_IRI_HTTPS
	}
}
pub struct UserBlocksIriOrLabel;
impl PartialEq<&str> for UserBlocksIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UserBlocksIri || *other == USER_BLOCKS_LABEL
	}
}
impl PartialEq<UserBlocksIriOrLabel> for &str {
	fn eq(&self, other: &UserBlocksIriOrLabel) -> bool {
		*self == UserBlocksIri || *self == USER_BLOCKS_LABEL
	}
}
