# OAuth Configuration Guide

OAuth entegrasyonu artık hazır! Aşağıdaki adımları izleyerek Google ve Apple ile giriş yapabilirsiniz.

## 🔧 Kurulum Adımları

### 1. Google OAuth Kurulumu

1. **Google Cloud Console**'a gidin: https://console.cloud.google.com/apis/credentials
2. Yeni bir proje oluşturun veya mevcut projeyi seçin
3. **"OAuth 2.0 Client IDs"** → **"Create Credentials"** → **"OAuth client ID"**
4. Application type: **Web application**
5. **Authorized redirect URIs** ekleyin:
   ```
   https://scrdesk.com/auth/oauth/callback?provider=google
   ```
6. Client ID ve Client Secret'ı kopyalayın

### 2. Apple OAuth Kurulumu (Opsiyonel)

1. **Apple Developer**'a gidin: https://developer.apple.com/account/resources/identifiers/list/serviceId
2. Yeni bir **Service ID** oluşturun
3. **Sign In with Apple** seçeneğini aktif edin
4. Return URLs:
   ```
   https://scrdesk.com/auth/oauth/callback?provider=apple
   ```
5. Service ID ve credentials'ı kopyalayın

### 3. Environment Variables'ı Ayarlayın

VPS'inizde `/opt/scrdesk/.env` dosyası oluşturun:

```bash
# Google OAuth
GOOGLE_CLIENT_ID=your-google-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URL=https://scrdesk.com/auth/oauth/callback?provider=google

# Apple OAuth
APPLE_CLIENT_ID=com.scrdesk.signin
APPLE_CLIENT_SECRET=your-apple-client-secret
APPLE_REDIRECT_URL=https://scrdesk.com/auth/oauth/callback?provider=apple
```

### 4. Servisleri Yeniden Başlatın

```bash
cd /opt/scrdesk
docker-compose down
docker-compose up -d --build auth-service admin-panel
```

## ✅ OAuth Nasıl Çalışır?

### Google ile Giriş:

1. Kullanıcı "Continue with Google" butonuna tıklar
2. Backend `/api/v1/auth/oauth/google` endpoint'ine istek gönderir
3. Google OAuth URL'i döner
4. Kullanıcı Google'a yönlendirilir ve giriş yapar
5. Google callback URL'ine yönlendirir: `/auth/oauth/callback?provider=google&code=...&state=...`
6. Backend kodu token'a çevirir ve kullanıcı bilgilerini alır
7. Kullanıcı yoksa otomatik kayıt olur
8. JWT token'lar üretilir
9. Kullanıcı dashboard'a yönlendirilir

### Apple ile Giriş:

Aynı akış, sadece provider `apple` olarak değişir.

## 🔐 Güvenlik

- **CSRF Protection**: Her OAuth akışı için benzersiz state parametresi kullanılır ve Redis'te doğrulanır
- **Token Validation**: OAuth provider'dan gelen token'lar doğrulanır
- **Automatic Account Linking**: Email bazlı hesap eşleştirme
- **Verified Email**: OAuth ile gelen kullanıcılar otomatik olarak email verified olarak işaretlenir

## 📝 Backend Endpoints

```
GET  /api/v1/auth/oauth/google          → Google OAuth URL'i döner
GET  /api/v1/auth/oauth/google/callback → Google callback handler
GET  /api/v1/auth/oauth/apple           → Apple OAuth URL'i döner
GET  /api/v1/auth/oauth/apple/callback  → Apple callback handler (TODO)
```

## 🎨 Frontend

- **OAuth Butonları**: [/auth](https://scrdesk.com/auth) sayfasında
- **Callback Handler**: [/auth/oauth/callback](https://scrdesk.com/auth/oauth/callback)
- **Modern UI**: Google ve Apple logoları ile profesyonel tasarım

## 🚀 Test

Environment variables ayarlandıktan sonra:

1. https://scrdesk.com/auth adresine gidin
2. "Continue with Google" butonuna tıklayın
3. Google hesabınızı seçin
4. Dashboard'a yönlendirileceksiniz

## ⚠️ Notlar

- OAuth credentials olmadan butonlar hata verecektir
- `.env` dosyasını `.gitignore`'a ekleyin (GİZLİ BİLGİ!)
- Production'da mutlaka gerçek credentials kullanın
- Test için Google Cloud Console'da OAuth consent screen'i yapılandırın

## 📚 Dökümantasyon

- Google OAuth: https://developers.google.com/identity/protocols/oauth2
- Apple Sign In: https://developer.apple.com/sign-in-with-apple/
