# Git Setup Guide

## Initial Git Setup

If this is a new repository, follow these steps:

### 1. Initialize Git Repository

```bash
cd "c:\Users\pennc\Documents\Nic Pouch Deals\Apps\Tiered Discounts app\tiered-discounts"
git init
```

### 2. Add All Files

```bash
git add .
```

### 3. Create Initial Commit

```bash
git commit -m "Initial commit: Tiered Discounts Shopify App with Rust Function"
```

### 4. Add Remote Repository

Replace `YOUR_USERNAME` and `YOUR_REPO_NAME` with your actual GitHub/GitLab details:

```bash
# For GitHub
git remote add origin https://github.com/YOUR_USERNAME/YOUR_REPO_NAME.git

# Or for GitLab
git remote add origin https://gitlab.com/YOUR_USERNAME/YOUR_REPO_NAME.git
```

### 5. Push to Remote

```bash
git branch -M main
git push -u origin main
```

## What's Included in the Repository

✅ **Included:**
- All source code (Rust, GraphQL, JavaScript/React)
- Configuration files (`shopify.app.toml`, `Cargo.toml`, `package.json`)
- Documentation files (`README.md`, `DEPLOYMENT.md`, `SETUP_AND_TEST.md`)
- GraphQL queries and schemas
- Extension configurations

❌ **Excluded (via .gitignore):**
- `node_modules/` - Dependencies (install with `npm install`)
- `target/` - Rust build artifacts (rebuild with `cargo build`)
- `.shopify/` - Local development files
- `database.sqlite` - Local database file
- `.env` files - Environment variables (create these locally)
- Build outputs and logs

## Important Notes

### Environment Variables

You'll need to create a `.env` file locally (not committed to Git) with:

```env
SHOPIFY_API_KEY=your_api_key_here
SHOPIFY_API_SECRET=your_api_secret_here
SCOPES=write_products,write_discounts,read_discounts
```

### After Cloning

When someone clones this repository, they'll need to:

1. **Install Node.js dependencies:**
   ```bash
   npm install
   ```

2. **Install Rust (if not already installed):**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Add WebAssembly target
   rustup target add wasm32-unknown-unknown
   ```

3. **Set up environment variables:**
   - Copy `.env.example` to `.env` (if you create one)
   - Or create `.env` manually with required variables

4. **Build the function:**
   ```bash
   shopify app function build
   ```

## Git Workflow Recommendations

### Branch Strategy

- `main` - Production-ready code
- `develop` - Development branch
- `feature/*` - Feature branches

### Commit Messages

Use clear, descriptive commit messages:

```bash
git commit -m "Add tiered discount function with metafield support"
git commit -m "Fix discount application to all products in cart"
git commit -m "Update deployment documentation"
```

### Before Pushing

Always ensure:
- ✅ Code compiles (`shopify app function build`)
- ✅ No sensitive data in commits (check `.env`, secrets, etc.)
- ✅ `.gitignore` is working correctly
- ✅ Documentation is up to date

## Digital Ocean Deployment Preparation

Before deploying to Digital Ocean, you may want to:

1. **Create a production branch:**
   ```bash
   git checkout -b production
   git push -u origin production
   ```

2. **Update `shopify.app.toml` with production URLs:**
   ```toml
   application_url = "https://your-domain.com"
   redirect_urls = [ "https://your-domain.com/api/auth" ]
   ```

3. **Create a `.env.production` template** (don't commit actual values):
   ```env
   SHOPIFY_API_KEY=
   SHOPIFY_API_SECRET=
   SCOPES=write_products,write_discounts,read_discounts
   NODE_ENV=production
   ```

## Useful Git Commands

```bash
# Check status
git status

# See what files are tracked/ignored
git status --ignored

# View recent commits
git log --oneline

# Create a new branch
git checkout -b feature-name

# Switch branches
git checkout main

# Merge a branch
git checkout main
git merge feature-name

# Push to remote
git push origin main

# Pull latest changes
git pull origin main
```
