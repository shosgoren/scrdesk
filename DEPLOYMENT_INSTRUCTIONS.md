# SCRDESK Kurulum Talimatları
## scrdesk.com.tr için VPS Deployment

---

## 📋 VPS Bilgileriniz

```
VPS IP:      72.61.138.218
Hostname:    srv1123022.hstgr.cloud
Domain:      scrdesk.com.tr
```

---

## 🚀 Hızlı Kurulum (Tavsiye Edilen)

### Adım 1: VPS'ye Bağlanın

Terminal açın ve aşağıdaki komutu çalıştırın:

```bash
ssh root@72.61.138.218
```

**Şifre**: `johqid-4Cuhxi-nugvov`

### Adım 2: Otomatik Kurulum Komutunu Çalıştırın

VPS'ye bağlandıktan sonra, aşağıdaki komutu **kopyalayıp yapıştırın**:

```bash
curl -fsSL https://raw.githubusercontent.com/shosgoren/scrdesk/main/scripts/install-production.sh | \
  SCRDESK_DOMAIN=scrdesk.com.tr \
  SCRDESK_EMAIL=admin@scrdesk.com.tr \
  bash
```

**Süre**: ~10-15 dakika

Kurulum sırasında ekranda ilerleme göreceksiniz:
```
[✓] System updated
[✓] Dependencies installed
[✓] Docker installed
[✓] SCRDESK server installed
...
```

### Adım 3: Public Key'i Kaydedin

Kurulum bittiğinde şuna benzer bir çıktı göreceksiniz:

```
╔═══════════════════════════════════════════════════════════╗
║ IMPORTANT: Save this public key                          ║
╚═══════════════════════════════════════════════════════════╝

Public Key:
AAAAC3NzaC1lZDI1NTE5AAAAIBxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

**ÖNEMLİ**: Bu key'i not edin! Client yapılandırmasında kullanacaksınız.

---

## 🌐 DNS Yapılandırması

Kurulum devam ederken, domain'inizin DNS ayarlarını yapın.

### nic.tr veya Domain Yöneticinizde:

A kayıtlarını ekleyin:

| Host/Name | Tip | Değer | TTL |
|-----------|-----|-------|-----|
| @ veya scrdesk.com.tr | A | 72.61.138.218 | 3600 |
| www | A | 72.61.138.218 | 3600 |
| server | A | 72.61.138.218 | 3600 |

### Örnek: nic.tr

1. https://nic.tr → Giriş Yap
2. Domain Yönetimi → scrdesk.com.tr
3. DNS Yönetimi
4. Yeni Kayıt Ekle:
   ```
   Kayıt Tipi: A
   Host: @
   IP Adresi: 72.61.138.218
   TTL: 3600
   ```
5. Tekrar et: `www` ve `server` için

**DNS Yayılım Süresi**: 10 dakika - 2 saat

### DNS Kontrolü

Terminal'de test edin:

```bash
# Local makinenizde
dig scrdesk.com.tr
nslookup server.scrdesk.com.tr

# Doğru IP'yi (72.61.138.218) görmeli
```

---

## ✅ Kurulum Sonrası Kontroller

### 1. Website Kontrolü

Tarayıcıda açın:
```
http://scrdesk.com.tr
```

Landing page'i görmelisiniz.

### 2. HTTPS Kontrolü

DNS yayıldıktan sonra:
```
https://scrdesk.com.tr
```

Yeşil kilit ikonu görmelisiniz (Let's Encrypt SSL).

### 3. SCRDESK Server Kontrolü

VPS'de:

```bash
# Logları görüntüle
docker-compose -f /opt/scrdesk/docker-compose.yml logs -f

# Servis durumu
docker ps

# Şu iki container çalışıyor olmalı:
# - scrdesk-hbbs
# - scrdesk-hbbr
```

---

## 🖥️ Client Yapılandırması

### Windows/macOS/Linux

1. **SCRDESK Client İndir**:
   - https://github.com/rustdesk/rustdesk/releases/latest
   - Veya kendi build'inizi yapın

2. **Client'ı Açın**

3. **Settings (⚙️) → Network**

4. **Ayarları Girin**:
   ```
   ID Server:    server.scrdesk.com.tr:21116
   Relay Server: server.scrdesk.com.tr:21117
   API Server:   (boş bırakın)
   Key:          [Kurulumdan aldığınız public key]
   ```

5. **Apply** ve client'ı yeniden başlatın

### Android

1. SCRDESK/RustDesk APK indir
2. Ayarlar → Custom Server
3. Aynı bilgileri girin

### iOS

1. App Store'dan RustDesk indir (kısıtlı özellikler)
2. Settings → Server
3. Aynı bilgileri girin

---

## 🔧 Yönetim Komutları

### VPS'ye Bağlanma

```bash
ssh root@72.61.138.218
```

### SCRDESK Servisleri Yönetimi

```bash
# Tüm logları görüntüle
docker-compose -f /opt/scrdesk/docker-compose.yml logs -f

# Sadece hbbs logları
docker logs -f scrdesk-hbbs

# Sadece hbbr logları
docker logs -f scrdesk-hbbr

# Servisleri yeniden başlat
docker-compose -f /opt/scrdesk/docker-compose.yml restart

# Servisleri durdur
docker-compose -f /opt/scrdesk/docker-compose.yml down

