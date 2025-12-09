'use client';

import { useEffect, useState } from 'react';
import { apiClient } from '@/lib/api';

export default function Dashboard() {
  const [stats, setStats] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // TODO: Implement /api/v1/admin/stats endpoint in backend
    // For now, using mock data
    setStats({
      total_tenants: 0,
      total_users: 0,
      active_devices: 0,
      active_sessions: 0,
    });
    setLoading(false);
  }, []);

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
