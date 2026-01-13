# Digital Ocean Deployment Guide

This guide walks you through deploying your Tiered Discounts Shopify App to a Digital Ocean Droplet.

## Prerequisites

- ✅ A Digital Ocean account ([Sign up here](https://www.digitalocean.com/))
- ✅ Your GitHub repository is set up and pushed
- ✅ A domain name (optional but recommended)
- ✅ SSH key pair for server access

---

## Step 1: Create a Digital Ocean Droplet

1. **Log in to Digital Ocean** and go to the Dashboard
2. Click **Create** → **Droplets**
3. **Choose an image**: Select **Ubuntu 22.04 (LTS) x64**
4. **Choose a plan**: 
   - Minimum: **Basic** plan, **Regular Intel** with **$6/month** (1GB RAM, 1 vCPU)
   - Recommended: **$12/month** (2GB RAM, 1 vCPU) for better performance
5. **Choose a datacenter region**: Select closest to your users
6. **Authentication**: 
   - Choose **SSH keys** (recommended) or **Password**
   - If using SSH keys, add your public key
7. **Finalize**:
   - Give your droplet a hostname (e.g., `shopify-tiered-discounts`)
   - Click **Create Droplet**

**Wait 1-2 minutes** for the droplet to be created.

---

## Step 2: Connect to Your Server

### On Windows (PowerShell or Git Bash):

```bash
ssh root@YOUR_DROPLET_IP_ADDRESS
```

Replace `YOUR_DROPLET_IP_ADDRESS` with the IP shown in your Digital Ocean dashboard.

**First time connection**: Type `yes` when prompted about authenticity.

---

## Step 3: Initial Server Setup

### 3.1 Update System Packages

```bash
apt update && apt upgrade -y
```

### 3.2 Create a Non-Root User (Recommended)

```bash
# Create a new user
adduser shopify
# Add to sudo group
usermod -aG sudo shopify
# Switch to the new user
su - shopify
```

---

## Step 4: Install Required Software

### 4.1 Install Node.js (v18 or v20)

```bash
# Install Node.js using NodeSource repository
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Verify installation
node --version
npm --version
```

### 4.2 Install Rust and Cargo

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts (press Enter for default installation)
# After installation, reload the shell
source $HOME/.cargo/env

# Add WebAssembly target (required for Shopify Functions)
rustup target add wasm32-unknown-unknown

# Verify installation
rustc --version
cargo --version
```

### 4.3 Install Shopify CLI

```bash
# Install Shopify CLI globally
npm install -g @shopify/cli @shopify/theme

# Verify installation
shopify version
```

### 4.4 Install Git

```bash
sudo apt install -y git
```

### 4.5 Install PM2 (Process Manager)

```bash
sudo npm install -g pm2
```

PM2 will keep your app running and restart it if it crashes.

### 4.6 Install Nginx (Web Server/Reverse Proxy)

```bash
sudo apt install -y nginx
```

---

## Step 5: Clone Your Repository

```bash
# Navigate to a suitable directory
cd /home/shopify

# Clone your repository
git clone https://github.com/Callum-Penn/shopify-tiered-discounts.git

# Navigate into the project
cd shopify-tiered-discounts
```

---

## Step 6: Set Up Environment Variables

### 6.1 Create `.env` File

```bash
# Create .env file
nano .env
```

Add your environment variables (you'll get these from your Shopify Partners Dashboard):

```env
SHOPIFY_API_KEY=your_api_key_here
SHOPIFY_API_SECRET=your_api_secret_here
SCOPES=write_products,write_discounts,read_discounts
HOST=your-domain.com
```

**Important**: Replace with your actual values from Shopify Partners Dashboard.

### 6.2 Get Your API Credentials

1. Go to [Shopify Partners Dashboard](https://partners.shopify.com/)
2. Navigate to **Apps** → **Tiered Discounts**
3. Go to **App setup** → **Client credentials**
4. Copy your **Client ID** (API Key) and **Client secret**

### 6.3 Save and Exit

Press `Ctrl+X`, then `Y`, then `Enter` to save.

---

## Step 7: Install Project Dependencies

```bash
# Install Node.js dependencies
npm install

# Navigate to frontend and install dependencies
cd web/frontend
npm install
cd ../..
```

---

## Step 8: Update Shopify App Configuration

### 8.1 Update `shopify.app.toml`

```bash
nano shopify.app.toml
```

Update the URLs to your Digital Ocean domain:

```toml
application_url = "https://your-domain.com"
redirect_urls = [ "https://your-domain.com/api/auth" ]
```

**If you don't have a domain yet**, you can use your droplet's IP address temporarily, but you'll need a domain for production.

### 8.2 Update URLs in Shopify Partners Dashboard

1. Go to [Shopify Partners Dashboard](https://partners.shopify.com/)
2. Navigate to **Apps** → **Tiered Discounts**
3. Go to **App setup** → **App URL**
4. Update:
   - **App URL**: `https://your-domain.com`
   - **Allowed redirection URL(s)**: `https://your-domain.com/api/auth`

---

## Step 9: Build and Deploy the Function

```bash
# Build the Shopify Function
shopify app function build

# Deploy the app (this will prompt for authentication)
shopify app deploy
```

**Note**: You'll need to authenticate with your Shopify Partners account. Follow the prompts.

---

## Step 10: Set Up Nginx Reverse Proxy

### 10.1 Create Nginx Configuration

```bash
sudo nano /etc/nginx/sites-available/shopify-tiered-discounts
```

Add this configuration:

```nginx
server {
    listen 80;
    server_name your-domain.com www.your-domain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

**Replace `your-domain.com` with your actual domain** (or use your IP address temporarily).

### 10.2 Enable the Site

```bash
# Create symbolic link
sudo ln -s /etc/nginx/sites-available/shopify-tiered-discounts /etc/nginx/sites-enabled/

# Remove default site (optional)
sudo rm /etc/nginx/sites-enabled/default

# Test Nginx configuration
sudo nginx -t

# Restart Nginx
sudo systemctl restart nginx
```

---

## Step 11: Set Up SSL with Let's Encrypt (Recommended)

### 11.1 Install Certbot

```bash
sudo apt install -y certbot python3-certbot-nginx
```

### 11.2 Obtain SSL Certificate

```bash
sudo certbot --nginx -d your-domain.com -d www.your-domain.com
```

Follow the prompts:
- Enter your email address
- Agree to terms
- Choose whether to redirect HTTP to HTTPS (recommended: Yes)

### 11.3 Auto-Renewal

Certbot sets up auto-renewal automatically. Test it:

```bash
sudo certbot renew --dry-run
```

---

## Step 12: Start Your App with PM2

### 12.1 Start the App

```bash
# Navigate to project root
cd /home/shopify/shopify-tiered-discounts

# Start with PM2
pm2 start web/index.js --name tiered-discounts

# Save PM2 configuration
pm2 save

# Set up PM2 to start on boot
pm2 startup
```

Follow the command that PM2 outputs (usually something like `sudo env PATH=...`).

### 12.2 Useful PM2 Commands

```bash
# View app status
pm2 status

# View logs
pm2 logs tiered-discounts

# Restart app
pm2 restart tiered-discounts

# Stop app
pm2 stop tiered-discounts
```

---

## Step 13: Configure Firewall

```bash
# Allow SSH (important - don't lock yourself out!)
sudo ufw allow OpenSSH

# Allow HTTP and HTTPS
sudo ufw allow 'Nginx Full'

# Enable firewall
sudo ufw enable

# Check status
sudo ufw status
```

---

## Step 14: Test Your Deployment

1. **Visit your app URL**: `https://your-domain.com`
2. **Check app status**: `pm2 status`
3. **Check Nginx**: `sudo systemctl status nginx`
4. **View app logs**: `pm2 logs tiered-discounts`

---

## Step 15: Complete Shopify Setup

Follow the steps in `DEPLOYMENT.md`:

1. **Install the app** on your production store
2. **Create the discount** using GraphQL or Admin UI
3. **Set up product metafields** for tiered pricing
4. **Test the discount** functionality

---

## Troubleshooting

### App Not Starting

```bash
# Check PM2 logs
pm2 logs tiered-discounts

# Check if port 3000 is in use
sudo netstat -tulpn | grep 3000

# Restart the app
pm2 restart tiered-discounts
```

### Nginx Errors

```bash
# Check Nginx error logs
sudo tail -f /var/log/nginx/error.log

# Test Nginx configuration
sudo nginx -t

# Restart Nginx
sudo systemctl restart nginx
```

### Function Build Errors

```bash
# Clean Rust build artifacts
cd extensions/tiered-quantity-discount
cargo clean
cd ../..

# Rebuild
shopify app function build
```

### Can't Connect via SSH

- Verify your droplet is running in Digital Ocean dashboard
- Check that your IP address hasn't changed
- Ensure firewall allows SSH: `sudo ufw allow OpenSSH`

### Domain Not Resolving

- Verify DNS records point to your droplet's IP
- Check DNS propagation: `nslookup your-domain.com`
- Ensure Nginx is listening on port 80/443

---

## Maintenance

### Updating Your App

```bash
# Navigate to project
cd /home/shopify/shopify-tiered-discounts

# Pull latest changes
git pull

# Install new dependencies (if any)
npm install

# Rebuild function (if changed)
shopify app function build

# Deploy updates
shopify app deploy

# Restart app
pm2 restart tiered-discounts
```

### Monitoring

- **PM2 Monitoring**: `pm2 monit`
- **System Resources**: `htop` or `top`
- **Disk Space**: `df -h`
- **Nginx Access Logs**: `sudo tail -f /var/log/nginx/access.log`

---

## Security Best Practices

1. ✅ **Use SSH keys** instead of passwords
2. ✅ **Keep system updated**: `sudo apt update && sudo apt upgrade`
3. ✅ **Use a non-root user** for running the app
4. ✅ **Enable firewall** (UFW)
5. ✅ **Use SSL/HTTPS** (Let's Encrypt)
6. ✅ **Keep `.env` file secure** (don't commit it to Git)
7. ✅ **Regular backups** of your code and database
8. ✅ **Monitor logs** for suspicious activity

---

## Cost Estimate

- **Droplet**: $6-12/month (Basic plan)
- **Domain**: ~$10-15/year (optional)
- **Total**: ~$6-12/month + domain

---

## Next Steps

- Set up automated backups
- Configure monitoring and alerts
- Set up a staging environment
- Implement CI/CD pipeline
- Add database if needed (PostgreSQL, MySQL, etc.)

---

## Need Help?

- [Digital Ocean Documentation](https://docs.digitalocean.com/)
- [Shopify App Deployment Guide](https://shopify.dev/docs/apps/deployment/web)
- [PM2 Documentation](https://pm2.keymetrics.io/docs/usage/quick-start/)
- [Nginx Documentation](https://nginx.org/en/docs/)
