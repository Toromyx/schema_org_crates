/// <https://schema.org/UnincorporatedAssociationCharity>
pub const UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTP: &str =
	"http://schema.org/UnincorporatedAssociationCharity";
/// <https://schema.org/UnincorporatedAssociationCharity>
pub const UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTPS: &str =
	"https://schema.org/UnincorporatedAssociationCharity";
/// <https://schema.org/UnincorporatedAssociationCharity>
pub const UNINCORPORATED_ASSOCIATION_CHARITY_LABEL: &str = "UnincorporatedAssociationCharity";
pub struct UnincorporatedAssociationCharityIri;
impl PartialEq<&str> for UnincorporatedAssociationCharityIri {
	fn eq(&self, other: &&str) -> bool {
		*other == UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTP
			|| *other == UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTPS
	}
}
impl PartialEq<UnincorporatedAssociationCharityIri> for &str {
	fn eq(&self, other: &UnincorporatedAssociationCharityIri) -> bool {
		*self == UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTP
			|| *self == UNINCORPORATED_ASSOCIATION_CHARITY_IRI_HTTPS
	}
}
pub struct UnincorporatedAssociationCharityIriOrLabel;
impl PartialEq<&str> for UnincorporatedAssociationCharityIriOrLabel {
	fn eq(&self, other: &&str) -> bool {
		*other == UnincorporatedAssociationCharityIri
			|| *other == UNINCORPORATED_ASSOCIATION_CHARITY_LABEL
	}
}
impl PartialEq<UnincorporatedAssociationCharityIriOrLabel> for &str {
	fn eq(&self, other: &UnincorporatedAssociationCharityIriOrLabel) -> bool {
		*self == UnincorporatedAssociationCharityIri
			|| *self == UNINCORPORATED_ASSOCIATION_CHARITY_LABEL
	}
}
