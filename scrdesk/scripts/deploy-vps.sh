#!/bin/bash
set -e

echo "🚀 ScrDesk PRO Enterprise - VPS Deployment Script"
echo "=================================================="

VPS_HOST="72.61.138.218"
VPS_USER="root"
VPS_PASSWORD="&2BzMs7Zd.JEABbjP)rr"
DEPLOY_DIR="/opt/scrdesk"
REPO_URL="https://github.com/shosgoren/scrdesk.git"

echo "🔧 Deploying to VPS..."
sshpass -p "$VPS_PASSWORD" ssh -o StrictHostKeyChecking=no $VPS_USER@$VPS_HOST << 'ENDSSH'
set -e

# Install dependencies
apt-get update
apt-get install -y docker.io docker-compose git curl

# Clone or update repository
if [ -d "/opt/scrdesk" ]; then
    echo "📥 Updating existing repository..."
    cd /opt/scrdesk
    git fetch origin
    git reset --hard origin/main
    git pull origin main
else
    echo "📥 Cloning repository..."
    git clone https://github.com/shosgoren/scrdesk.git /opt/scrdesk
    cd /opt/scrdesk
fi

# Create .env file
cp .env.example .env

# Update .env with production values
sed -i 's/scrdesk_dev_password/$(openssl rand -base64 32)/g' .env
sed -i 's/your-super-secret-jwt-key-change-in-production/$(openssl rand -base64 64)/g' .env

# Start services
docker-compose down || true
docker-compose up -d --build

# Wait for services to be ready
echo "⏳ Waiting for services to start..."
sleep 30

# Check service health
docker-compose ps

echo "✅ Deployment completed!"
echo "🌐 Access points:"
echo "   Admin Panel: http://72.61.138.218:3000"
echo "   Core API: http://72.61.138.218:8000"
echo "   Relay Server: tcp://72.61.138.218:21117"

ENDSSH

echo "✅ VPS Deployment completed successfully!"
echo "🌐 Your ScrDesk instance is running at http://72.61.138.218"
