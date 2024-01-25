/// <https://schema.org/offersPrescriptionByMail>
pub const OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_IRI_HTTP: &str =
	"http://schema.org/offersPrescriptionByMail";
/// <https://schema.org/offersPrescriptionByMail>
pub const OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_IRI_HTTPS: &str =
	"https://schema.org/offersPrescriptionByMail";
/// <https://schema.org/offersPrescriptionByMail>
pub const OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_LABEL: &str = "offersPrescriptionByMail";
pub struct OffersPrescriptionByMailPropertyIri;
impl PartialEq<&str> for OffersPrescriptionByMailPropertyIri {
	fn eq(&self, other: &&str) -> bool {
		*other == OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_IRI_HTTP
			|| *other == OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_IRI_HTTPS
	}
}
impl PartialEq<OffersPrescriptionByMailPropertyIri> for &str {
	fn eq(&self, other: &OffersPrescriptionByMailPropertyIri) -> bool {
		*self == OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_IRI_HTTP
			|| *self == OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_IRI_HTTPS
	}
}
pub struct OffersPrescriptionByMailPropertyIriOrLabel;
impl PartialEq<&str> for OffersPrescriptionByMailPropertyIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == OffersPrescriptionByMailPropertyIri
			|| *other == OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_LABEL
	}
}
impl PartialEq<OffersPrescriptionByMailPropertyIriOrLabel> for &str {
	fn eq(&self, other: &OffersPrescriptionByMailPropertyIriOrLabel) -> bool {
		*self == OffersPrescriptionByMailPropertyIri
			|| *self == OFFERS_PRESCRIPTION_BY_MAIL_PROPERTY_LABEL
	}
}
