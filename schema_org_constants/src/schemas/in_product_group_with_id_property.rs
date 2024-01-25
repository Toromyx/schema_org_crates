/// <https://schema.org/inProductGroupWithID>
pub const IN_PRODUCT_GROUP_WITH_ID_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/inProductGroupWithID";
/// <https://schema.org/inProductGroupWithID>
pub const IN_PRODUCT_GROUP_WITH_ID_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/inProductGroupWithID";
/// <https://schema.org/inProductGroupWithID>
pub const IN_PRODUCT_GROUP_WITH_ID_PROPERTY_LABEL: &str = "inProductGroupWithID";
pub struct InProductGroupWithIdPropertyIri;
impl PartialEq<&str> for InProductGroupWithIdPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == IN_PRODUCT_GROUP_WITH_ID_PROPERTY_IRI_HTTP
			|| *other == IN_PRODUCT_GROUP_WITH_ID_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<InProductGroupWithIdPropertyIri> for &str {
	fn eq(&self, other: &InProductGroupWithIdPropertyIri) -> bool {
		*self == IN_PRODUCT_GROUP_WITH_ID_PROPERTY_IRI_HTTP
			|| *self == IN_PRODUCT_GROUP_WITH_ID_PROPERTY_IRI_HTTPS
	}
}
pub struct InProductGroupWithIdPropertyIriOrLabel;
impl PartialEq<&str> for InProductGroupWithIdPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == InProductGroupWithIdPropertyIri
			|| *other == IN_PRODUCT_GROUP_WITH_ID_PROPERTY_LABEL
	}
}
impl PartialEq<InProductGroupWithIdPropertyIriOrLabel> for &str {
	fn eq(&self, other: &InProductGroupWithIdPropertyIriOrLabel) -> bool {
		*self == InProductGroupWithIdPropertyIri || *self == IN_PRODUCT_GROUP_WITH_ID_PROPERTY_LABEL
	}
}
