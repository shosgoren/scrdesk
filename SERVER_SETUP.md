# SCRDESK VPS Server Kurulum Rehberi

Bu rehber, SCRDESK'i kendi VPS sunucunuzda nasıl kuracağınızı adım adım gösterir.

## Gereksinimler

### Minimum Sunucu Özellikleri
- **RAM**: 1GB (2GB+ önerilir)
- **CPU**: 1 Core (2+ önerilir)
- **Disk**: 10GB
- **OS**: Ubuntu 20.04/22.04, Debian 10/11, CentOS 7/8
- **Network**: Statik IP adresi

### Açık Olması Gereken Portlar
```
TCP: 21115, 21116, 21117, 21118, 21119
UDP: 21116
```

## Kurulum Yöntemleri

### Yöntem 1: Docker ile Kurulum (Önerilen)

#### 1. Docker Kurulumu

```bash
# Docker kurulum scripti
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Docker Compose kurulumu
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
```

#### 2. SCRDESK Sunucu Dosyalarını Oluşturma

`docker-compose.yml` dosyası oluşturun:

```yaml
version: '3'

networks:
  scrdesk-net:
    external: false

services:
  hbbs:
    container_name: scrdesk-hbbs
    image: rustdesk/rustdesk-server:latest
    command: hbbs -r your-server-ip:21117
    volumes:
      - ./data:/root
    network_mode: host
    restart: unless-stopped

  hbbr:
    container_name: scrdesk-hbbr
    image: rustdesk/rustdesk-server:latest
    command: hbbr
    volumes:
      - ./data:/root
    network_mode: host
    restart: unless-stopped
```

**ÖNEMLİ**: `your-server-ip` kısmını VPS'inizin gerçek IP adresi ile değiştirin!

#### 3. Sunucuyu Başlatma

```bash
# Dizin oluştur
mkdir -p ~/scrdesk-server
cd ~/scrdesk-server

# docker-compose.yml dosyasını kaydet (yukarıdaki içeriği kullan)

# Servisleri başlat
docker-compose up -d

# Logları kontrol et
docker-compose logs -f
```

#### 4. Erişim Anahtarını Alma

```bash
# Public key'i görüntüle
cat ./data/id_ed25519.pub
```

Bu anahtarı not edin, client yapılandırmasında kullanacaksınız.

---

### Yöntem 2: Manuel Kurulum

#### Ubuntu/Debian için

```bash
# Sunucu dosyalarını indir
cd /opt
sudo wget https://github.com/rustdesk/rustdesk-server/releases/latest/download/rustdesk-server-linux-amd64.zip

# Zip'i aç
sudo apt install unzip -y
sudo unzip rustdesk-server-linux-amd64.zip
cd rustdesk-server-linux-amd64
```

#### Systemd Servisi Oluşturma

**hbbs servisi** (`/etc/systemd/system/scrdesk-hbbs.service`):

```ini
[Unit]
Description=SCRDESK ID/Rendezvous Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/rustdesk-server-linux-amd64
ExecStart=/opt/rustdesk-server-linux-amd64/hbbs -r YOUR_SERVER_IP:21117
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

**hbbr servisi** (`/etc/systemd/system/scrdesk-hbbr.service`):

```ini
[Unit]
Description=SCRDESK Relay Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/opt/rustdesk-server-linux-amd64
ExecStart=/opt/rustdesk-server-linux-amd64/hbbr
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

#### Servisleri Etkinleştirme

```bash
# Servisleri yükle
sudo systemctl daemon-reload

# Başlat ve etkinleştir
sudo systemctl enable scrdesk-hbbs
sudo systemctl enable scrdesk-hbbr
sudo systemctl start scrdesk-hbbs
sudo systemctl start scrdesk-hbbr

# Durum kontrolü
sudo systemctl status scrdesk-hbbs
sudo systemctl status scrdesk-hbbr
```

---

## Firewall Ayarları

### UFW (Ubuntu/Debian)

```bash
sudo ufw allow 21115:21119/tcp
sudo ufw allow 21116/udp
sudo ufw reload
```

### FirewallD (CentOS/RHEL)

```bash
sudo firewall-cmd --permanent --add-port=21115-21119/tcp
sudo firewall-cmd --permanent --add-port=21116/udp
sudo firewall-cmd --reload
```

