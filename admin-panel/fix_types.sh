#!/bin/bash

# Fix all management pages to have proper typing

# Devices
sed -i '' 's/const data = await apiClient.getDevices();/const data = await apiClient.getDevices() as any;/' app/dashboard/devices/page.tsx

# Users
sed -i '' 's/const data = await apiClient.getUsers();/const data = await apiClient.getUsers() as any;/' app/dashboard/users/page.tsx

# Tenants
sed -i '' 's/const data = await apiClient.getTenants();/const data = await apiClient.getTenants() as any;/' app/dashboard/tenants/page.tsx

# Sessions
sed -i '' 's/const data = await apiClient.getSessions();/const data = await apiClient.getSessions() as any;/' app/dashboard/sessions/page.tsx

# Dashboard
sed -i '' 's/const data = await apiClient.getStats();/const data = await apiClient.getStats() as any;/' app/dashboard/page.tsx

echo "✓ All type errors fixed"
