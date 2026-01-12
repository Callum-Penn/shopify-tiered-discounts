# Setting Up and Testing Your Tiered Discount Function

## Step 1: Create an Automatic Discount

Your function is already deployed and installed. Now you need to create a discount in your Shopify admin that uses this function.

### Option A: Using GraphiQL (Recommended for Testing)

1. **Open GraphiQL**:
   - In your terminal where `shopify app dev` is running, press `g` to open GraphiQL
   - Or navigate to: https://shopify-graphiql-app.shopifycloud.com/
   - Make sure you're authenticated with your dev store

2. **Create the Discount**:
   Run this mutation in GraphiQL:

```graphql
mutation {
  discountAutomaticAppCreate(
    automaticAppDiscount: {
      title: "Tiered Quantity Discount"
      functionHandle: "tiered-quantity-discount"
      discountClasses: [PRODUCT]
      startsAt: "2025-01-01T00:00:00Z"
    }
  ) {
    automaticAppDiscount {
      discountId
      title
      status
    }
    userErrors {
      field
      message
    }
  }
}
```

3. **Save the discount ID** from the response for future reference.

### Option B: Using Shopify Admin UI

1. Go to your Shopify admin → **Discounts**
2. Click **Create discount**
3. Under your app name ("Tiered Discounts"), select your discount function
4. Configure:
   - **Method**: Automatic
   - **Title**: Tiered Quantity Discount
   - **Discount Classes**: Select **Product**
   - **Start date**: Set to today or a past date
5. Click **Save**

## Step 2: Set Up Product Metafields (For Future Use)

**Note**: Currently, your function uses hardcoded sample data. Once you implement metafield deserialization, you'll need to:

1. Go to **Settings** → **Custom data** → **Products**
2. Create a metafield:
   - **Name**: `pack_prices`
   - **Namespace and key**: `custom.pack_prices`
   - **Type**: JSON
   - **Description**: Tiered pricing data

3. For each product variant, add the metafield with JSON like:
```json
[
  {"quantity": 1, "unit_price": 383},
  {"quantity": 5, "unit_price": 364},
  {"quantity": 10, "unit_price": 345},
  {"quantity": 20, "unit_price": 325}
]
```

**Remember**: `unit_price` values are in **pence** (e.g., 383 = £3.83)

## Step 3: Test the Discount

### Current Behavior (With Hardcoded Data)

Your function currently uses these hardcoded tiered prices:
- 1 unit: £3.83 (383 pence)
- 5 units: £3.64 (364 pence)
- 10 units: £3.45 (345 pence)
- 20 units: £3.25 (325 pence)

### Testing Steps

1. **Deactivate Other Discounts**:
   - Go to **Discounts** in Shopify admin
   - Deactivate or delete any other active discounts to avoid conflicts

2. **Add Products to Cart**:
   - Open your dev store frontend
   - Add a product to your cart
   - **Important**: The function currently applies to products that have the `custom.pack_prices` metafield (even though it uses hardcoded data). Make sure your test product has this metafield set, or the function will skip it.

3. **Test Different Quantities**:
   - Add **1 item**: Should see discount to £3.83 per unit
   - Add **5 items**: Should see discount to £3.64 per unit
   - Add **10 items**: Should see discount to £3.45 per unit
   - Add **20 items**: Should see discount to £3.25 per unit

4. **Check the Cart**:
   - The discount should appear automatically on the cart page
   - You should see a message like: "Tiered pricing: 5 units at £3.64 each"

5. **Proceed to Checkout**:
   - The discount should persist through checkout
   - Verify the final price is correct

### View Function Execution Logs

1. In your terminal where `shopify app dev` is running, you'll see function execution logs
2. Each time a cart is updated, the function runs and logs will appear
3. Look for any errors or debug information

### Replay Function Executions Locally

To debug a specific execution:

1. In a new terminal, run:
   ```bash
   shopify app function replay
   ```

2. Select the function execution from the list
3. This will show you the exact input and output for that execution

## Troubleshooting

### Discount Not Appearing

1. **Check Discount is Active**:
   - Go to **Discounts** in admin
   - Ensure "Tiered Quantity Discount" is active

2. **Check Product Metafield**:
   - The function currently requires the `custom.pack_prices` metafield to exist (even though it uses hardcoded data)
   - Go to **Products** → Select your product → Scroll to **Metafields**
   - Ensure `custom.pack_prices` exists (can be empty JSON `[]` for now)

3. **Check Function Logs**:
   - Look at the terminal where `shopify app dev` is running
   - Check for any error messages

4. **Verify Function Handle**:
   - Ensure the discount was created with `functionHandle: "tiered-quantity-discount"`

### Discount Amount Seems Wrong

- Remember: `unit_price` in the metafield is in **pence**
- 383 pence = £3.83
- The function converts pence to pounds by dividing by 100

### Function Not Running

1. Ensure the app is installed on your dev store
2. Ensure `shopify app dev` is running
3. Check that the discount is active and not expired

## Next Steps

1. **Implement Real Metafield Deserialization**: Replace the hardcoded `pack_prices` with actual metafield data
2. **Add More Products**: Test with multiple products that have different tiered pricing
3. **Test Edge Cases**: Test with quantities that don't match tiers exactly (e.g., 7 items should use the 5-unit tier)

## GraphQL Mutation Reference

To update the discount later:

```graphql
mutation {
  discountAutomaticAppUpdate(
    id: "gid://shopify/DiscountAutomaticApp/YOUR_DISCOUNT_ID"
    automaticAppDiscount: {
      title: "Updated Tiered Quantity Discount"
      # ... other fields
    }
  ) {
    automaticAppDiscount {
      discountId
      title
    }
    userErrors {
      field
      message
    }
  }
}
```

To deactivate the discount:

```graphql
mutation {
  discountAutomaticDeactivate(id: "gid://shopify/DiscountAutomaticApp/YOUR_DISCOUNT_ID") {
    automaticDiscount {
      discountId
    }
    userErrors {
      field
      message
    }
  }
}
```
