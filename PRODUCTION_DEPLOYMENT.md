# SCRDESK Production Deployment Guide
## scrdesk.com.tr için Üretim Ortamı Kurulumu

---

## 🖥️ VPS/VDS Gereksinimleri

### Minimum Özellikler (10-50 eşzamanlı kullanıcı)
- **CPU**: 2 vCPU
- **RAM**: 4 GB
- **Disk**: 40 GB SSD
- **Bandwidth**: 1 TB/ay
- **Network**: 100 Mbps
- **OS**: Ubuntu 22.04 LTS (önerilen)

### Önerilen Özellikler (50-200 eşzamanlı kullanıcı)
- **CPU**: 4 vCPU
- **RAM**: 8 GB
- **Disk**: 80 GB SSD
- **Bandwidth**: 2 TB/ay
- **Network**: 1 Gbps
- **OS**: Ubuntu 22.04 LTS

### Enterprise Özellikler (200+ eşzamanlı kullanıcı)
- **CPU**: 8+ vCPU
- **RAM**: 16+ GB
- **Disk**: 160+ GB SSD
- **Bandwidth**: 5+ TB/ay
- **Network**: 1+ Gbps
- **OS**: Ubuntu 22.04 LTS

---

## 💰 Türkiye VPS Sağlayıcıları ve Fiyatlar

### 1. **Turhost** (Önerilen - Türkiye)
**Web**: https://www.turhost.com/vds
**Konum**: İstanbul

| Plan | CPU | RAM | Disk | Fiyat/Ay |
|------|-----|-----|------|----------|
| Başlangıç | 2 vCPU | 4 GB | 40 GB SSD | ~300 TL |
| Pro | 4 vCPU | 8 GB | 80 GB SSD | ~600 TL |
| Enterprise | 8 vCPU | 16 GB | 160 GB SSD | ~1200 TL |

✅ Avantajlar:
- Türkiye lokasyonu (düşük latency)
- 7/24 Türkçe destek
- DDoS koruması
- Snapshot backup

### 2. **Hetzner** (Önerilen - Almanya)
**Web**: https://www.hetzner.com/cloud
**Konum**: Almanya (Falkenstein/Nürnberg)

| Plan | CPU | RAM | Disk | Fiyat/Ay |
|------|-----|-----|------|----------|
| CX22 | 2 vCPU | 4 GB | 40 GB SSD | €5.83 (~190 TL) |
| CX32 | 4 vCPU | 8 GB | 80 GB SSD | €11.39 (~370 TL) |
| CX42 | 8 vCPU | 16 GB | 160 GB SSD | €21.68 (~700 TL) |

✅ Avantajlar:
- Çok uygun fiyat
- Yüksek performans
- 20 TB bandwidth
- Snapshot & backup

### 3. **DigitalOcean**
**Web**: https://www.digitalocean.com/pricing
**Konum**: Frankfurt (Almanya)

| Plan | CPU | RAM | Disk | Fiyat/Ay |
|------|-----|-----|------|----------|
| Basic | 2 vCPU | 4 GB | 80 GB SSD | $24 (~790 TL) |
| Pro | 4 vCPU | 8 GB | 160 GB SSD | $48 (~1580 TL) |

### 4. **Linode (Akamai)**
**Web**: https://www.linode.com/pricing
**Konum**: Frankfurt

| Plan | CPU | RAM | Disk | Fiyat/Ay |
|------|-----|-----|------|----------|
| Linode 4GB | 2 vCPU | 4 GB | 80 GB SSD | $24 (~790 TL) |
| Linode 8GB | 4 vCPU | 8 GB | 160 GB SSD | $48 (~1580 TL) |

---

## 🎯 Öneri

### Başlangıç için: **Hetzner CX22** veya **Turhost Başlangıç**
- **Maliyet**: ~200-300 TL/ay
- **Performans**: 50 kullanıcıya kadar
- **Konum**: İhtiyaca göre (Türkiye veya Almanya)

