/// <https://schema.org/publishingPrinciples>
pub const PUBLISHING_PRINCIPLES_PROPERTY_IRI_HTTP: &str = "http://schema.org/publishingPrinciples";
/// <https://schema.org/publishingPrinciples>
pub const PUBLISHING_PRINCIPLES_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/publishingPrinciples";
/// <https://schema.org/publishingPrinciples>
pub const PUBLISHING_PRINCIPLES_PROPERTY_LABEL: &str = "publishingPrinciples";
pub struct PublishingPrinciplesPropertyIri;
impl PartialEq<&str> for PublishingPrinciplesPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == PUBLISHING_PRINCIPLES_PROPERTY_IRI_HTTP
			|| *other == PUBLISHING_PRINCIPLES_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<PublishingPrinciplesPropertyIri> for &str {
	fn eq(&self, other: &PublishingPrinciplesPropertyIri) -> bool {
		*self == PUBLISHING_PRINCIPLES_PROPERTY_IRI_HTTP
			|| *self == PUBLISHING_PRINCIPLES_PROPERTY_IRI_HTTPS
	}
}
pub struct PublishingPrinciplesPropertyIriOrLabel;
impl PartialEq<&str> for PublishingPrinciplesPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == PublishingPrinciplesPropertyIri || *other == PUBLISHING_PRINCIPLES_PROPERTY_LABEL
	}
}
impl PartialEq<PublishingPrinciplesPropertyIriOrLabel> for &str {
	fn eq(&self, other: &PublishingPrinciplesPropertyIriOrLabel) -> bool {
		*self == PublishingPrinciplesPropertyIri || *self == PUBLISHING_PRINCIPLES_PROPERTY_LABEL
	}
}
