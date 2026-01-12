use crate::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[derive(Deserialize, Default, PartialEq)]
pub struct PackPrice {
    pub quantity: i32,
    // unit_price is stored as an integer in the smallest currency unit (pence)
    // e.g., 383 means £3.83 (383 pence)
    pub unit_price: i32,
}

// Type alias for the array - this is what the custom scalar override will deserialize to
pub type PackPrices = Vec<PackPrice>;

#[shopify_function]
fn cart_lines_discounts_generate_run(
    input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<schema::CartLinesDiscountsGenerateRunResult> {
    let mut candidates = vec![];

    for line in input.cart().lines() {
        let variant = match &line.merchandise() {
            schema::cart_lines_discounts_generate_run::input::cart::lines::Merchandise::ProductVariant(variant) => variant,
            _ => continue,
        };

        // Get the metafield with tiered pricing from the product (not the variant)
        let product = variant.product();
        let metafield = match product.metafield() {
            Some(mf) => mf,
            None => continue, // Skip products without the metafield
        };

        // Deserialize the JSON metafield into PackPrices
        // Try to deserialize JsonValue directly using shopify_function's Deserialize
        let json_value = metafield.json_value();
        
        // Extract array from JsonValue and manually parse each object
        let pack_prices: Vec<PackPrice> = match json_value {
            JsonValue::Array(arr) => {
                let mut prices = Vec::new();
                for item in arr.iter() {
                    if let JsonValue::Object(obj) = item {
                        let mut quantity: Option<i32> = None;
                        let mut unit_price: Option<i32> = None;
                        
                        for (key, value) in obj.iter() {
                            match key.as_str() {
                                "quantity" => {
                                    if let JsonValue::Number(n) = value {
                                        // JsonValue::Number is f64, convert to i32
                                        quantity = Some(*n as i32);
                                    }
                                }
                                "unit_price" => {
                                    if let JsonValue::Number(n) = value {
                                        // JsonValue::Number is f64, convert to i32
                                        unit_price = Some(*n as i32);
                                    }
                                }
                                _ => {}
                            }
                        }
                        
                        if let (Some(qty), Some(price)) = (quantity, unit_price) {
                            prices.push(PackPrice {
                                quantity: qty,
                                unit_price: price,
                            });
                        }
                    }
                }
                prices
            }
            _ => continue, // Only handle arrays, skip if not an array
        };

        if pack_prices.is_empty() {
            continue; // Skip if no valid pricing tiers found
        }

        // Get the current quantity in cart
        let cart_quantity = *line.quantity();

        // Find the best tier that applies to this quantity
        // Sort by quantity descending to find the highest applicable tier
        let mut applicable_tier: Option<&PackPrice> = None;
        for tier in pack_prices.iter() {
            if tier.quantity <= cart_quantity {
                match applicable_tier {
                    None => applicable_tier = Some(tier),
                    Some(current) => {
                        if tier.quantity > current.quantity {
                            applicable_tier = Some(tier);
                        }
                    }
                }
            }
        }

        // If we found an applicable tier, calculate the discount
        if let Some(tier) = applicable_tier {
            let current_price = line.cost().amount_per_quantity().amount().0;
            // Convert pence to pounds: divide by 100.0
            // e.g., 383 pence = £3.83
            let tier_price_value = tier.unit_price as f64 / 100.0;
            
            // Only apply discount if tier price is less than current price
            if tier_price_value < current_price {
                let discount_amount = Decimal::from(current_price - tier_price_value);
                
                // Format price for display: convert pence to pounds
                let tier_price_display = tier.unit_price as f64 / 100.0;
                
                candidates.push(schema::ProductDiscountCandidate {
                    value: schema::ProductDiscountCandidateValue::FixedAmount(
                        schema::ProductDiscountCandidateFixedAmount {
                            amount: discount_amount,
                            applies_to_each_item: Some(true),
                        },
                    ),
                    targets: vec![schema::ProductDiscountCandidateTarget::CartLine(
                        schema::CartLineTarget {
                            id: line.id().clone(),
                            quantity: None,
                        },
                    )],
                    message: Some(format!(
                        "Tiered pricing: {} units at £{:.2} each",
                        tier.quantity,
                        tier_price_display
                    )),
                    associated_discount_code: None,
                });
            }
        }
    }

    if candidates.is_empty() {
        return Ok(schema::CartLinesDiscountsGenerateRunResult {
            operations: vec![],
        });
    }

    Ok(schema::CartLinesDiscountsGenerateRunResult {
        operations: vec![schema::CartOperation::ProductDiscountsAdd(
            schema::ProductDiscountsAddOperation {
                selection_strategy: schema::ProductDiscountSelectionStrategy::All,
                candidates,
            },
        )],
    })
}
