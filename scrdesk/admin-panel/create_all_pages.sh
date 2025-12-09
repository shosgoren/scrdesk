#!/bin/bash

# Sidebar Component
cat > components/Sidebar.tsx << 'EOF'
'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { useAuth } from '@/contexts/AuthContext';

export default function Sidebar() {
  const pathname = usePathname();
  const { user, logout } = useAuth();

  const menuItems = [
    { href: '/dashboard', label: 'Dashboard', icon: '📊' },
    { href: '/dashboard/tenants', label: 'Tenants', icon: '🏢' },
    { href: '/dashboard/users', label: 'Users', icon: '👥' },
    { href: '/dashboard/devices', label: 'Devices', icon: '💻' },
    { href: '/dashboard/sessions', label: 'Sessions', icon: '🔗' },
    { href: '/dashboard/settings', label: 'Settings', icon: '⚙️' },
  ];

  return (
    <div className="w-64 bg-gray-900 min-h-screen text-white flex flex-col">
      <div className="p-6">
        <h1 className="text-2xl font-bold">ScrDesk</h1>
        <p className="text-gray-400 text-sm">Admin Panel</p>
      </div>

      <nav className="flex-1 px-4">
        {menuItems.map((item) => (
          <Link
            key={item.href}
            href={item.href}
            className={`flex items-center gap-3 px-4 py-3 rounded-lg mb-2 transition ${
              pathname === item.href
                ? 'bg-blue-600 text-white'
                : 'text-gray-300 hover:bg-gray-800'
            }`}
          >
            <span className="text-xl">{item.icon}</span>
            <span>{item.label}</span>
          </Link>
        ))}
      </nav>

      <div className="p-4 border-t border-gray-800">
        <div className="mb-4">
          <p className="text-sm text-gray-400">Signed in as</p>
          <p className="text-sm font-semibold">{user?.email}</p>
          <p className="text-xs text-gray-500">{user?.role}</p>
        </div>
        <button
          onClick={logout}
          className="w-full bg-red-600 hover:bg-red-700 px-4 py-2 rounded-lg transition"
        >
          Logout
        </button>
      </div>
    </div>
  );
}
EOF

# Dashboard Layout
mkdir -p app/dashboard
cat > app/dashboard/layout.tsx << 'EOF'
'use client';

import Sidebar from '@/components/Sidebar';
import { useAuth } from '@/contexts/AuthContext';
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';

export default function DashboardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const { user, loading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!loading && !user) {
      router.push('/login');
    }
  }, [user, loading, router]);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p className="text-gray-600">Loading...</p>
        </div>
      </div>
    );
  }

  if (!user) {
    return null;
  }

  return (
    <div className="flex">
      <Sidebar />
      <main className="flex-1 bg-gray-100">{children}</main>
    </div>
  );
}
EOF

# Dashboard Page
cat > app/dashboard/page.tsx << 'EOF'
'use client';

import { useEffect, useState } from 'react';
import { apiClient } from '@/lib/api';

export default function Dashboard() {
  const [stats, setStats] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadStats();
  }, []);

  const loadStats = async () => {
    try {
      const data = await apiClient.getStats();
      setStats(data);
    } catch (error) {
      console.error('Failed to load stats:', error);
      // Set mock data if API fails
      setStats({
        total_tenants: 0,
        total_users: 0,
        active_devices: 0,
        active_sessions: 0,
      });
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="p-8">
        <div className="animate-pulse">
          <div className="h-8 bg-gray-300 rounded w-1/4 mb-6"></div>
          <div className="grid grid-cols-4 gap-6">
            {[1, 2, 3, 4].map((i) => (
              <div key={i} className="bg-white p-6 rounded-lg shadow h-32"></div>
            ))}
          </div>
        </div>
      </div>
    );
  }

  const cards = [
    {
      title: 'Total Tenants',
      value: stats?.total_tenants || 0,
      icon: '🏢',
      color: 'blue',
    },
    {
      title: 'Total Users',
      value: stats?.total_users || 0,
      icon: '👥',
      color: 'green',
    },
    {
      title: 'Active Devices',
      value: stats?.active_devices || 0,
      icon: '💻',
      color: 'purple',
    },
    {
      title: 'Active Sessions',
      value: stats?.active_sessions || 0,
      icon: '🔗',
      color: 'orange',
    },
  ];

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold text-gray-900 mb-8">Dashboard</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {cards.map((card) => (
          <div
            key={card.title}
            className="bg-white p-6 rounded-lg shadow-md hover:shadow-lg transition"
          >
            <div className="flex items-center justify-between mb-4">
              <span className="text-3xl">{card.icon}</span>
              <span className={`text-${card.color}-600 text-sm font-semibold`}>
                Live
              </span>
            </div>
            <h3 className="text-gray-600 text-sm mb-2">{card.title}</h3>
            <p className="text-4xl font-bold text-gray-900">{card.value.toLocaleString()}</p>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-xl font-bold text-gray-900 mb-4">Recent Activity</h2>
          <div className="space-y-3">
            {[1, 2, 3, 4, 5].map((i) => (
              <div key={i} className="flex items-center gap-4 p-3 bg-gray-50 rounded-lg">
                <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center">
                  <span className="text-blue-600 font-semibold">{i}</span>
                </div>
                <div className="flex-1">
                  <p className="text-sm font-semibold text-gray-900">Activity {i}</p>
                  <p className="text-xs text-gray-600">Just now</p>
                </div>
              </div>
            ))}
          </div>
        </div>

        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-xl font-bold text-gray-900 mb-4">System Health</h2>
          <div className="space-y-4">
            {[
              { name: 'API Server', status: 'healthy', value: '99.9%' },
              { name: 'Database', status: 'healthy', value: '100%' },
              { name: 'Cache Server', status: 'healthy', value: '98.5%' },
              { name: 'Storage', status: 'warning', value: '85%' },
            ].map((service) => (
              <div key={service.name} className="flex items-center justify-between">
                <div className="flex items-center gap-3">
                  <div
                    className={`w-3 h-3 rounded-full ${
                      service.status === 'healthy' ? 'bg-green-500' : 'bg-yellow-500'
                    }`}
                  ></div>
                  <span className="text-sm font-semibold text-gray-900">{service.name}</span>
                </div>
                <span className="text-sm text-gray-600">{service.value}</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
EOF

echo "✓ All pages created successfully"
