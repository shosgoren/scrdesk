# SCRDESK Hızlı Başlangıç Kılavuzu

SCRDESK'e hoş geldiniz! Bu kılavuz, projeyi en hızlı şekilde çalıştırmanıza yardımcı olacak.

## 🚀 5 Dakikada Başlayın

### Adım 1: VPS Sunucu Kurulumu

Bir VPS sağlayıcısından sunucu kiralayın (DigitalOcean, Hetzner, Linode, vb.)

**Minimum Gereksinimler:**
- 2GB RAM
- 1 CPU Core
- Ubuntu 22.04

### Adım 2: Sunucuya Bağlanın

```bash
ssh root@your-server-ip
```

### Adım 3: Docker ile SCRDESK Sunucusunu Kurun

```bash
# Docker kur
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Docker Compose kur
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# SCRDESK dizini oluştur
mkdir -p ~/scrdesk-server
cd ~/scrdesk-server

# docker-compose.yml oluştur
cat > docker-compose.yml << 'EOF'
version: '3'

services:
  scrdesk-hbbs:
    container_name: scrdesk-hbbs
    image: rustdesk/rustdesk-server:latest
    command: hbbs -r YOUR_SERVER_IP:21117
    volumes:
      - ./data:/root
    network_mode: host
    restart: unless-stopped

  scrdesk-hbbr:
    container_name: scrdesk-hbbr
    image: rustdesk/rustdesk-server:latest
    command: hbbr
    volumes:
      - ./data:/root
    network_mode: host
    restart: unless-stopped
EOF

# YOUR_SERVER_IP'yi değiştir (örn: sed komutu ile)
sed -i "s/YOUR_SERVER_IP/$(curl -s ifconfig.me)/g" docker-compose.yml

# Servisleri başlat
docker-compose up -d

# Public key'i al
cat ./data/id_ed25519.pub
```

**ÖNEMLİ:** `cat ./data/id_ed25519.pub` komutunun çıktısını not edin!

### Adım 4: Firewall'u Açın

```bash
# UFW kullanıyorsanız
sudo ufw allow 21115:21119/tcp
sudo ufw allow 21116/udp
sudo ufw reload

# Veya FirewallD
sudo firewall-cmd --permanent --add-port=21115-21119/tcp
sudo firewall-cmd --permanent --add-port=21116/udp
sudo firewall-cmd --reload
```

### Adım 5: Client İndir ve Yapılandır

#### Windows
1. [RustDesk Windows Client](https://github.com/rustdesk/rustdesk/releases/latest) indir
2. Kur ve çalıştır
3. Ayarlar ⚙️ → Network → ID Server: `YOUR_SERVER_IP:21116`
4. Relay Server: `YOUR_SERVER_IP:21117`
5. Key: (Adım 3'te aldığınız public key)

#### macOS
1. [RustDesk macOS Client](https://github.com/rustdesk/rustdesk/releases/latest) indir
2. Kur ve çalıştır
3. Aynı yapılandırmayı uygula

#### Linux
```bash
# Debian/Ubuntu
wget https://github.com/rustdesk/rustdesk/releases/latest/download/rustdesk-x.x.x-x86_64.deb
sudo dpkg -i rustdesk-*.deb

# Arch
yay -S rustdesk

# Flatpak
flatpak install flathub com.rustdesk.RustDesk
```

#### Android
1. [F-Droid](https://f-droid.org/packages/com.carriez.flutter_hbb/) veya [GitHub](https://github.com/rustdesk/rustdesk/releases) üzerinden APK indir
2. Yükle
3. Ayarlar → Custom Server
4. Sunucu bilgilerini gir

#### iOS
- App Store'dan indir (sınırlı özellikler)
- Veya TestFlight beta sürümü kullan

---

## ✅ Test Edin

1. İki farklı cihaza SCRDESK client kurun
2. Her ikisinde de sunucu ayarlarını yapın
3. Bir cihazın ID'sini diğerine girin
4. Bağlanın!

---

## 🔒 Güvenlik Ayarları

### Public Key Zorunlu Kılma

Sunucu yapılandırmasını güncelleyin:

```yaml
# docker-compose.yml
services:
  scrdesk-hbbs:
    command: hbbs -r YOUR_SERVER_IP:21117 -k _
```

Sonra yeniden başlatın:
```bash
docker-compose restart
```

### SSL/TLS (Opsiyonel ama önerilir)

Nginx reverse proxy ile:

```bash
# Certbot kur
sudo apt install certbot python3-certbot-nginx

# SSL sertifikası al
sudo certbot --nginx -d scrdesk.yourdomain.com

# Nginx yapılandırması
sudo nano /etc/nginx/sites-available/scrdesk
```

Yapılandırma:
```nginx
server {
    listen 443 ssl;
    server_name scrdesk.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/scrdesk.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/scrdesk.yourdomain.com/privkey.pem;

    location / {
        proxy_pass http://localhost:21114;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

---

## 📊 Monitoring

### Logları İzleme

```bash
# Canlı loglar
docker-compose logs -f

# Sadece hbbs
docker logs -f scrdesk-hbbs

# Sadece hbbr
docker logs -f scrdesk-hbbr
```

### Sistem Durumu

```bash
# Container'lar çalışıyor mu?
docker ps

# Portlar açık mı?
sudo netstat -tulpn | grep -E '21115|21116|21117'

# Dışarıdan erişim testi
telnet YOUR_SERVER_IP 21116
```

---

## 🛠️ Sorun Giderme

### Bağlantı Problemi

**Client "ID server offline" hatası veriyor:**
```bash
# Sunucuda servisleri kontrol et
docker ps
docker-compose logs

# Firewall kontrolü
sudo ufw status
```

**"Relay server error":**
```bash
# hbbr servisini yeniden başlat
docker restart scrdesk-hbbr

# Logları incele
docker logs scrdesk-hbbr
```

### Performans Problemi

```bash
# Kaynak kullanımını kontrol et
docker stats

# Bellek yetersizse upgrade yapın veya swap ekleyin
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

---

## 📚 İleri Seviye

Daha detaylı bilgi için:

- **Sunucu Kurulumu**: [SERVER_SETUP.md](SERVER_SETUP.md)
- **Build Rehberi**: [BUILD_GUIDE.md](BUILD_GUIDE.md)
- **Kaynak Kod**: [src/](src/)

---

## 🆘 Yardım

- GitHub Issues: Sorun bildirin
- Dokümantasyon: Wiki sayfalarını kontrol edin
- RustDesk Community: [Discord](https://discord.gg/nDceKgxnkV)

---

## 🎯 Sonraki Adımlar

1. ✅ Sunucuyu kurdunuz
2. ✅ Client'ları yapılandırdınız
3. ⬜ Kaynak koddan kendi versiyonunuzu build edin
4. ⬜ Logo ve branding'i özelleştirin
5. ⬜ Mobil uygulamaları build edin

**Kolay gelsin! 🚀**
