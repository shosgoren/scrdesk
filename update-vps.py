#!/usr/bin/env python3
import subprocess
import sys

html_content = """<!DOCTYPE html>
<html lang="tr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SCRDESK - Uzaktan Masaüstü Bağlantısı</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .container {
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            padding: 50px;
            max-width: 800px;
            width: 100%;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            text-align: center;
        }
        h1 { font-size: 3em; color: #667eea; margin-bottom: 15px; font-weight: 700; }
        .subtitle { font-size: 1.2em; color: #555; margin-bottom: 30px; }
        .download-title { font-size: 1.8em; color: #333; margin: 30px 0 20px; }
        .platforms {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
            gap: 12px;
            margin-bottom: 25px;
        }
        .platform-btn {
            padding: 18px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            text-decoration: none;
            border-radius: 8px;
            font-weight: 600;
            transition: all 0.3s;
        }
        .platform-btn:hover { transform: scale(1.05); }
        .server-info {
            background: #f0f4ff;
            padding: 25px;
            border-radius: 12px;
            margin-top: 30px;
            border: 2px solid #667eea;
        }
        .server-details {
            background: white;
            padding: 15px;
            border-radius: 8px;
            font-family: monospace;
            font-size: 0.9em;
            text-align: left;
            line-height: 1.6;
        }
        .server-details strong { color: #667eea; }
        footer { margin-top: 30px; color: white; }
        footer a { color: white; text-decoration: none; font-weight: 600; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🖥️ SCRDESK</h1>
        <p class="subtitle">Güvenli, Hızlı ve Ücretsiz Uzaktan Masaüstü</p>
        
        <h2 class="download-title">📥 İndir</h2>
        <div class="platforms">
            <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.exe" class="platform-btn">🪟 Windows</a>
            <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.dmg" class="platform-btn">🍎 macOS Intel</a>
            <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-aarch64.dmg" class="platform-btn">🍎 macOS ARM</a>
            <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.deb" class="platform-btn">🐧 Linux DEB</a>
            <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-x86_64.AppImage" class="platform-btn">🐧 AppImage</a>
            <a href="https://github.com/rustdesk/rustdesk/releases/download/1.4.4/rustdesk-1.4.4-universal-signed.apk" class="platform-btn">🤖 Android</a>
        </div>
        
        <div class="server-info">
            <h3 style="color: #667eea; margin-bottom: 15px;">⚙️ Sunucu Ayarları</h3>
            <div class="server-details">
                <strong>ID Server:</strong> server.scrdesk.com.tr:21116<br>
                <strong>Relay Server:</strong> server.scrdesk.com.tr:21117<br>
                <strong>Key:</strong> KddKT26upyAdq21LCAMlnQC1vLiXehLpQAgQI1prHYo=
            </div>
            <p style="margin-top: 12px; color: #666; font-size: 0.85em;">Client'ı indirip Settings → Network'ten bu ayarları girin.</p>
        </div>
    </div>
    <footer>
        <p>SCRDESK | <a href="https://github.com/shosgoren/scrdesk">GitHub</a></p>
    </footer>
</body>
</html>
"""

# Write HTML via SSH
cmd = [
    "ssh", "-o", "StrictHostKeyChecking=no",
    "root@72.61.138.218",
    f"echo '{html_content}' > /var/www/scrdesk/index.html && echo 'Updated!' || echo 'Failed!'"
]

print("Connecting to VPS and updating landing page...")
print("Please enter password: johqid-4Cuhxi-nugvov")
result = subprocess.run(cmd, capture_output=False, text=True)
sys.exit(result.returncode)
