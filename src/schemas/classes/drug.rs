use super::*;
/// <https://schema.org/Drug>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Drug {
    #[cfg(any(
        any(
            feature = "active-ingredient-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "activeIngredient"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#active_ingredient: Vec<ActiveIngredientProperty>,
    #[cfg(any(
        any(
            feature = "additional-property-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "additionalProperty"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#additional_property: Vec<AdditionalPropertyProperty>,
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "additionalType"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "administration-route-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "administrationRoute"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#administration_route: Vec<AdministrationRouteProperty>,
    #[cfg(any(
        any(
            feature = "aggregate-rating-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "aggregateRating"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#aggregate_rating: Vec<AggregateRatingProperty>,
    #[cfg(any(
        any(
            feature = "alcohol-warning-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "alcoholWarning"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#alcohol_warning: Vec<AlcoholWarningProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "alternateName"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(feature = "asin-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "asin"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#asin: Vec<AsinProperty>,
    #[cfg(any(
        any(
            feature = "audience-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "audience"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#audience: Vec<AudienceProperty>,
    #[cfg(any(
        any(
            feature = "available-strength-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "availableStrength"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#available_strength: Vec<AvailableStrengthProperty>,
    #[cfg(any(
        any(feature = "award-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "award"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#award: Vec<AwardProperty>,
    #[cfg(any(
        any(feature = "awards-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "awards"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#awards: Vec<AwardsProperty>,
    #[cfg(any(
        any(feature = "brand-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "brand"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#brand: Vec<BrandProperty>,
    #[cfg(any(
        any(
            feature = "breastfeeding-warning-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "breastfeedingWarning"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#breastfeeding_warning: Vec<BreastfeedingWarningProperty>,
    #[cfg(any(
        any(
            feature = "category-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "category"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#category: Vec<CategoryProperty>,
    #[cfg(any(
        any(
            feature = "clincal-pharmacology-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "clincalPharmacology"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#clincal_pharmacology: Vec<ClincalPharmacologyProperty>,
    #[cfg(any(
        any(
            feature = "clinical-pharmacology-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "clinicalPharmacology"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#clinical_pharmacology: Vec<ClinicalPharmacologyProperty>,
    #[cfg(any(
        any(
            feature = "code-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "code"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#code: Vec<CodeProperty>,
    #[cfg(any(
        any(feature = "color-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "color"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#color: Vec<ColorProperty>,
    #[cfg(any(
        any(
            feature = "country-of-assembly-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "countryOfAssembly"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#country_of_assembly: Vec<CountryOfAssemblyProperty>,
    #[cfg(any(
        any(
            feature = "country-of-last-processing-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "countryOfLastProcessing"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#country_of_last_processing: Vec<CountryOfLastProcessingProperty>,
    #[cfg(any(
        any(
            feature = "country-of-origin-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "countryOfOrigin"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#country_of_origin: Vec<CountryOfOriginProperty>,
    #[cfg(any(
        any(feature = "depth-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "depth"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#depth: Vec<DepthProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "description"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "disambiguatingDescription")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "dosage-form-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "dosageForm"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#dosage_form: Vec<DosageFormProperty>,
    #[cfg(any(
        any(
            feature = "dose-schedule-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "doseSchedule"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#dose_schedule: Vec<DoseScheduleProperty>,
    #[cfg(any(
        any(
            feature = "drug-class-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "drugClass"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#drug_class: Vec<DrugClassProperty>,
    #[cfg(any(
        any(
            feature = "drug-unit-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "drugUnit"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#drug_unit: Vec<DrugUnitProperty>,
    #[cfg(any(
        any(
            feature = "food-warning-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "foodWarning"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#food_warning: Vec<FoodWarningProperty>,
    #[cfg(any(
        any(
            feature = "funding-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "funding"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#funding: Vec<FundingProperty>,
    #[cfg(any(
        any(feature = "gtin-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin: Vec<GtinProperty>,
    #[cfg(any(
        any(
            feature = "gtin-12-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin12"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_12: Vec<Gtin12Property>,
    #[cfg(any(
        any(
            feature = "gtin-13-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin13"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_13: Vec<Gtin13Property>,
    #[cfg(any(
        any(
            feature = "gtin-14-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin14"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_14: Vec<Gtin14Property>,
    #[cfg(any(
        any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "gtin8"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#gtin_8: Vec<Gtin8Property>,
    #[cfg(any(
        any(
            feature = "guideline-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "guideline"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#guideline: Vec<GuidelineProperty>,
    #[cfg(any(
        any(
            feature = "has-adult-consideration-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasAdultConsideration"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
    #[cfg(any(
        any(
            feature = "has-energy-consumption-details-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "hasEnergyConsumptionDetails")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_energy_consumption_details: Vec<HasEnergyConsumptionDetailsProperty>,
    #[cfg(any(
        any(
            feature = "has-measurement-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasMeasurement"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_measurement: Vec<HasMeasurementProperty>,
    #[cfg(any(
        any(
            feature = "has-merchant-return-policy-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasMerchantReturnPolicy"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
    #[cfg(any(
        any(
            feature = "has-product-return-policy-property-schema",
            feature = "attic-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasProductReturnPolicy"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
    #[cfg(any(
        any(feature = "height-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "height"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#height: Vec<HeightProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "identifier"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "image"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "in-product-group-with-id-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "inProductGroupWithID"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#in_product_group_with_id: Vec<InProductGroupWithIdProperty>,
    #[cfg(any(
        any(
            feature = "included-in-health-insurance-plan-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "includedInHealthInsurancePlan")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#included_in_health_insurance_plan: Vec<IncludedInHealthInsurancePlanProperty>,
    #[cfg(any(
        any(
            feature = "interacting-drug-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "interactingDrug"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#interacting_drug: Vec<InteractingDrugProperty>,
    #[cfg(any(
        any(
            feature = "is-accessory-or-spare-part-for-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "isAccessoryOrSparePartFor")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_accessory_or_spare_part_for: Vec<IsAccessoryOrSparePartForProperty>,
    #[cfg(any(
        any(
            feature = "is-available-generically-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isAvailableGenerically"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_available_generically: Vec<IsAvailableGenericallyProperty>,
    #[cfg(any(
        any(
            feature = "is-consumable-for-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isConsumableFor"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_consumable_for: Vec<IsConsumableForProperty>,
    #[cfg(any(
        any(
            feature = "is-family-friendly-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isFamilyFriendly"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
    #[cfg(any(
        any(
            feature = "is-proprietary-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isProprietary"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_proprietary: Vec<IsProprietaryProperty>,
    #[cfg(any(
        any(
            feature = "is-related-to-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isRelatedTo"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_related_to: Vec<IsRelatedToProperty>,
    #[cfg(any(
        any(
            feature = "is-similar-to-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isSimilarTo"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_similar_to: Vec<IsSimilarToProperty>,
    #[cfg(any(
        any(
            feature = "is-variant-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "isVariantOf"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#is_variant_of: Vec<IsVariantOfProperty>,
    #[cfg(any(
        any(
            feature = "item-condition-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "itemCondition"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#item_condition: Vec<ItemConditionProperty>,
    #[cfg(any(
        any(
            feature = "keywords-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "keywords"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#keywords: Vec<KeywordsProperty>,
    #[cfg(any(
        any(
            feature = "label-details-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "labelDetails"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#label_details: Vec<LabelDetailsProperty>,
    #[cfg(any(
        any(
            feature = "legal-status-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "legalStatus"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#legal_status: Vec<LegalStatusProperty>,
    #[cfg(any(
        any(feature = "logo-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "logo"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#logo: Vec<LogoProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "manufacturer-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "manufacturer"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#manufacturer: Vec<ManufacturerProperty>,
    #[cfg(any(
        any(
            feature = "material-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "material"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#material: Vec<MaterialProperty>,
    #[cfg(any(
        any(
            feature = "maximum-intake-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "maximumIntake"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#maximum_intake: Vec<MaximumIntakeProperty>,
    #[cfg(any(
        any(
            feature = "mechanism-of-action-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mechanismOfAction"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#mechanism_of_action: Vec<MechanismOfActionProperty>,
    #[cfg(any(
        any(
            feature = "medicine-system-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "medicineSystem"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#medicine_system: Vec<MedicineSystemProperty>,
    #[cfg(any(
        any(
            feature = "mobile-url-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mobileUrl"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#mobile_url: Vec<MobileUrlProperty>,
    #[cfg(any(
        any(feature = "model-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "model"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#model: Vec<ModelProperty>,
    #[cfg(any(
        any(feature = "mpn-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mpn"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#mpn: Vec<MpnProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "name"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "negative-notes-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "negativeNotes"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#negative_notes: Vec<NegativeNotesProperty>,
    #[cfg(any(
        any(
            feature = "non-proprietary-name-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "nonProprietaryName"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#non_proprietary_name: Vec<NonProprietaryNameProperty>,
    #[cfg(any(
        any(feature = "nsn-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "nsn"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#nsn: Vec<NsnProperty>,
    #[cfg(any(
        any(feature = "offers-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "offers"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        any(
            feature = "overdosage-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "overdosage"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#overdosage: Vec<OverdosageProperty>,
    #[cfg(any(
        any(
            feature = "pattern-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "pattern"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#pattern: Vec<PatternProperty>,
    #[cfg(any(
        any(
            feature = "positive-notes-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "positiveNotes"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#positive_notes: Vec<PositiveNotesProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "potentialAction"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "pregnancy-category-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "pregnancyCategory"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#pregnancy_category: Vec<PregnancyCategoryProperty>,
    #[cfg(any(
        any(
            feature = "pregnancy-warning-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "pregnancyWarning"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#pregnancy_warning: Vec<PregnancyWarningProperty>,
    #[cfg(any(
        any(
            feature = "prescribing-info-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "prescribingInfo"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#prescribing_info: Vec<PrescribingInfoProperty>,
    #[cfg(any(
        any(
            feature = "prescription-status-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "prescriptionStatus"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#prescription_status: Vec<PrescriptionStatusProperty>,
    #[cfg(any(
        any(
            feature = "product-id-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "productID"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#product_id: Vec<ProductIdProperty>,
    #[cfg(any(
        any(
            feature = "production-date-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "productionDate"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#production_date: Vec<ProductionDateProperty>,
    #[cfg(any(
        any(
            feature = "proprietary-name-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "proprietaryName"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#proprietary_name: Vec<ProprietaryNameProperty>,
    #[cfg(any(
        any(
            feature = "purchase-date-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "purchaseDate"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#purchase_date: Vec<PurchaseDateProperty>,
    #[cfg(any(
        any(
            feature = "recognizing-authority-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "recognizingAuthority"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#recognizing_authority: Vec<RecognizingAuthorityProperty>,
    #[cfg(any(
        any(
            feature = "related-drug-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "relatedDrug"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#related_drug: Vec<RelatedDrugProperty>,
    #[cfg(any(
        any(
            feature = "release-date-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "releaseDate"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#release_date: Vec<ReleaseDateProperty>,
    #[cfg(any(
        any(
            feature = "relevant-specialty-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "relevantSpecialty"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#relevant_specialty: Vec<RelevantSpecialtyProperty>,
    #[cfg(any(
        any(feature = "review-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "review"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#review: Vec<ReviewProperty>,
    #[cfg(any(
        any(
            feature = "reviews-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "reviews"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#reviews: Vec<ReviewsProperty>,
    #[cfg(any(
        any(feature = "rxcui-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "rxcui"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#rxcui: Vec<RxcuiProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "sameAs"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(feature = "size-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "size"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#size: Vec<SizeProperty>,
    #[cfg(any(
        any(feature = "sku-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "sku"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#sku: Vec<SkuProperty>,
    #[cfg(any(
        any(feature = "slogan-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "slogan"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#slogan: Vec<SloganProperty>,
    #[cfg(any(
        any(
            feature = "study-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "study"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#study: Vec<StudyProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "subjectOf"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "url"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        any(
            feature = "warning-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "warning"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#warning: Vec<WarningProperty>,
    #[cfg(any(
        any(feature = "weight-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "weight"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#weight: Vec<WeightProperty>,
    #[cfg(any(
        any(feature = "width-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "width"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#width: Vec<WidthProperty>,
}
