#!/bin/bash

###############################################################################
# SCRDESK Production Installation Script
# Domain: scrdesk.com.tr
# Author: SCRDESK Team
# Version: 1.0.0
###############################################################################

set -e  # Exit on error

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DOMAIN="${SCRDESK_DOMAIN:-scrdesk.com.tr}"
SUBDOMAIN="${SCRDESK_SUBDOMAIN:-server}"
EMAIL="${SCRDESK_EMAIL:-admin@scrdesk.com.tr}"
INSTALL_DIR="/opt/scrdesk"

###############################################################################
# Helper Functions
###############################################################################

print_header() {
    echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║${NC}  ${GREEN}SCRDESK Production Installation${NC}                    ${BLUE}║${NC}"
    echo -e "${BLUE}║${NC}  Domain: ${YELLOW}$DOMAIN${NC}                                ${BLUE}║${NC}"
    echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
    echo
}

print_step() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_info() {
    echo -e "${BLUE}[i]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

check_root() {
    if [[ $EUID -ne 0 ]]; then
        print_error "This script must be run as root"
        exit 1
    fi
}

confirm() {
    read -p "$1 [y/N]: " -n 1 -r
    echo
    [[ $REPLY =~ ^[Yy]$ ]]
}

###############################################################################
# Pre-flight Checks
###############################################################################

preflight_checks() {
    print_header

    print_info "Running pre-flight checks..."

    # Check OS
    if [ ! -f /etc/os-release ]; then
        print_error "Unsupported OS"
        exit 1
    fi

    . /etc/os-release
    if [[ "$ID" != "ubuntu" && "$ID" != "debian" ]]; then
        print_warning "This script is tested on Ubuntu/Debian"
        if ! confirm "Continue anyway?"; then
            exit 1
        fi
    fi

    # Check internet connection
    if ! ping -c 1 8.8.8.8 &> /dev/null; then
        print_error "No internet connection"
        exit 1
    fi

    # Get server IP
    SERVER_IP=$(curl -s ifconfig.me)
    print_info "Server IP: $SERVER_IP"

    # Check DNS
    print_info "Checking DNS for $DOMAIN..."
    DOMAIN_IP=$(dig +short $DOMAIN | tail -n1)

    if [ -z "$DOMAIN_IP" ]; then
        print_warning "Domain $DOMAIN does not resolve to any IP"
        print_info "Please configure DNS A record:"
        print_info "  $DOMAIN → $SERVER_IP"
        if ! confirm "Continue anyway?"; then
            exit 1
        fi
    elif [ "$DOMAIN_IP" != "$SERVER_IP" ]; then
        print_warning "Domain resolves to $DOMAIN_IP but server IP is $SERVER_IP"
        if ! confirm "Continue anyway?"; then
            exit 1
        fi
    else
        print_step "DNS correctly configured"
    fi

    echo
    print_info "Configuration:"
    echo "  Domain:     $DOMAIN"
    echo "  Subdomain:  $SUBDOMAIN.$DOMAIN"
    echo "  Server IP:  $SERVER_IP"
    echo "  Email:      $EMAIL"
    echo "  Install:    $INSTALL_DIR"
    echo

    if ! confirm "Proceed with installation?"; then
        print_info "Installation cancelled"
        exit 0
    fi
}

###############################################################################
# System Setup
###############################################################################

update_system() {
    print_info "Updating system..."
    apt update > /dev/null 2>&1
    apt upgrade -y > /dev/null 2>&1
    print_step "System updated"
}

install_dependencies() {
    print_info "Installing dependencies..."
    apt install -y curl wget git ufw fail2ban htop ncdu iftop \
        ca-certificates gnupg lsb-release > /dev/null 2>&1
    print_step "Dependencies installed"
}

setup_firewall() {
    print_info "Configuring firewall..."

    # Allow SSH
    ufw allow 22/tcp > /dev/null 2>&1

    # Allow HTTP/HTTPS
    ufw allow 80/tcp > /dev/null 2>&1
    ufw allow 443/tcp > /dev/null 2>&1

    # Allow SCRDESK ports
    ufw allow 21115:21119/tcp > /dev/null 2>&1
    ufw allow 21116/udp > /dev/null 2>&1

    # Enable firewall
    echo "y" | ufw enable > /dev/null 2>&1

    print_step "Firewall configured"
}

###############################################################################
# Docker Installation
###############################################################################

install_docker() {
    print_info "Installing Docker..."

    # Remove old versions
    apt remove -y docker docker-engine docker.io containerd runc > /dev/null 2>&1 || true

    # Add Docker's official GPG key
    install -m 0755 -d /etc/apt/keyrings
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | \
        gpg --dearmor -o /etc/apt/keyrings/docker.gpg > /dev/null 2>&1
    chmod a+r /etc/apt/keyrings/docker.gpg

    # Add repository
    echo \
        "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] \
        https://download.docker.com/linux/ubuntu \
        $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
        tee /etc/apt/sources.list.d/docker.list > /dev/null

    # Install Docker Engine
    apt update > /dev/null 2>&1
    apt install -y docker-ce docker-ce-cli containerd.io \
        docker-buildx-plugin docker-compose-plugin > /dev/null 2>&1

    # Start Docker
    systemctl enable docker > /dev/null 2>&1
    systemctl start docker

    print_step "Docker installed: $(docker --version | cut -d' ' -f3 | tr -d ',')"
}

install_docker_compose() {
    print_info "Installing Docker Compose..."

    curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" \
        -o /usr/local/bin/docker-compose > /dev/null 2>&1
    chmod +x /usr/local/bin/docker-compose

    print_step "Docker Compose installed: $(docker-compose --version | cut -d' ' -f4 | tr -d ',')"
}

###############################################################################
# SCRDESK Installation
###############################################################################

install_scrdesk() {
    print_info "Installing SCRDESK server..."

    # Create directory
    mkdir -p $INSTALL_DIR
    cd $INSTALL_DIR

    # Create docker-compose.yml
    cat > docker-compose.yml << EOF
version: '3'

services:
  scrdesk-hbbs:
    container_name: scrdesk-hbbs
    image: rustdesk/rustdesk-server:latest
    command: hbbs -r $SERVER_IP:21117 -k _
    volumes:
      - ./data:/root
    network_mode: host
    restart: unless-stopped
    environment:
      - RUST_LOG=info
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

  scrdesk-hbbr:
    container_name: scrdesk-hbbr
    image: rustdesk/rustdesk-server:latest
    command: hbbr -k _
    volumes:
      - ./data:/root
    network_mode: host
    restart: unless-stopped
    environment:
      - RUST_LOG=info
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
EOF

    # Start services
    docker-compose up -d > /dev/null 2>&1

    # Wait for services to start
    sleep 5

    # Get public key
    if [ -f "$INSTALL_DIR/data/id_ed25519.pub" ]; then
        PUBLIC_KEY=$(cat $INSTALL_DIR/data/id_ed25519.pub)
        print_step "SCRDESK server installed"
        echo
        print_info "╔═══════════════════════════════════════════════════════════╗"
        print_info "║ IMPORTANT: Save this public key                          ║"
        print_info "╚═══════════════════════════════════════════════════════════╝"
        echo
        echo -e "${YELLOW}Public Key:${NC}"
        echo -e "${GREEN}$PUBLIC_KEY${NC}"
        echo
        echo "$PUBLIC_KEY" > $INSTALL_DIR/public_key.txt
        print_info "Key saved to: $INSTALL_DIR/public_key.txt"
    else
        print_error "Failed to get public key"
    fi
}

###############################################################################
# Nginx & SSL Installation
###############################################################################

install_nginx() {
    print_info "Installing Nginx..."
    apt install -y nginx > /dev/null 2>&1
    systemctl enable nginx > /dev/null 2>&1
    systemctl start nginx
    print_step "Nginx installed"
}

install_certbot() {
    print_info "Installing Certbot..."
    apt install -y certbot python3-certbot-nginx > /dev/null 2>&1
    print_step "Certbot installed"
}

setup_website() {
    print_info "Setting up website..."

    # Create web root
    mkdir -p /var/www/scrdesk

    # Create landing page
    cat > /var/www/scrdesk/index.html << 'EOF'
<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SCRDESK - Uzaktan Masaüstü Çözümü</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
        }
        .container {
            text-align: center;
            padding: 50px;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 20px;
            backdrop-filter: blur(10px);
            max-width: 600px;
        }
        h1 {
            font-size: 3em;
            margin-bottom: 20px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        p {
            font-size: 1.2em;
            margin: 20px 0;
            opacity: 0.9;
        }
        .download {
            margin: 30px 0;
        }
        .download a {
            display: inline-block;
            padding: 15px 40px;
            background: white;
            color: #667eea;
            text-decoration: none;
            border-radius: 50px;
            font-weight: bold;
            transition: transform 0.3s;
        }
        .download a:hover {
            transform: scale(1.05);
        }
        .info {
            background: rgba(0,0,0,0.2);
            padding: 20px;
            border-radius: 10px;
            margin-top: 30px;
        }
        .info code {
            background: rgba(0,0,0,0.3);
            padding: 5px 10px;
            border-radius: 5px;
            font-family: 'Courier New', monospace;
        }
        .status {
            display: inline-block;
            width: 10px;
            height: 10px;
            background: #4CAF50;
            border-radius: 50%;
            margin-right: 5px;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🖥️ SCRDESK</h1>
        <p>Güvenli ve Hızlı Uzaktan Masaüstü Erişimi</p>

        <div class="download">
            <a href="https://github.com/shosgoren/scrdesk/releases">Client İndir</a>
        </div>

        <div class="info">
            <p><span class="status"></span> Sunucu Aktif</p>
            <p style="font-size: 0.9em; margin-top: 15px;">Sunucu Adresi:</p>
            <p><code>SERVER_DOMAIN:21116</code></p>
        </div>

        <p style="font-size: 0.8em; margin-top: 30px; opacity: 0.7;">
            Powered by SCRDESK v1.0.0
        </p>
    </div>
</body>
</html>
EOF

    # Replace SERVER_DOMAIN with actual subdomain
    sed -i "s/SERVER_DOMAIN/$SUBDOMAIN.$DOMAIN/g" /var/www/scrdesk/index.html

    # Create Nginx config
    cat > /etc/nginx/sites-available/$DOMAIN << EOF
server {
    listen 80;
    server_name $DOMAIN www.$DOMAIN;

    root /var/www/scrdesk;
    index index.html;

    location / {
        try_files \$uri \$uri/ =404;
    }

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
EOF

    # Enable site
    ln -sf /etc/nginx/sites-available/$DOMAIN /etc/nginx/sites-enabled/
    rm -f /etc/nginx/sites-enabled/default

    # Test and reload
    nginx -t > /dev/null 2>&1
    systemctl reload nginx

    print_step "Website configured"
}

setup_ssl() {
    print_info "Setting up SSL certificate..."

    # Check if domain resolves
    if ! dig +short $DOMAIN | grep -q .; then
        print_warning "Domain does not resolve yet. Skipping SSL setup."
        print_info "Run this command later to setup SSL:"
        print_info "  certbot --nginx -d $DOMAIN -d www.$DOMAIN --non-interactive --agree-tos -m $EMAIL"
        return
    fi

    # Get certificate
    certbot --nginx -d $DOMAIN -d www.$DOMAIN \
        --non-interactive --agree-tos -m $EMAIL > /dev/null 2>&1

    if [ $? -eq 0 ]; then
        print_step "SSL certificate installed"
    else
        print_warning "SSL setup failed. You can run certbot manually later."
    fi
}

###############################################################################
# Additional Setup
###############################################################################

setup_fail2ban() {
    print_info "Configuring Fail2Ban..."

    cat > /etc/fail2ban/jail.local << 'EOF'
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true
port = 22
logpath = /var/log/auth.log

[nginx-http-auth]
enabled = true
port = http,https
logpath = /var/log/nginx/error.log
EOF

    systemctl enable fail2ban > /dev/null 2>&1
    systemctl restart fail2ban

    print_step "Fail2Ban configured"
}

setup_backup() {
    print_info "Setting up backup script..."

    cat > $INSTALL_DIR/backup.sh << 'EOF'
#!/bin/bash
BACKUP_DIR="/backup/scrdesk"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR

tar czf $BACKUP_DIR/scrdesk-backup-$DATE.tar.gz \
    /opt/scrdesk/data/id_ed25519* \
    /opt/scrdesk/docker-compose.yml

find $BACKUP_DIR -name "*.tar.gz" -mtime +30 -delete

echo "Backup completed: scrdesk-backup-$DATE.tar.gz"
EOF

    chmod +x $INSTALL_DIR/backup.sh

    # Add to crontab
    (crontab -l 2>/dev/null; echo "0 3 * * * $INSTALL_DIR/backup.sh") | crontab -

    print_step "Backup script installed (daily at 3:00 AM)"
}

###############################################################################
# Final Steps
###############################################################################

print_summary() {
    echo
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║${NC}  ${BLUE}Installation Complete!${NC}                               ${GREEN}║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════╝${NC}"
    echo
    print_step "SCRDESK server is running"
    echo
    echo -e "${YELLOW}Server Information:${NC}"
    echo "  Website:      https://$DOMAIN"
    echo "  ID Server:    $SUBDOMAIN.$DOMAIN:21116"
    echo "  Relay Server: $SUBDOMAIN.$DOMAIN:21117"
    echo "  Server IP:    $SERVER_IP"
    echo
    echo -e "${YELLOW}Public Key (save this!):${NC}"
    if [ -f "$INSTALL_DIR/public_key.txt" ]; then
        echo -e "${GREEN}$(cat $INSTALL_DIR/public_key.txt)${NC}"
    fi
    echo
    echo -e "${YELLOW}Client Configuration:${NC}"
    echo "  1. Download SCRDESK client from: https://github.com/shosgoren/scrdesk/releases"
    echo "  2. Open Settings → Network"
    echo "  3. Set ID Server: $SUBDOMAIN.$DOMAIN:21116"
    echo "  4. Set Relay Server: $SUBDOMAIN.$DOMAIN:21117"
    echo "  5. Set Key: (use the public key above)"
    echo
    echo -e "${YELLOW}Useful Commands:${NC}"
    echo "  View logs:      docker-compose -f $INSTALL_DIR/docker-compose.yml logs -f"
    echo "  Restart:        docker-compose -f $INSTALL_DIR/docker-compose.yml restart"
    echo "  Stop:           docker-compose -f $INSTALL_DIR/docker-compose.yml down"
    echo "  Start:          docker-compose -f $INSTALL_DIR/docker-compose.yml up -d"
    echo "  Backup:         $INSTALL_DIR/backup.sh"
    echo
    echo -e "${YELLOW}Files Location:${NC}"
    echo "  Install dir:    $INSTALL_DIR"
    echo "  Config:         $INSTALL_DIR/docker-compose.yml"
    echo "  Keys:           $INSTALL_DIR/data/id_ed25519*"
    echo "  Public key:     $INSTALL_DIR/public_key.txt"
    echo "  Website:        /var/www/scrdesk"
    echo
    print_info "🎉 Enjoy SCRDESK!"
    echo
}

###############################################################################
# Main Installation Flow
###############################################################################

main() {
    check_root
    preflight_checks

    echo
    print_info "Starting installation..."
    echo

    update_system
    install_dependencies
    setup_firewall

    install_docker
    install_docker_compose

    install_scrdesk

    install_nginx
    install_certbot
    setup_website
    setup_ssl

    setup_fail2ban
    setup_backup

    print_summary
}

# Run installation
main
