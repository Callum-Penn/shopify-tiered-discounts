# Production Deployment Guide

## Prerequisites

Before deploying to production, ensure:
- ✅ The function is working correctly in your dev store
- ✅ All tests pass
- ✅ You have a production hosting solution for your app backend (if needed)
- ✅ You have your production store URL ready

## Step 1: Update Production URLs (If Needed)

If you have a production backend server, update `shopify.app.toml`:

```toml
application_url = "https://your-production-domain.com"
redirect_urls = [ "https://your-production-domain.com/api/auth" ]
```

**Note**: If you're using the default template URLs (`https://example.com`), you may need to set up a production server or use Shopify's hosting options.

## Step 2: Build and Deploy the App

1. **Build the function** to ensure everything compiles:
   ```bash
   shopify app function build
   ```

2. **Deploy the app**:
   ```bash
   shopify app deploy
   ```

   This will:
   - Create a new app version
   - Build all extensions (including your function)
   - Upload everything to Shopify

3. **Release the app version** (if not automatically released):
   ```bash
   shopify app release
   ```

   Or you can release it through the Shopify Partners Dashboard.

## Step 3: Install on Production Store

### Option A: Install via Partners Dashboard

1. Go to [Shopify Partners Dashboard](https://partners.shopify.com/)
2. Navigate to **Apps** → **Tiered Discounts**
3. Go to **Overview** or **Distribution**
4. Find your production store and click **Install** or **Add store**
5. Authorize the app with the required scopes

### Option B: Install via Store Admin

1. In your production store admin, go to **Settings** → **Apps and sales channels**
2. Click **Develop apps** (if you're the store owner) or ask the store owner to install
3. Find "Tiered Discounts" and click **Install**
4. Authorize the required permissions

## Step 4: Create the Discount in Production

After installing the app on your production store, create the discount:

### Using GraphiQL

1. Open GraphiQL in your production store (or use Shopify GraphiQL App)
2. Run this mutation:

```graphql
mutation {
  discountAutomaticAppCreate(
    automaticAppDiscount: {
      title: "Tiered Quantity Discount"
      functionHandle: "tiered-quantity-discount"
      discountClasses: [PRODUCT]
      startsAt: "2025-01-01T00:00:00Z"
      combinesWith: {
        productDiscounts: true
        orderDiscounts: true
        shippingDiscounts: true
      }
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

### Using Shopify Admin UI

1. Go to **Discounts** in your production store admin
2. Click **Create discount**
3. Under "Tiered Discounts" app, select your discount function
4. Configure:
   - **Method**: Automatic
   - **Title**: Tiered Quantity Discount
   - **Discount Classes**: Product
   - **Combines with**: Enable all discount types
   - **Start date**: Set appropriately
5. Click **Save**

## Step 5: Set Up Product Metafields in Production

For each product that should have tiered pricing:

1. Go to **Products** in your production store
2. Select a product
3. Scroll to **Metafields** section
4. Add a metafield:
   - **Namespace**: `custom`
   - **Key**: `pack_prices`
   - **Type**: JSON
   - **Value**: 
   ```json
   [
     {"quantity": 1, "unit_price": 383},
     {"quantity": 5, "unit_price": 364},
     {"quantity": 10, "unit_price": 345},
     {"quantity": 20, "unit_price": 325}
   ]
   ```
   **Remember**: `unit_price` is in **pence** (383 = £3.83)

## Step 6: Test in Production

1. Add products with the metafield to your cart
2. Test different quantities to verify tiered pricing works
3. Check that discounts apply to all products in the cart
4. Verify the discount amounts are correct

## Troubleshooting

### App Not Appearing in Store

- Ensure the app version is **released** (not just created)
- Check that you've installed the app on the production store
- Verify the app is not in draft mode

### Discount Not Working

- Verify the discount is **active** in the Discounts section
- Check that products have the `custom.pack_prices` metafield set
- Ensure the metafield JSON is valid
- Check function execution logs in the Partners Dashboard

### Function Errors

- Check the app's error history in Partners Dashboard
- Review function execution logs
- Ensure all dependencies are correctly deployed

## Important Notes

1. **App Versions**: Each deployment creates a new app version. Released versions are what stores see.

2. **Function Updates**: When you update the function code, you need to:
   - Rebuild: `shopify app function build`
   - Deploy: `shopify app deploy`
   - Release: `shopify app release` (or via dashboard)

3. **Metafields**: You'll need to set up the `custom.pack_prices` metafield on each product in production. Consider:
   - Using bulk import tools
   - Creating a script to set metafields via API
   - Using Shopify's bulk editor

4. **Monitoring**: After deployment, monitor:
   - Function execution logs
   - Error rates
   - Discount application rates
   - Customer feedback

## Next Steps

- Set up monitoring and alerts
- Create documentation for your team on how to manage tiered pricing
- Consider building an admin UI to manage tiered pricing more easily
- Plan for scaling if you have many products
