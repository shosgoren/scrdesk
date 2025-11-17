#!/bin/bash

###############################################################################
# SCRDESK Quick Deploy Script
# This script provides the command to run on VPS
###############################################################################

VPS_IP="72.61.138.218"
DOMAIN="scrdesk.com.tr"
EMAIL="admin@scrdesk.com.tr"

cat << 'EOF'
╔══════════════════════════════════════════════════════════╗
║  SCRDESK Production Deployment                           ║
║  Quick Installation Guide                                ║
╚══════════════════════════════════════════════════════════╝

STEP 1: Connect to Your VPS
────────────────────────────────────────────────────────────
EOF

echo "ssh root@$VPS_IP"
echo "Password: johqid-4Cuhxi-nugvov"
echo

cat << 'EOF'

STEP 2: Run This Command on VPS
────────────────────────────────────────────────────────────
Copy and paste the following command:

EOF

echo "curl -fsSL https://raw.githubusercontent.com/shosgoren/scrdesk/main/scripts/install-production.sh | \\"
echo "  SCRDESK_DOMAIN=$DOMAIN \\"
echo "  SCRDESK_EMAIL=$EMAIL \\"
echo "  bash"

echo

cat << EOF

Or if you prefer step-by-step:

# Download the script
wget https://raw.githubusercontent.com/shosgoren/scrdesk/main/scripts/install-production.sh -O install.sh

# Make it executable
chmod +x install.sh

# Run it
SCRDESK_DOMAIN=$DOMAIN SCRDESK_EMAIL=$EMAIL ./install.sh


STEP 3: Configure DNS (While installation runs)
────────────────────────────────────────────────────────────
Go to your domain registrar (nic.tr or wherever you bought the domain)
and add these A records:

  Host/Name         Type    Value           TTL
  ─────────────────────────────────────────────────
  @                 A       $VPS_IP         3600
  www               A       $VPS_IP         3600
  server            A       $VPS_IP         3600

DNS propagation can take 10 minutes to 48 hours.


STEP 4: After Installation Completes
────────────────────────────────────────────────────────────
The installation script will display:
  ✓ Public Key (SAVE THIS!)
  ✓ Server addresses
  ✓ Client configuration instructions


STEP 5: Test Your Installation
────────────────────────────────────────────────────────────
1. Visit: https://$DOMAIN
2. Configure client:
   - ID Server: server.$DOMAIN:21116
   - Relay Server: server.$DOMAIN:21117
   - Key: [from installation output]


╔══════════════════════════════════════════════════════════╗
║  Need Help?                                              ║
╚══════════════════════════════════════════════════════════╝

Documentation:
  • Full Guide: PRODUCTION_DEPLOYMENT.md
  • Quick Start: HOSTING_QUICKSTART.md
  • GitHub: https://github.com/shosgoren/scrdesk

Common Issues:
  • "Domain not found": DNS not propagated yet, wait longer
  • "Connection refused": Firewall blocking, run: ufw status
  • "SSL error": DNS must resolve first, then run: certbot renew

Useful Commands on VPS:
  • View logs:    docker-compose -f /opt/scrdesk/docker-compose.yml logs -f
  • Restart:      docker-compose -f /opt/scrdesk/docker-compose.yml restart
  • Get key:      cat /opt/scrdesk/public_key.txt
  • Check status: docker ps


🎉 Installation typically takes 10-15 minutes.
   Be patient and let the script complete!

EOF
