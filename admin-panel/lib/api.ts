// Use empty string to make API calls relative to the current domain
// Nginx will proxy /api/ requests to the backend services
const API_URL = process.env.NEXT_PUBLIC_API_URL || '';

export interface LoginCredentials {
  email: string;
  password: string;
  two_factor_code?: string;
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  expires_in: number;
  user: {
    id: string;
    email: string;
    full_name: string;
    role: string;
    tenant_id: string;
  };
}

export interface ApiError {
  error: string;
  message: string;
}

class ApiClient {
  private baseUrl: string;
  private accessToken: string | null = null;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
    if (typeof window !== 'undefined') {
      this.accessToken = localStorage.getItem('access_token');
    }
  }

  setAccessToken(token: string) {
    this.accessToken = token;
    if (typeof window !== 'undefined') {
      localStorage.setItem('access_token', token);
    }
  }

  clearAccessToken() {
    this.accessToken = null;
    if (typeof window !== 'undefined') {
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
    }
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (this.accessToken) {
      headers['Authorization'] = `Bearer ${this.accessToken}`;
    }

    const response = await fetch(`${this.baseUrl}${endpoint}`, {
      ...options,
      headers,
    });

    if (!response.ok) {
      const error: ApiError = await response.json();
      throw new Error(error.message || 'An error occurred');
    }

    return response.json();
  }

  async login(credentials: LoginCredentials): Promise<AuthResponse> {
    const response = await this.request<AuthResponse>('/api/v1/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    });

    this.setAccessToken(response.access_token);
    if (typeof window !== 'undefined') {
      localStorage.setItem('refresh_token', response.refresh_token);
    }

    return response;
  }

  async logout(): Promise<void> {
    try {
      await this.request('/api/v1/auth/logout', { method: 'POST' });
    } finally {
      this.clearAccessToken();
    }
  }

  async getCurrentUser() {
    return this.request('/api/v1/auth/me', { method: 'GET' });
  }

  async getTenants(params?: { limit?: number; offset?: number }) {
    const query = new URLSearchParams(params as any).toString();
    return this.request(`/api/v1/tenants${query ? `?${query}` : ''}`, {
      method: 'GET',
    });
  }

  async getUsers(params?: { limit?: number; offset?: number }) {
    const query = new URLSearchParams(params as any).toString();
    return this.request(`/api/v1/users${query ? `?${query}` : ''}`, {
      method: 'GET',
    });
  }

  async getDevices(params?: { limit?: number; offset?: number }) {
    const query = new URLSearchParams(params as any).toString();
    return this.request(`/api/v1/devices${query ? `?${query}` : ''}`, {
      method: 'GET',
    });
  }

  async getSessions(params?: { limit?: number; offset?: number }) {
    const query = new URLSearchParams(params as any).toString();
    return this.request(`/api/v1/sessions${query ? `?${query}` : ''}`, {
      method: 'GET',
    });
  }

  async getStats() {
    return this.request('/api/v1/admin/stats', { method: 'GET' });
  }
}

export const apiClient = new ApiClient(API_URL);
