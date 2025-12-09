'use client';

import { useState } from 'react';

interface Download {
  platform: string;
  icon: string;
  downloads: {
    name: string;
    url: string;
    size: string;
    arch?: string;
  }[];
  instructions: string[];
  color: string;
}

export default function DownloadsPage() {
  const [selectedPlatform, setSelectedPlatform] = useState<string>('macos');

  const downloads: Download[] = [
    {
      platform: 'macOS',
      icon: '🍎',
      color: 'blue',
      downloads: [
        {
          name: 'macOS Installer (M1/M2/M3/M4)',
          url: 'https://scrdesk.com/downloads/ScrDesk-M4-v1.0.0.dmg',
          size: '3.1 MB',
          arch: 'Apple Silicon - Signed DMG',
        },
        {
          name: 'macOS Binary (Apple Silicon)',
          url: 'https://scrdesk.com/downloads/scrdesk-macos-arm64',
          size: '4.6 MB',
          arch: 'M1/M2/M3/M4 - Binary',
        },
        {
          name: 'macOS Binary (Intel)',
          url: 'https://scrdesk.com/downloads/scrdesk-macos-intel',
          size: '5.1 MB',
          arch: 'x86_64 - Binary',
        },
      ],
      instructions: [
        'Download the DMG installer for your Mac',
        'Double-click the DMG file to mount it',
        'Drag ScrDesk.app to the Applications folder',
        'Eject the DMG',
        'Open Applications folder and find ScrDesk',
        'Right-click ScrDesk.app and select "Open" (first time only)',
        'Click "Open" in the security dialog',
      ],
    },
    {
      platform: 'Windows',
      icon: '🪟',
      color: 'green',
      downloads: [
        {
          name: 'Windows Installer (64-bit)',
          url: 'https://scrdesk.com/downloads/ScrDesk-Windows-x64-v1.0.0.exe',
          size: '5.8 MB',
          arch: 'x86_64 - Windows 10+',
        },
      ],
      instructions: [
        'Download the Windows installer',
        'Double-click the downloaded .exe file',
        'Windows may show a security warning - click "More info" then "Run anyway"',
        'The app will launch automatically',
        'Choose "Quick Connect" for free 1-hour trial or "Sign In" for full access',
      ],
    },
    {
      platform: 'Linux',
      icon: '🐧',
      color: 'purple',
      downloads: [
        {
          name: 'Debian/Ubuntu (.deb)',
          url: 'https://scrdesk.com/downloads/scrdesk-linux.deb',
          size: 'Coming Soon',
        },
        {
          name: 'Red Hat/Fedora (.rpm)',
          url: 'https://scrdesk.com/downloads/scrdesk-linux.rpm',
          size: 'Coming Soon',
        },
      ],
      instructions: [
        'Download the appropriate package for your distribution',
        'For Debian/Ubuntu: sudo dpkg -i scrdesk-linux.deb',
        'For Red Hat/Fedora: sudo rpm -i scrdesk-linux.rpm',
        'Launch from applications menu or run: scrdesk',
      ],
    },
    {
      platform: 'Android',
      icon: '🤖',
      color: 'orange',
      downloads: [
        {
          name: 'Android APK',
          url: 'https://scrdesk.com/downloads/scrdesk-android.apk',
          size: 'Coming Soon',
        },
      ],
      instructions: [
        'Download the APK file',
        'Enable "Install from Unknown Sources" in Settings > Security',
        'Open the downloaded APK file',
        'Follow the installation prompts',
        'Launch ScrDesk from your app drawer',
      ],
    },
  ];

  const selectedDownload = downloads.find((d) => d.platform.toLowerCase() === selectedPlatform);

  return (
    <div className="p-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Download ScrDesk</h1>
        <p className="text-gray-600">
          Download the ScrDesk client for your platform and start accessing your remote desktops
        </p>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-4 gap-4 mb-8">
        {downloads.map((download) => (
          <button
            key={download.platform}
            onClick={() => setSelectedPlatform(download.platform.toLowerCase())}
            className={`p-6 rounded-lg border-2 transition-all ${
              selectedPlatform === download.platform.toLowerCase()
                ? `border-${download.color}-500 bg-${download.color}-50`
                : 'border-gray-200 bg-white hover:border-gray-300'
            }`}
          >
            <div className="text-5xl mb-3">{download.icon}</div>
            <h3 className="text-lg font-bold text-gray-900">{download.platform}</h3>
          </button>
        ))}
      </div>

      {selectedDownload && (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <div className="bg-white p-6 rounded-lg shadow-md">
            <h2 className="text-xl font-bold text-gray-900 mb-4 flex items-center gap-2">
              <span className="text-3xl">{selectedDownload.icon}</span>
              Download {selectedDownload.platform}
            </h2>
            <div className="space-y-3">
              {selectedDownload.downloads.map((file, index) => (
                <div
                  key={index}
                  className="flex items-center justify-between p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition"
                >
                  <div className="flex-1">
                    <h3 className="font-semibold text-gray-900">{file.name}</h3>
                    <div className="flex items-center gap-3 text-sm text-gray-600 mt-1">
                      <span>{file.size}</span>
                      {file.arch && <span className="text-gray-400">•</span>}
                      {file.arch && <span>{file.arch}</span>}
                    </div>
                  </div>
                  {file.size !== 'Coming Soon' ? (
                    <a
                      href={file.url}
                      download
                      className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
                    >
                      Download
                    </a>
                  ) : (
                    <span className="px-4 py-2 bg-gray-300 text-gray-600 rounded-lg cursor-not-allowed">
                      Coming Soon
                    </span>
                  )}
                </div>
              ))}
            </div>

            {selectedDownload.platform === 'macOS' && (
              <div className="mt-6 p-4 bg-green-50 border border-green-200 rounded-lg">
                <h4 className="font-semibold text-green-900 mb-2">⚡ Quick Install (Recommended)</h4>
                <p className="text-sm text-green-800 mb-3">
                  Run this one-line command in Terminal to automatically download and install:
                </p>
                <code className="text-xs bg-white px-3 py-2 rounded border border-green-200 block overflow-x-auto">
                  curl -fsSL https://scrdesk.com/downloads/install-macos.sh | bash
                </code>
                <p className="text-xs text-green-700 mt-2">
                  This script detects your Mac architecture and installs to ~/.local/bin
                </p>
              </div>
            )}

            <div className="mt-6 p-4 bg-blue-50 border border-blue-200 rounded-lg">
              <h4 className="font-semibold text-blue-900 mb-2">Verify Download</h4>
              <p className="text-sm text-blue-800 mb-2">
                To verify the integrity of your download, check the SHA256 checksum:
              </p>
              <code className="text-xs bg-white px-2 py-1 rounded border border-blue-200 block overflow-x-auto">
                shasum -a 256 scrdesk-*
              </code>
              <a
                href="https://scrdesk.com/downloads/SHA256SUMS"
                className="text-sm text-blue-600 hover:text-blue-800 mt-2 inline-block"
              >
                View SHA256SUMS file
              </a>
            </div>
          </div>

          <div className="bg-white p-6 rounded-lg shadow-md">
            <h2 className="text-xl font-bold text-gray-900 mb-4">Installation Instructions</h2>
            <ol className="space-y-3">
              {selectedDownload.instructions.map((instruction, index) => (
                <li key={index} className="flex gap-3">
                  <span className="flex-shrink-0 w-6 h-6 bg-blue-600 text-white rounded-full flex items-center justify-center text-sm font-bold">
                    {index + 1}
                  </span>
                  <span className="text-gray-700 pt-0.5">{instruction}</span>
                </li>
              ))}
            </ol>

            <div className="mt-6 p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
              <h4 className="font-semibold text-yellow-900 mb-2 flex items-center gap-2">
                <span>⚠️</span>
                Important
              </h4>
              <p className="text-sm text-yellow-800">
                Make sure you have an active ScrDesk account before using the client. You can register at{' '}
                <a href="https://scrdesk.com" className="text-blue-600 hover:underline">
                  scrdesk.com
                </a>
              </p>
            </div>

            <div className="mt-6">
              <h4 className="font-semibold text-gray-900 mb-3">System Requirements</h4>
              <ul className="space-y-2 text-sm text-gray-700">
                {selectedDownload.platform === 'macOS' && (
                  <>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      macOS 10.15 (Catalina) or later
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      4GB RAM minimum
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      Internet connection required
                    </li>
                  </>
                )}
                {selectedDownload.platform === 'Windows' && (
                  <>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      Windows 10 or later
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      4GB RAM minimum
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      Internet connection required
                    </li>
                  </>
                )}
                {selectedDownload.platform === 'Linux' && (
                  <>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      Modern Linux distribution
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      4GB RAM minimum
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      Internet connection required
                    </li>
                  </>
                )}
                {selectedDownload.platform === 'Android' && (
                  <>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      Android 8.0 (Oreo) or later
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      2GB RAM minimum
                    </li>
                    <li className="flex items-center gap-2">
                      <span className="text-green-600">✓</span>
                      Internet connection required
                    </li>
                  </>
                )}
              </ul>
            </div>

            {selectedDownload.platform === 'macOS' && (
              <div className="mt-6 p-4 bg-red-50 border border-red-200 rounded-lg">
                <h4 className="font-semibold text-red-900 mb-3">Troubleshooting macOS Security</h4>
                <div className="text-sm text-red-800 space-y-2">
                  <p className="font-medium">If you see "cannot be opened because it is from an unidentified developer":</p>
                  <ol className="list-decimal ml-5 space-y-1">
                    <li>Open Terminal and run: <code className="bg-white px-2 py-1 rounded text-xs">xattr -d com.apple.quarantine ~/Downloads/scrdesk-macos-*</code></li>
                    <li>Or: Control-click the file, select "Open", then click "Open" in the dialog</li>
                    <li>Or: Go to System Settings → Privacy & Security → scroll down and click "Open Anyway"</li>
                  </ol>
                  <p className="font-medium mt-3">If the file won't execute:</p>
                  <ol className="list-decimal ml-5 space-y-1">
                    <li>Make sure you ran: <code className="bg-white px-2 py-1 rounded text-xs">chmod +x scrdesk-macos-*</code></li>
                    <li>Try running with: <code className="bg-white px-2 py-1 rounded text-xs">./scrdesk-macos-*</code> (don't forget the ./)</li>
                  </ol>
                </div>
              </div>
            )}
          </div>
        </div>
      )}

      <div className="mt-8 bg-white p-6 rounded-lg shadow-md">
        <h2 className="text-xl font-bold text-gray-900 mb-4">Need Help?</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="p-4 bg-gray-50 rounded-lg">
            <h3 className="font-semibold text-gray-900 mb-2">Documentation</h3>
            <p className="text-sm text-gray-600 mb-3">
              Read our comprehensive guides and documentation
            </p>
            <a href="#" className="text-sm text-blue-600 hover:text-blue-800">
              View Docs →
            </a>
          </div>
          <div className="p-4 bg-gray-50 rounded-lg">
            <h3 className="font-semibold text-gray-900 mb-2">Support</h3>
            <p className="text-sm text-gray-600 mb-3">Get help from our support team</p>
            <a href="#" className="text-sm text-blue-600 hover:text-blue-800">
              Contact Support →
            </a>
          </div>
          <div className="p-4 bg-gray-50 rounded-lg">
            <h3 className="font-semibold text-gray-900 mb-2">Community</h3>
            <p className="text-sm text-gray-600 mb-3">Join our community forums</p>
            <a href="#" className="text-sm text-blue-600 hover:text-blue-800">
              Join Community →
            </a>
          </div>
        </div>
      </div>
    </div>
  );
}
