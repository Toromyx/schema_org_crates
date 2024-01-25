/// <https://schema.org/PawnShop>
pub const PAWN_SHOP_IRI_HTTP: &str = "http://schema.org/PawnShop";
/// <https://schema.org/PawnShop>
pub const PAWN_SHOP_IRI_HTTPS: &str = "https://schema.org/PawnShop";
/// <https://schema.org/PawnShop>
pub const PAWN_SHOP_LABEL: &str = "PawnShop";
pub struct PawnShopIri;
impl PartialEq<&str> for PawnShopIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PAWN_SHOP_IRI_HTTP || *other == PAWN_SHOP_IRI_HTTPS
	}
}
impl PartialEq<PawnShopIri> for &str {
	fn eq(&self, other: &PawnShopIri) -> bool {
		*self == PAWN_SHOP_IRI_HTTP || *self == PAWN_SHOP_IRI_HTTPS
	}
}
pub struct PawnShopIriOrLabel;
impl PartialEq<&str> for PawnShopIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PawnShopIri || *other == PAWN_SHOP_LABEL
	}
}
impl PartialEq<PawnShopIriOrLabel> for &str {
	fn eq(&self, other: &PawnShopIriOrLabel) -> bool {
		*self == PawnShopIri || *self == PAWN_SHOP_LABEL
	}
}
