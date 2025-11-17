# SCRDESK Hosting - Hızlı Başlangıç
## scrdesk.com.tr için 15 Dakikada Kurulum

---

## 🚀 En Hızlı Yöntem (Tek Komut)

### Adım 1: VPS Kiralayın

**Önerilen**: Hetzner CX22 (€5.83/ay ~ 190 TL/ay)
- 2 vCPU, 4 GB RAM, 40 GB SSD
- https://www.hetzner.com/cloud

### Adım 2: VPS'ye Bağlanın

```bash
ssh root@YOUR_VPS_IP
```

### Adım 3: Otomatik Kurulum

```bash
# Kurulum scriptini indir ve çalıştır
curl -fsSL https://raw.githubusercontent.com/shosgoren/scrdesk/main/scripts/install-production.sh | \
  SCRDESK_DOMAIN=scrdesk.com.tr \
  SCRDESK_EMAIL=admin@scrdesk.com.tr \
  bash
```

**İşlem süresi**: ~10-15 dakika

### Adım 4: DNS Ayarları

Domain yöneticinizde (örn: nic.tr) A kayıtları ekleyin:

```
@ (veya scrdesk.com.tr)     A    YOUR_VPS_IP
www                          A    YOUR_VPS_IP
server                       A    YOUR_VPS_IP
```

**Bekleme süresi**: 10 dakika - 2 saat

### Adım 5: Client'ı Yapılandır

Kurulum bitince görüntülenen **Public Key**'i not alın.

Windows/Mac/Linux Client Ayarları:
```
ID Server: server.scrdesk.com.tr:21116
Relay Server: server.scrdesk.com.tr:21117
Key: [Kurulumdan aldığınız public key]
```

---

## ✅ İşte Bu Kadar!

Website: https://scrdesk.com.tr
Client Download: https://github.com/shosgoren/scrdesk/releases

---

## 🔧 Manuel Kurulum

Detaylı adım adım kurulum için: [PRODUCTION_DEPLOYMENT.md](PRODUCTION_DEPLOYMENT.md)

---

## 💰 Aylık Maliyet

```
VPS:        190 TL/ay (Hetzner CX22)
Domain:     50 TL/yıl (4 TL/ay)
SSL:        ÜCRETSİZ (Let's Encrypt)
---------------------------------
TOPLAM:     ~195 TL/ay
```

---

## 📊 VPS Karşılaştırması

| Sağlayıcı | CPU | RAM | Disk | Fiyat/Ay | Link |
|-----------|-----|-----|------|----------|------|
| **Hetzner** ⭐ | 2 | 4 GB | 40 GB | ~190 TL | [Link](https://hetzner.com/cloud) |
| Turhost | 2 | 4 GB | 40 GB | ~300 TL | [Link](https://turhost.com/vds) |
| DigitalOcean | 2 | 4 GB | 80 GB | ~790 TL | [Link](https://digitalocean.com) |

**Öneri**: Hetzner (en iyi fiyat/performans)

---

## 🆘 Sorun mu Yaşıyorsunuz?

### Kurulum Hatası
```bash
# Logları kontrol et
docker-compose -f /opt/scrdesk/docker-compose.yml logs -f
```

### DNS Henüz Yayılmadı
```bash
# Kontrol et
dig scrdesk.com.tr
nslookup server.scrdesk.com.tr
```

### SSL Sertifika Hatası
```bash
# Manuel olarak SSL ekle
certbot --nginx -d scrdesk.com.tr -d www.scrdesk.com.tr
```

---

## 📞 Destek

- GitHub Issues: https://github.com/shosgoren/scrdesk/issues
- Dokümantasyon: [PRODUCTION_DEPLOYMENT.md](PRODUCTION_DEPLOYMENT.md)
- Server Kurulum: [SERVER_SETUP.md](SERVER_SETUP.md)

---

**⚡ Hızlı, Kolay, Güvenli - SCRDESK**
