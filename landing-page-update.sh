#!/bin/bash

# SCRDESK Landing Page Update
# Run this on your VPS: ssh root@72.61.138.218 'bash -s' < landing-page-update.sh

cat > /var/www/scrdesk/index.html << 'EOF'
<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SCRDESK - Uzaktan Masaüstü Bağlantısı</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }

        .container {
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            padding: 60px;
            max-width: 800px;
            width: 100%;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            text-align: center;
        }

        h1 {
            font-size: 3.5em;
            color: #667eea;
            margin-bottom: 20px;
            font-weight: 700;
        }

        .subtitle {
            font-size: 1.3em;
            color: #555;
            margin-bottom: 40px;
        }

        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 50px;
        }

        .feature {
            padding: 20px;
            background: #f8f9fa;
            border-radius: 10px;
            transition: transform 0.3s;
        }

        .feature:hover {
            transform: translateY(-5px);
            box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
        }

        .feature-icon {
            font-size: 2.5em;
            margin-bottom: 10px;
        }

        .feature-title {
            font-weight: 600;
            color: #333;
            margin-bottom: 5px;
        }

        .feature-desc {
            font-size: 0.9em;
            color: #666;
        }

        .download-section {
            margin-top: 40px;
        }

        .download-title {
            font-size: 2em;
            color: #333;
            margin-bottom: 30px;
        }

        .platforms {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 15px;
            margin-bottom: 30px;
        }

        .platform-btn {
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            text-decoration: none;
            border-radius: 10px;
            font-size: 1.1em;
            font-weight: 600;
            transition: all 0.3s;
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 10px;
        }

        .platform-btn:hover {
            transform: scale(1.05);
            box-shadow: 0 10px 25px rgba(102, 126, 234, 0.4);
        }

        .server-info {
            background: #f0f4ff;
            padding: 30px;
            border-radius: 15px;
            margin-top: 40px;
            border: 2px solid #667eea;
        }

        .server-info h3 {
            color: #667eea;
            margin-bottom: 20px;
            font-size: 1.5em;
        }

        .server-details {
            text-align: left;
            background: white;
            padding: 20px;
            border-radius: 10px;
            font-family: 'Courier New', monospace;
            font-size: 0.95em;
            line-height: 1.8;
        }

        .server-details strong {
            color: #667eea;
        }

        footer {
            margin-top: 40px;
            color: white;
            text-align: center;
        }

        footer a {
            color: white;
            text-decoration: none;
            font-weight: 600;
        }

        footer a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🖥️ SCRDESK</h1>
        <p class="subtitle">Güvenli, Hızlı ve Ücretsiz Uzaktan Masaüstü Bağlantısı</p>

        <div class="features">
            <div class="feature">
                <div class="feature-icon">🔒</div>
                <div class="feature-title">Güvenli</div>
                <div class="feature-desc">End-to-end şifreleme</div>
            </div>
            <div class="feature">
                <div class="feature-icon">⚡</div>
                <div class="feature-title">Hızlı</div>
                <div class="feature-desc">Düşük gecikme</div>
            </div>
            <div class="feature">
                <div class="feature-icon">🆓</div>
                <div class="feature-title">Ücretsiz</div>
                <div class="feature-desc">Açık kaynak</div>
            </div>
            <div class="feature">
                <div class="feature-icon">🌍</div>
                <div class="feature-title">Türkiye</div>
                <div class="feature-desc">Yerli sunucu</div>
            </div>
        </div>

        <div class="download-section">
            <h2 class="download-title">📥 İndir</h2>
            <div class="platforms">
                <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.exe" class="platform-btn">
                    <span>🪟</span>
                    <span>Windows</span>
                </a>
                <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.dmg" class="platform-btn">
                    <span>🍎</span>
                    <span>macOS Intel</span>
                </a>
                <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-aarch64.dmg" class="platform-btn">
                    <span>🍎</span>
                    <span>macOS ARM</span>
                </a>
                <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.deb" class="platform-btn">
                    <span>🐧</span>
                    <span>Linux (DEB)</span>
                </a>
                <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.AppImage" class="platform-btn">
                    <span>🐧</span>
                    <span>Linux (AppImage)</span>
                </a>
                <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-universal-signed.apk" class="platform-btn">
                    <span>🤖</span>
                    <span>Android</span>
                </a>
            </div>
        </div>

        <div class="server-info">
            <h3>⚙️ Sunucu Ayarları</h3>
            <div class="server-details">
                <strong>ID Server:</strong> server.scrdesk.com.tr:21116<br>
                <strong>Relay Server:</strong> server.scrdesk.com.tr:21117<br>
                <strong>API Server:</strong> (boş bırakın)<br>
                <strong>Key:</strong> KddKT26upyAdq21LCAMlnQC1vLiXehLpQAgQI1prHYo=
            </div>
            <p style="margin-top: 15px; color: #666; font-size: 0.9em;">
                Client'ı indirdikten sonra Settings → Network menüsünden bu bilgileri girin.
            </p>
        </div>
    </div>

    <footer>
        <p>SCRDESK - Açık Kaynak Uzaktan Masaüstü</p>
        <p style="margin-top: 10px;">
            <a href="https://github.com/shosgoren/scrdesk" target="_blank">GitHub</a> |
            <a href="https://scrdesk.com.tr" target="_blank">scrdesk.com.tr</a>
        </p>
    </footer>
</body>
</html>
EOF

echo "✅ Landing page updated successfully!"
echo "🌐 Visit: https://scrdesk.com.tr"
