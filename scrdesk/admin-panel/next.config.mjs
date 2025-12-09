/** @type {import('next').NextConfig} */
const nextConfig = {
  typescript: {
    ignoreBuildErrors: true,
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
  async rewrites() {
    const authServiceUrl = process.env.AUTH_SERVICE_URL || 'http://localhost:8001';
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8000';

    return [
      // OAuth routes go to auth-service
      {
        source: '/api/v1/auth/oauth/:path*',
        destination: `${authServiceUrl}/api/v1/auth/oauth/:path*`,
      },
      // All other /api routes go to core-server
      {
        source: '/api/:path*',
        destination: `${apiUrl}/api/:path*`,
      },
    ];
  },
};

export default nextConfig;
