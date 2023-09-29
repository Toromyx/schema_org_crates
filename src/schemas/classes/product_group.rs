use super::*;
/// <https://schema.org/ProductGroup>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProductGroup {
    #[cfg(any(
        any(
            feature = "additional-property-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalProperty"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/additionalProperty")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/additionalProperty")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#additional_property: Vec<AdditionalPropertyProperty>,
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "aggregate-rating-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#aggregate_rating: Vec<AggregateRatingProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(feature = "asin-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "asin"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/asin"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/asin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#asin: Vec<AsinProperty>,
    #[cfg(any(
        any(
            feature = "audience-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "audience"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/audience"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/audience"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#audience: Vec<AudienceProperty>,
    #[cfg(any(
        any(feature = "award-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "award"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/award"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/award"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#award: Vec<AwardProperty>,
    #[cfg(any(
        any(feature = "awards-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "awards"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/awards"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/awards"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#awards: Vec<AwardsProperty>,
    #[cfg(any(
        any(feature = "brand-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "brand"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/brand"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/brand"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#brand: Vec<BrandProperty>,
    #[cfg(any(
        any(
            feature = "category-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "category"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/category"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/category"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#category: Vec<CategoryProperty>,
    #[cfg(any(
        any(feature = "color-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "color"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/color"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/color"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#color: Vec<ColorProperty>,
    #[cfg(any(
        any(
            feature = "country-of-assembly-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfAssembly"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/countryOfAssembly")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/countryOfAssembly")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#country_of_assembly: Vec<CountryOfAssemblyProperty>,
    #[cfg(any(
        any(
            feature = "country-of-last-processing-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfLastProcessing"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/countryOfLastProcessing")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/countryOfLastProcessing")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#country_of_last_processing: Vec<CountryOfLastProcessingProperty>,
    #[cfg(any(
        any(
            feature = "country-of-origin-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#country_of_origin: Vec<CountryOfOriginProperty>,
    #[cfg(any(
        any(feature = "depth-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "depth"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/depth"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/depth"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#depth: Vec<DepthProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "funding-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "funding"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/funding"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/funding"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#funding: Vec<FundingProperty>,
    #[cfg(any(
        any(feature = "gtin-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/gtin"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/gtin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#gtin: Vec<GtinProperty>,
    #[cfg(any(
        any(
            feature = "gtin-12-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin12"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/gtin12"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/gtin12"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#gtin_12: Vec<Gtin12Property>,
    #[cfg(any(
        any(
            feature = "gtin-13-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin13"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/gtin13"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/gtin13"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#gtin_13: Vec<Gtin13Property>,
    #[cfg(any(
        any(
            feature = "gtin-14-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin14"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/gtin14"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/gtin14"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#gtin_14: Vec<Gtin14Property>,
    #[cfg(any(
        any(feature = "gtin-8-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gtin8"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/gtin8"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/gtin8"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#gtin_8: Vec<Gtin8Property>,
    #[cfg(any(
        any(
            feature = "has-adult-consideration-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasAdultConsideration"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasAdultConsideration")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasAdultConsideration")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
    #[cfg(any(
        any(
            feature = "has-energy-consumption-details-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasEnergyConsumptionDetails"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasEnergyConsumptionDetails")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasEnergyConsumptionDetails")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_energy_consumption_details: Vec<HasEnergyConsumptionDetailsProperty>,
    #[cfg(any(
        any(
            feature = "has-measurement-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasMeasurement"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/hasMeasurement"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/hasMeasurement"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_measurement: Vec<HasMeasurementProperty>,
    #[cfg(any(
        any(
            feature = "has-merchant-return-policy-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasMerchantReturnPolicy"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasMerchantReturnPolicy")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasMerchantReturnPolicy")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
    #[cfg(any(
        any(
            feature = "has-product-return-policy-property-schema",
            feature = "attic-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasProductReturnPolicy"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasProductReturnPolicy")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasProductReturnPolicy")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
    #[cfg(any(
        any(
            feature = "has-variant-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasVariant"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/hasVariant"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/hasVariant"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_variant: Vec<HasVariantProperty>,
    #[cfg(any(
        any(feature = "height-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "height"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/height"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/height"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#height: Vec<HeightProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "in-product-group-with-id-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "inProductGroupWithID"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/inProductGroupWithID")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/inProductGroupWithID")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#in_product_group_with_id: Vec<InProductGroupWithIdProperty>,
    #[cfg(any(
        any(
            feature = "is-accessory-or-spare-part-for-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isAccessoryOrSparePartFor"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isAccessoryOrSparePartFor")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/isAccessoryOrSparePartFor")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_accessory_or_spare_part_for: Vec<IsAccessoryOrSparePartForProperty>,
    #[cfg(any(
        any(
            feature = "is-consumable-for-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isConsumableFor"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isConsumableFor"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isConsumableFor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_consumable_for: Vec<IsConsumableForProperty>,
    #[cfg(any(
        any(
            feature = "is-family-friendly-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isFamilyFriendly"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isFamilyFriendly")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isFamilyFriendly"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
    #[cfg(any(
        any(
            feature = "is-related-to-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isRelatedTo"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isRelatedTo"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isRelatedTo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_related_to: Vec<IsRelatedToProperty>,
    #[cfg(any(
        any(
            feature = "is-similar-to-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isSimilarTo"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isSimilarTo"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isSimilarTo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_similar_to: Vec<IsSimilarToProperty>,
    #[cfg(any(
        any(
            feature = "is-variant-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isVariantOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isVariantOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isVariantOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_variant_of: Vec<IsVariantOfProperty>,
    #[cfg(any(
        any(
            feature = "item-condition-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "itemCondition"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/itemCondition"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/itemCondition"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#item_condition: Vec<ItemConditionProperty>,
    #[cfg(any(
        any(
            feature = "keywords-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "keywords"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/keywords"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/keywords"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#keywords: Vec<KeywordsProperty>,
    #[cfg(any(
        any(feature = "logo-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "logo"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/logo"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/logo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#logo: Vec<LogoProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/mainEntityOfPage")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "manufacturer-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "manufacturer"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/manufacturer"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/manufacturer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#manufacturer: Vec<ManufacturerProperty>,
    #[cfg(any(
        any(
            feature = "material-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/material"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/material"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#material: Vec<MaterialProperty>,
    #[cfg(any(
        any(
            feature = "mobile-url-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mobileUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/mobileUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mobileUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#mobile_url: Vec<MobileUrlProperty>,
    #[cfg(any(
        any(feature = "model-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "model"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/model"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/model"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#model: Vec<ModelProperty>,
    #[cfg(any(
        any(feature = "mpn-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mpn"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/mpn"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mpn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#mpn: Vec<MpnProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "negative-notes-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "negativeNotes"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/negativeNotes"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/negativeNotes"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#negative_notes: Vec<NegativeNotesProperty>,
    #[cfg(any(
        any(feature = "nsn-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "nsn"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/nsn"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/nsn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#nsn: Vec<NsnProperty>,
    #[cfg(any(
        any(feature = "offers-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "offers"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/offers"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/offers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        any(
            feature = "pattern-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "pattern"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/pattern"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/pattern"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#pattern: Vec<PatternProperty>,
    #[cfg(any(
        any(
            feature = "positive-notes-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "positiveNotes"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/positiveNotes"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/positiveNotes"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#positive_notes: Vec<PositiveNotesProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "product-group-id-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "productGroupID"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/productGroupID"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/productGroupID"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#product_group_id: Vec<ProductGroupIdProperty>,
    #[cfg(any(
        any(
            feature = "product-id-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "productID"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/productID"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/productID"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#product_id: Vec<ProductIdProperty>,
    #[cfg(any(
        any(
            feature = "production-date-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "productionDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/productionDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/productionDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#production_date: Vec<ProductionDateProperty>,
    #[cfg(any(
        any(
            feature = "purchase-date-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "purchaseDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/purchaseDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/purchaseDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#purchase_date: Vec<PurchaseDateProperty>,
    #[cfg(any(
        any(
            feature = "release-date-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "releaseDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/releaseDate"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/releaseDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#release_date: Vec<ReleaseDateProperty>,
    #[cfg(any(
        any(feature = "review-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "review"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/review"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/review"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#review: Vec<ReviewProperty>,
    #[cfg(any(
        any(
            feature = "reviews-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "reviews"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/reviews"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/reviews"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#reviews: Vec<ReviewsProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(feature = "size-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "size"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/size"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/size"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#size: Vec<SizeProperty>,
    #[cfg(any(
        any(feature = "sku-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sku"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sku"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sku"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#sku: Vec<SkuProperty>,
    #[cfg(any(
        any(feature = "slogan-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "slogan"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/slogan"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/slogan"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#slogan: Vec<SloganProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        any(
            feature = "varies-by-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "variesBy"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/variesBy"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/variesBy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#varies_by: Vec<VariesByProperty>,
    #[cfg(any(
        any(feature = "weight-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "weight"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/weight"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/weight"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#weight: Vec<WeightProperty>,
    #[cfg(any(
        any(feature = "width-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "width"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/width"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/width"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#width: Vec<WidthProperty>,
}
