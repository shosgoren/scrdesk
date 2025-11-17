#!/bin/bash

###############################################################################
# SCRDESK Production Deployment for scrdesk.com.tr
# VPS: srv1123022.hstgr.cloud (72.61.138.218)
#
# This script will:
# 1. Connect to VPS
# 2. Install all requirements
# 3. Deploy SCRDESK server
# 4. Setup SSL for scrdesk.com.tr
###############################################################################

set -e

# Configuration
VPS_IP="72.61.138.218"
VPS_HOST="srv1123022.hstgr.cloud"
DOMAIN="scrdesk.com.tr"
SUBDOMAIN="server"
EMAIL="admin@scrdesk.com.tr"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║${NC}  ${GREEN}SCRDESK Production Deployment${NC}                       ${BLUE}║${NC}"
echo -e "${BLUE}║${NC}  VPS: ${YELLOW}$VPS_IP${NC}                                  ${BLUE}║${NC}"
echo -e "${BLUE}║${NC}  Domain: ${YELLOW}$DOMAIN${NC}                           ${BLUE}║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo

echo -e "${GREEN}[1/5]${NC} Uploading installation script to VPS..."
scp -o StrictHostKeyChecking=no \
    /Users/shosgoren/Documents/Visual\ Studio\ Code/SCRDESK/scripts/install-production.sh \
    root@$VPS_IP:/tmp/install-scrdesk.sh

echo -e "${GREEN}[2/5]${NC} Making script executable..."
ssh root@$VPS_IP "chmod +x /tmp/install-scrdesk.sh"

echo
echo -e "${YELLOW}[!] Starting installation on VPS...${NC}"
echo -e "${YELLOW}[!] This will take 10-15 minutes.${NC}"
echo

echo -e "${GREEN}[3/5]${NC} Running installation..."
ssh root@$VPS_IP "SCRDESK_DOMAIN=$DOMAIN SCRDESK_EMAIL=$EMAIL /tmp/install-scrdesk.sh"

echo
echo -e "${GREEN}[4/5]${NC} Getting public key..."
PUBLIC_KEY=$(ssh root@$VPS_IP "cat /opt/scrdesk/public_key.txt 2>/dev/null || cat /opt/scrdesk/data/id_ed25519.pub")

echo
echo -e "${GREEN}[5/5]${NC} Installation complete!"
echo

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║${NC}  ${GREEN}SCRDESK Successfully Deployed!${NC}                      ${BLUE}║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo
echo -e "${YELLOW}Server Information:${NC}"
echo "  VPS IP:       $VPS_IP"
echo "  Domain:       $DOMAIN"
echo "  Subdomain:    $SUBDOMAIN.$DOMAIN"
echo "  Website:      https://$DOMAIN (after DNS setup)"
echo
echo -e "${YELLOW}IMPORTANT - Save this Public Key:${NC}"
echo -e "${GREEN}$PUBLIC_KEY${NC}"
echo
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Configure DNS (A records):"
echo "     $DOMAIN          → $VPS_IP"
echo "     www.$DOMAIN      → $VPS_IP"
echo "     $SUBDOMAIN.$DOMAIN → $VPS_IP"
echo
echo "  2. Wait for DNS propagation (10 min - 2 hours)"
echo
echo "  3. Client Configuration:"
echo "     ID Server:    $SUBDOMAIN.$DOMAIN:21116"
echo "     Relay Server: $SUBDOMAIN.$DOMAIN:21117"
echo "     Key:          $PUBLIC_KEY"
echo
echo -e "${YELLOW}Useful Commands:${NC}"
echo "  View logs:    ssh root@$VPS_IP 'docker-compose -f /opt/scrdesk/docker-compose.yml logs -f'"
echo "  Restart:      ssh root@$VPS_IP 'docker-compose -f /opt/scrdesk/docker-compose.yml restart'"
echo "  Get key:      ssh root@$VPS_IP 'cat /opt/scrdesk/public_key.txt'"
echo

# Save info locally
cat > /Users/shosgoren/Documents/Visual\ Studio\ Code/SCRDESK/.scrdesk/deployment-info.txt << EOF
SCRDESK Production Deployment
==============================
Date: $(date)

VPS Information:
  IP:       $VPS_IP
  Host:     $VPS_HOST
  Domain:   $DOMAIN

Server Configuration:
  ID Server:    $SUBDOMAIN.$DOMAIN:21116
  Relay Server: $SUBDOMAIN.$DOMAIN:21117

Public Key:
$PUBLIC_KEY

Client Settings:
  1. Settings → Network
  2. ID Server: $SUBDOMAIN.$DOMAIN:21116
  3. Relay Server: $SUBDOMAIN.$DOMAIN:21117
  4. Key: $PUBLIC_KEY

Management:
  SSH: ssh root@$VPS_IP
  Logs: docker-compose -f /opt/scrdesk/docker-compose.yml logs -f
  Restart: docker-compose -f /opt/scrdesk/docker-compose.yml restart
EOF

echo -e "${GREEN}✓${NC} Deployment info saved to: .scrdesk/deployment-info.txt"
echo