### Neden Hetzner?
1. ✅ En uygun fiyat/performans
2. ✅ 20 TB bandwidth (DigitalOcean'da 4 TB)
3. ✅ Snapshot ücretsiz
4. ✅ IPv6 desteği
5. ✅ 99.9% uptime

---

## 🌐 Domain Yapılandırması (scrdesk.com.tr)

### DNS Kayıtları

Domain'inizi (scrdesk.com.tr) VPS IP adresinize yönlendirin:

```dns
# A Kayıtları
scrdesk.com.tr.        A      YOUR_VPS_IP
www.scrdesk.com.tr.    A      YOUR_VPS_IP
server.scrdesk.com.tr. A      YOUR_VPS_IP

# CNAME Kayıtları (Opsiyonel)
api.scrdesk.com.tr.    CNAME  scrdesk.com.tr.
admin.scrdesk.com.tr.  CNAME  scrdesk.com.tr.
```

### Subdomain Yapısı

```
scrdesk.com.tr              → Ana web sitesi (tanıtım, indirme linkleri)
server.scrdesk.com.tr       → SCRDESK sunucu servisleri
api.scrdesk.com.tr          → API endpoint (gelecek)
admin.scrdesk.com.tr        → Admin paneli (gelecek)
download.scrdesk.com.tr     → Client indirme sayfası
docs.scrdesk.com.tr         → Dokümantasyon
```

---

## 📦 Kurulum Adımları

### 1. VPS Kiralama ve İlk Kurulum

#### Hetzner Örneği:

```bash
# 1. Hetzner Cloud Console'a giriş yapın
# 2. "New Project" → "Add Server"
# 3. Location: Falkenstein (Almanya)
# 4. Image: Ubuntu 22.04
# 5. Type: CX22 (2 vCPU, 4GB RAM)
# 6. SSH Key: Ekleyin (mevcut id_ed25519.pub)
# 7. Server Name: scrdesk-production
# 8. Create & Start
```

#### İlk Bağlantı:

```bash
# VPS IP adresinizi alın (örnek: 95.217.123.456)
ssh root@YOUR_VPS_IP

# İlk güncelleme
apt update && apt upgrade -y
```

---

### 2. Domain DNS Yapılandırması

**scrdesk.com.tr** için DNS kayıtlarını ekleyin:

```
Tip: A
Host: @
Değer: YOUR_VPS_IP
TTL: 3600

Tip: A
Host: www
Değer: YOUR_VPS_IP
TTL: 3600

Tip: A
Host: server
Değer: YOUR_VPS_IP
TTL: 3600
```

DNS yayılımı: 10 dakika - 48 saat (genelde 1 saat)

**Test**:
```bash
# Local makinenizde
dig scrdesk.com.tr
nslookup server.scrdesk.com.tr
```

---

### 3. Sunucu Güvenlik Ayarları

```bash
# Firewall (UFW) kurulumu
apt install ufw -y

# SSH portunu aç
ufw allow 22/tcp

# SCRDESK portları
ufw allow 21115:21119/tcp
ufw allow 21116/udp

# HTTP/HTTPS (web için)
ufw allow 80/tcp
ufw allow 443/tcp

# Firewall'u etkinleştir
ufw enable

# Durum kontrolü
ufw status verbose
```

---

### 4. Docker Kurulumu

```bash
# Docker kurulum scripti
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Docker Compose kurulumu
curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
chmod +x /usr/local/bin/docker-compose

# Doğrulama
docker --version
docker-compose --version
```

---

### 5. SCRDESK Sunucu Kurulumu

```bash
# Dizin oluştur
mkdir -p /opt/scrdesk
cd /opt/scrdesk

# docker-compose.yml oluştur
cat > docker-compose.yml << 'EOF'
version: '3'

services:
  scrdesk-hbbs:
    container_name: scrdesk-hbbs
    image: rustdesk/rustdesk-server:latest
    command: hbbs -r YOUR_VPS_IP:21117 -k _
    volumes:
      - ./data:/root
    network_mode: host
    restart: unless-stopped
    environment:
      - RUST_LOG=info

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
EOF

# YOUR_VPS_IP'yi değiştir
VPS_IP=$(curl -s ifconfig.me)
sed -i "s/YOUR_VPS_IP/$VPS_IP/g" docker-compose.yml

# Servisleri başlat
docker-compose up -d

# Log kontrolü
docker-compose logs -f
```

---

### 6. SSL/TLS Sertifika Kurulumu (Let's Encrypt)

```bash
# Certbot kurulumu
apt install certbot python3-certbot-nginx -y

# Nginx kurulumu
apt install nginx -y

# Nginx yapılandırması
cat > /etc/nginx/sites-available/scrdesk.com.tr << 'EOF'
server {
    listen 80;
    server_name scrdesk.com.tr www.scrdesk.com.tr;

    root /var/www/scrdesk;
    index index.html;

    location / {
        try_files $uri $uri/ =404;
    }

    # API endpoint (gelecek için)
    location /api {
        proxy_pass http://localhost:21114;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }
}
EOF

# Web root oluştur
mkdir -p /var/www/scrdesk

# Basit landing page
cat > /var/www/scrdesk/index.html << 'EOF'
<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SCRDESK - Uzaktan Masaüstü Çözümü</title>
    <style>
        body { font-family: Arial, sans-serif; text-align: center; padding: 50px; }
        h1 { color: #2196F3; }
        .download { margin: 20px; }
        .download a { padding: 15px 30px; background: #2196F3; color: white; text-decoration: none; border-radius: 5px; }
    </style>
</head>
<body>
    <h1>SCRDESK</h1>
    <p>Güvenli ve Hızlı Uzaktan Masaüstü Erişimi</p>
    <div class="download">
        <a href="https://github.com/shosgoren/scrdesk/releases">Client İndir</a>
    </div>
    <p>Sunucu Adresi: <strong>server.scrdesk.com.tr:21116</strong></p>
</body>
</html>
EOF

# Site'ı etkinleştir
ln -s /etc/nginx/sites-available/scrdesk.com.tr /etc/nginx/sites-enabled/
nginx -t
systemctl restart nginx

# SSL sertifikası al
certbot --nginx -d scrdesk.com.tr -d www.scrdesk.com.tr

# Otomatik yenileme test
certbot renew --dry-run
```

---

### 7. SCRDESK Public Key Alma

```bash
# Public key'i görüntüle
cat /opt/scrdesk/data/id_ed25519.pub

# Örnek çıktı:
# BQNNA3NzaC1lZDI1NTE5AAAAIAl3k...
```

Bu key'i not edin - client'larda kullanılacak!

---

## 🔧 Client Yapılandırması

### Windows/macOS/Linux Client Ayarları:

```
ID Server: server.scrdesk.com.tr:21116
Relay Server: server.scrdesk.com.tr:21117
API Server: https://api.scrdesk.com.tr (opsiyonel)
Key: [Sunucudan aldığınız public key]
```

### Android/iOS Client Ayarları:

Settings → Custom Server:
```
Host: server.scrdesk.com.tr
ID/Relay Server: server.scrdesk.com.tr
Key: [Public key]
```

---

## 📊 İzleme ve Bakım

### Log İzleme

```bash
# Docker logları
docker-compose logs -f scrdesk-hbbs
docker-compose logs -f scrdesk-hbbr

# Nginx logları
tail -f /var/log/nginx/access.log
tail -f /var/log/nginx/error.log

# Sistem kaynakları
htop
df -h
```

### Performans İzleme

```bash
# Docker stats
docker stats

# Network trafiği
iftop

# Disk kullanımı
ncdu /opt/scrdesk
```

### Yedekleme

```bash
# Otomatik yedekleme scripti
cat > /opt/scrdesk/backup.sh << 'EOF'
#!/bin/bash
BACKUP_DIR="/backup/scrdesk"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR

# Keys ve data yedekleme
tar czf $BACKUP_DIR/scrdesk-backup-$DATE.tar.gz \
  /opt/scrdesk/data/id_ed25519* \
  /opt/scrdesk/docker-compose.yml

# Eski yedekleri sil (30 günden eski)
find $BACKUP_DIR -name "*.tar.gz" -mtime +30 -delete

echo "Backup completed: scrdesk-backup-$DATE.tar.gz"
EOF

chmod +x /opt/scrdesk/backup.sh

# Cron job ekle (günlük 3:00)
echo "0 3 * * * /opt/scrdesk/backup.sh" | crontab -
```

---

## 🚀 Güncelleme

```bash
cd /opt/scrdesk

# Yeni imajları çek
docker-compose pull

# Servisleri yeniden başlat
docker-compose down
docker-compose up -d

# Logları kontrol et
docker-compose logs -f
```

---

## 🔒 Güvenlik İyileştirmeleri

### 1. Fail2Ban Kurulumu

```bash
apt install fail2ban -y

cat > /etc/fail2ban/jail.local << 'EOF'
[sshd]
enabled = true
port = 22
maxretry = 3
bantime = 3600

[nginx-http-auth]
enabled = true
port = http,https
logpath = /var/log/nginx/error.log
maxretry = 5
EOF

systemctl restart fail2ban
fail2ban-client status
```

### 2. SSH Güvenlik

```bash
# /etc/ssh/sshd_config düzenle
sed -i 's/#PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config
sed -i 's/#PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config

# Yeni admin kullanıcı oluştur
adduser scradmin
usermod -aG sudo scradmin

# SSH key'i scradmin'e kopyala
mkdir -p /home/scradmin/.ssh
cp ~/.ssh/authorized_keys /home/scradmin/.ssh/
chown -R scradmin:scradmin /home/scradmin/.ssh
chmod 700 /home/scradmin/.ssh
chmod 600 /home/scradmin/.ssh/authorized_keys

# SSH restart
systemctl restart sshd
```

### 3. Automatic Security Updates

```bash
apt install unattended-upgrades -y
dpkg-reconfigure -plow unattended-upgrades
```

---

## 📈 Ölçeklendirme (Scaling)

### Load Balancer ile Çoklu Sunucu

```nginx
# Nginx load balancer
upstream scrdesk_backend {
    least_conn;
    server 10.0.0.1:21116;
    server 10.0.0.2:21116;
    server 10.0.0.3:21116;
}

server {
    listen 21116;
    proxy_pass scrdesk_backend;
}
```

---

## 💰 Maliyet Hesaplaması (Aylık)

### Başlangıç Senaryosu:
```
VPS (Hetzner CX22):      €5.83  (~190 TL)
Domain (.com.tr):        ~50 TL/yıl (4 TL/ay)
SSL Sertifikası:         ÜCRETSİZ (Let's Encrypt)
Backup Storage (20GB):   ~30 TL
------------------------
TOPLAM:                  ~220 TL/ay
```

### Profesyonel Senaryosu:
```
VPS (Hetzner CX32):      €11.39 (~370 TL)
Domain:                  ~4 TL/ay
SSL:                     ÜCRETSİZ
Backup Storage (50GB):   ~75 TL
DDoS Protection:         ~150 TL (opsiyonel)
------------------------
TOPLAM:                  ~600 TL/ay
```

---

## 🎯 Kurulum Sonrası Kontrol Listesi

- [ ] VPS kuruldu ve erişilebilir
- [ ] Domain DNS kayıtları ayarlandı
- [ ] Firewall yapılandırıldı
- [ ] Docker ve Docker Compose kuruldu
- [ ] SCRDESK servisleri çalışıyor
- [ ] SSL sertifikası yüklendi (HTTPS)
- [ ] Web sitesi erişilebilir (scrdesk.com.tr)
- [ ] Public key alındı ve saklandı
- [ ] Test client bağlantısı başarılı
- [ ] Backup scripti çalışıyor
- [ ] Monitoring kuruldu
- [ ] Fail2Ban aktif

---

## 📞 Destek ve Kaynaklar

- **SCRDESK Repository**: https://github.com/shosgoren/scrdesk
- **RustDesk Docs**: https://rustdesk.com/docs/
- **Hetzner Status**: https://status.hetzner.com/
- **Let's Encrypt Status**: https://letsencrypt.status.io/

---

**Kurulum Tarihi**: 2025-11-17
**Son Güncelleme**: 2025-11-17
**Versiyon**: 1.0.0-production