# Servisleri başlat
docker-compose -f /opt/scrdesk/docker-compose.yml up -d

# Container durumu
docker ps

# Sistem kaynakları
docker stats
```

### Public Key Görüntüleme

```bash
cat /opt/scrdesk/public_key.txt
# veya
cat /opt/scrdesk/data/id_ed25519.pub
```

### Yedekleme

```bash
# Manuel yedekleme
/opt/scrdesk/backup.sh

# Yedekleri görüntüle
ls -lh /backup/scrdesk/

# Yedekleri local'e indir
scp root@72.61.138.218:/backup/scrdesk/*.tar.gz ~/Desktop/
```

### SSL Sertifika Yenileme

```bash
# Otomatik yenileme test
certbot renew --dry-run

# Manuel yenileme
certbot renew

# Sertifika bilgileri
certbot certificates
```

---

## 🐛 Sorun Giderme

### Problem: "DNS çözümlemiyor"

**Çözüm**:
```bash
# DNS kontrolü
dig scrdesk.com.tr

# Eğer IP göstermiyorsa:
# - DNS kayıtlarını kontrol edin
# - 1-2 saat bekleyin (yayılım)
# - Başka bir DNS kullanın: 8.8.8.8
```

### Problem: "SSL sertifikası alınamadı"

**Çözüm**:
```bash
# DNS'in çözümlendiğinden emin olun
dig scrdesk.com.tr

# Manuel SSL kurulumu
certbot --nginx -d scrdesk.com.tr -d www.scrdesk.com.tr \
  --non-interactive --agree-tos -m admin@scrdesk.com.tr
```

### Problem: "Client bağlanamıyor"

**Kontrol Listesi**:
```bash
# 1. Servislerin çalıştığından emin olun
docker ps

# 2. Firewall portlarını kontrol edin
ufw status

# 3. Logları inceleyin
docker-compose -f /opt/scrdesk/docker-compose.yml logs

# 4. Port dinleme kontrolü
netstat -tulpn | grep -E '21115|21116|21117'
```

### Problem: "Container başlamıyor"

**Çözüm**:
```bash
# Container loglarını incele
docker logs scrdesk-hbbs
docker logs scrdesk-hbbr

# Yeniden başlat
docker-compose -f /opt/scrdesk/docker-compose.yml restart

# Tamamen yeniden oluştur
docker-compose -f /opt/scrdesk/docker-compose.yml down
docker-compose -f /opt/scrdesk/docker-compose.yml up -d
```

---

## 📊 Monitoring

### Sistem Kaynakları

```bash
# CPU, RAM, Disk
htop

# Disk kullanımı
df -h

# Network trafiği
iftop

# Docker stats
docker stats
```

### Log Analizi

```bash
# Son 100 log satırı
docker-compose -f /opt/scrdesk/docker-compose.yml logs --tail=100

# Belirli tarih
docker-compose -f /opt/scrdesk/docker-compose.yml logs --since="2025-11-17T10:00:00"

# Hata logları
docker-compose -f /opt/scrdesk/docker-compose.yml logs | grep -i error
```

---

## 🔄 Güncelleme

### SCRDESK Server Güncelleme

```bash
cd /opt/scrdesk

# Yeni imajları çek
docker-compose pull

# Servisleri güncelle
docker-compose down
docker-compose up -d

# Logları kontrol et
docker-compose logs -f
```

### Sistem Güncelleme

```bash
# Paket güncellemeleri
apt update
apt upgrade -y

# Yeniden başlatma gerekirse
reboot
```

---

## 📞 Destek

### Dokümantasyon
- **Genel Rehber**: [PRODUCTION_DEPLOYMENT.md](PRODUCTION_DEPLOYMENT.md)
- **Hızlı Başlangıç**: [HOSTING_QUICKSTART.md](HOSTING_QUICKSTART.md)
- **Build Rehberi**: [BUILD_GUIDE.md](BUILD_GUIDE.md)

### GitHub
- **Repository**: https://github.com/shosgoren/scrdesk
- **Issues**: https://github.com/shosgoren/scrdesk/issues
- **Releases**: https://github.com/shosgoren/scrdesk/releases

### RustDesk Topluluk
- **Docs**: https://rustdesk.com/docs/
- **Discord**: https://discord.gg/nDceKgxnkV

---

## 📝 Önemli Notlar

1. **Public Key'i Saklayin**: Kaybederseniz server'ı yeniden kurmanız gerekir
2. **Düzenli Yedek**: Backup scripti günlük çalışıyor (/backup/scrdesk/)
3. **SSL Otomatik**: Let's Encrypt 90 günde bir otomatik yenilenir
4. **Güvenlik**: Root login sonradan devre dışı bırakılabilir
5. **Performans**: 50+ kullanıcı için VPS upgrade edin

---

## ✅ Kontrol Listesi

- [ ] VPS'ye SSH ile bağlandım
- [ ] Kurulum scriptini çalıştırdım
- [ ] Public key'i kaydettim
- [ ] DNS A kayıtlarını ekledim
- [ ] DNS yayılımını bekledim
- [ ] https://scrdesk.com.tr açılıyor
- [ ] Client'ı yapılandırdım
- [ ] Test bağlantısı başarılı
- [ ] Yedekleme çalışıyor

---

**🎉 Kurulum tamamlandı! SCRDESK artık scrdesk.com.tr üzerinde yayında!**
