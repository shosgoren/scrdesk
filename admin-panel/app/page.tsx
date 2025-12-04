'use client';

import Link from 'next/link';
import { useState } from 'react';

export default function HomePage() {
  const [hoveredFeature, setHoveredFeature] = useState<number | null>(null);

  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-purple-50">
      {/* Navigation */}
      <nav className="fixed top-0 left-0 right-0 z-50 bg-white/80 backdrop-blur-md border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-6 py-4 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-10 h-10 bg-gradient-to-br from-indigo-600 to-purple-600 rounded-xl flex items-center justify-center">
              <span className="text-white font-bold text-xl">S</span>
            </div>
            <span className="text-2xl font-bold bg-gradient-to-r from-indigo-600 to-purple-600 bg-clip-text text-transparent">
              ScrDesk
            </span>
          </div>

          <div className="flex items-center gap-4">
            <Link
              href="/auth"
              className="px-6 py-2.5 text-gray-700 hover:text-indigo-600 font-medium transition-colors"
            >
              Sign in
            </Link>
            <Link
              href="/auth?mode=signup"
              className="px-6 py-2.5 bg-indigo-600 text-white rounded-full hover:bg-indigo-700 font-medium transition-all hover:shadow-lg"
            >
              Get started
            </Link>
          </div>
        </div>
      </nav>

      {/* Hero Section */}
      <div className="pt-32 pb-20 px-6">
        <div className="max-w-7xl mx-auto">
          <div className="text-center max-w-4xl mx-auto">
            <h1 className="text-6xl font-bold mb-6 leading-tight">
              <span className="bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-600 bg-clip-text text-transparent">
                Remote Desktop
              </span>
              <br />
              Made Simple
            </h1>
            <p className="text-xl text-gray-600 mb-10 leading-relaxed">
              Enterprise-grade remote desktop solution. Secure, fast, and easy to use.
              Connect to any device, anywhere, anytime.
            </p>
            <div className="flex items-center justify-center gap-4">
              <Link
                href="/auth?mode=signup"
                className="px-8 py-4 bg-indigo-600 text-white rounded-full hover:bg-indigo-700 font-semibold text-lg transition-all hover:shadow-xl hover:scale-105"
              >
                Start for free
              </Link>
              <Link
                href="/auth"
                className="px-8 py-4 bg-white text-indigo-600 rounded-full hover:bg-gray-50 font-semibold text-lg transition-all border-2 border-indigo-200 hover:border-indigo-300"
              >
                Learn more
              </Link>
            </div>
          </div>

          {/* Feature Cards */}
          <div className="grid md:grid-cols-3 gap-8 mt-20">
            {[
              {
                title: 'Lightning Fast',
                description: 'Ultra-low latency connections with optimized protocols',
                icon: '⚡',
                gradient: 'from-yellow-400 to-orange-500'
              },
              {
                title: 'Bank-Level Security',
                description: 'End-to-end encryption with enterprise compliance',
                icon: '🔒',
                gradient: 'from-green-400 to-cyan-500'
              },
              {
                title: 'Cross-Platform',
                description: 'Works on Windows, macOS, Linux, iOS, and Android',
                icon: '🌐',
                gradient: 'from-blue-400 to-indigo-500'
              }
            ].map((feature, index) => (
              <div
                key={index}
                onMouseEnter={() => setHoveredFeature(index)}
                onMouseLeave={() => setHoveredFeature(null)}
                className={`relative p-8 bg-white rounded-3xl shadow-lg hover:shadow-2xl transition-all duration-300 transform ${
                  hoveredFeature === index ? 'scale-105' : ''
                }`}
              >
                <div className={`w-16 h-16 bg-gradient-to-br ${feature.gradient} rounded-2xl flex items-center justify-center text-3xl mb-6 shadow-lg`}>
                  {feature.icon}
                </div>
                <h3 className="text-2xl font-bold mb-3 text-gray-900">{feature.title}</h3>
                <p className="text-gray-600 leading-relaxed">{feature.description}</p>
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Footer */}
      <footer className="border-t border-gray-200 py-12 px-6">
        <div className="max-w-7xl mx-auto text-center">
          <p className="text-gray-600">
            © 2025 ScrDesk PRO Enterprise. All rights reserved.
          </p>
        </div>
      </footer>
    </div>
  );
}