---

## Client Yapılandırması

### Windows/macOS/Linux Client

1. SCRDESK client'ı çalıştırın
2. Ayarlar (⚙️) → Network menüsüne gidin
3. Aşağıdaki bilgileri girin:

```
ID Server: YOUR_SERVER_IP:21116
Relay Server: YOUR_SERVER_IP:21117
API Server: http://YOUR_SERVER_IP:21114
Key: [Sunucudan aldığınız public key]
```

### Android/iOS Client

Mobil uygulamalarda:
1. Ayarlar → Network Settings
2. Sunucu bilgilerini aynı şekilde girin

---

## Güvenlik Önerileri

### 1. Public Key Şifrelemeyi Etkinleştir

Sunucunuzda public key zorunlu kılın:

```bash
# hbbs servisini key ile başlat
./hbbs -r YOUR_SERVER_IP:21117 -k _
```

### 2. Reverse Proxy ile SSL (Nginx)

```nginx
server {
    listen 443 ssl;
    server_name scrdesk.yourdomain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:21114;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
    }
}
```

### 3. Fail2Ban Kurulumu

```bash
sudo apt install fail2ban -y

# /etc/fail2ban/jail.local
[scrdesk]
enabled = true
port = 21115:21119
filter = scrdesk
logpath = /var/log/scrdesk/*.log
maxretry = 5
bantime = 3600
```

---

## Sorun Giderme

### Logları İnceleme

**Docker:**
```bash
docker logs scrdesk-hbbs
docker logs scrdesk-hbbr
```

**Systemd:**
```bash
sudo journalctl -u scrdesk-hbbs -f
sudo journalctl -u scrdesk-hbbr -f
```

### Port Kontrolü

```bash
# Portların açık olduğunu doğrula
sudo netstat -tulpn | grep -E '21115|21116|21117|21118|21119'

# Dışarıdan erişim testi
telnet YOUR_SERVER_IP 21116
```

### Yaygın Hatalar

**Hata: "Connection refused"**
- Firewall ayarlarını kontrol edin
- Servislerin çalıştığını doğrulayın: `systemctl status scrdesk-hbbs`

**Hata: "ID server not reachable"**
- IP adresinin doğru olduğundan emin olun
- 21116 portuna erişilebildiğini test edin

**Hata: "Relay server error"**
- hbbr servisinin çalıştığını kontrol edin
- 21117 portunu kontrol edin

---

## Performans İyileştirme

### Sistem Limitleri

`/etc/security/limits.conf` dosyasına ekleyin:

```
* soft nofile 65535
* hard nofile 65535
```

### Kernel Parametreleri

`/etc/sysctl.conf`:

```
net.ipv4.tcp_fin_timeout = 30
net.ipv4.tcp_keepalive_time = 300
net.core.rmem_max = 16777216
net.core.wmem_max = 16777216
```

Uygula:
```bash
sudo sysctl -p
```

---

## Yedekleme

Önemli dosyalar:

```bash
# Public/Private key çifti
/opt/rustdesk-server-linux-amd64/id_ed25519
/opt/rustdesk-server-linux-amd64/id_ed25519.pub

# Docker kullanıyorsanız
./data/id_ed25519
./data/id_ed25519.pub
```

Yedekleme scripti:

```bash
#!/bin/bash
BACKUP_DIR="/backups/scrdesk"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p $BACKUP_DIR
cp -r /opt/rustdesk-server-linux-amd64/id_ed25519* $BACKUP_DIR/keys_$DATE/
```

---

## Güncelleme

### Docker

```bash
docker-compose pull
docker-compose up -d
```

### Manuel

```bash
cd /opt
sudo wget https://github.com/rustdesk/rustdesk-server/releases/latest/download/rustdesk-server-linux-amd64.zip
sudo unzip -o rustdesk-server-linux-amd64.zip
sudo systemctl restart scrdesk-hbbs scrdesk-hbbr
```

---

## Destek ve Topluluk

- GitHub Issues: [SCRDESK Repository]
- Dokümantasyon: [Project Wiki]

**Not**: SCRDESK, RustDesk'in fork'udur. RustDesk dokümantasyonu da faydalı olabilir.
