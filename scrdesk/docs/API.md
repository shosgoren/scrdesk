# ScrDesk PRO Enterprise API Documentation

## Table of Contents

1. [Overview](#overview)
2. [Authentication](#authentication)
3. [API Endpoints](#api-endpoints)
   - [Authentication Service](#authentication-service)
   - [Device Manager Service](#device-manager-service)
   - [Policy Engine Service](#policy-engine-service)
   - [Audit Service](#audit-service)
   - [Notification Service](#notification-service)
4. [Data Models](#data-models)
5. [Error Handling](#error-handling)
6. [Rate Limiting](#rate-limiting)

---

## Overview

ScrDesk PRO Enterprise provides a RESTful API for remote desktop management. The API is organized around REST principles with JSON request/response bodies.

**Base URL:** `https://your-domain.com`

**API Version:** v1

**Content-Type:** `application/json`

---

## Authentication

### Overview

The API uses JWT (JSON Web Token) based authentication with access and refresh tokens.

### Token Types

- **Access Token**: Short-lived token (15 minutes default) for API requests
- **Refresh Token**: Long-lived token (7 days default) for obtaining new access tokens

### Authentication Flow

1. Obtain tokens via login endpoint
2. Include access token in `Authorization` header: `Bearer <access_token>`
3. When access token expires, use refresh token to obtain new tokens
4. Re-login when refresh token expires

---

## API Endpoints

### Authentication Service

Base: `/api/v1/auth`

#### Register User

```http
POST /api/v1/auth/register
```

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "name": "John Doe",
  "organization_id": "uuid-optional"
}
```

**Response:** `201 Created`
```json
{
  "id": "user-uuid",
  "email": "user@example.com",
  "name": "John Doe",
  "created_at": "2024-01-15T10:30:00Z"
}
```

#### Login

```http
POST /api/v1/auth/login
```

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "totp_code": "123456"
}
```

Note: `totp_code` is required only if 2FA is enabled.

**Response:** `200 OK`
```json
{
  "access_token": "eyJhbGci...",
  "refresh_token": "eyJhbGci...",
  "user": {
    "id": "user-uuid",
    "email": "user@example.com",
    "name": "John Doe",
    "role": "admin"
  }
}
```

#### Refresh Token

```http
POST /api/v1/auth/refresh
```

**Request Body:**
```json
{
  "refresh_token": "eyJhbGci..."
}
```

**Response:** `200 OK`
```json
{
  "access_token": "eyJhbGci...",
  "refresh_token": "eyJhbGci..."
}
```

#### Logout

```http
POST /api/v1/auth/logout
Authorization: Bearer <access_token>
```

**Response:** `204 No Content`

#### Get Current User

```http
GET /api/v1/auth/me
Authorization: Bearer <access_token>
```

**Response:** `200 OK`
```json
{
  "id": "user-uuid",
  "email": "user@example.com",
  "name": "John Doe",
  "role": "admin",
  "organization_id": "org-uuid",
  "two_factor_enabled": false,
  "created_at": "2024-01-15T10:30:00Z"
}
```

#### Enable 2FA

```http
POST /api/v1/auth/2fa/enable
Authorization: Bearer <access_token>
```

**Response:** `200 OK`
```json
{
  "secret": "JBSWY3DPEHPK3PXP",
  "qr_code": "data:image/png;base64,iVBORw0KGgo...",
  "backup_codes": [
    "12345678",
    "87654321",
    "..."
  ]
}
```

#### Verify 2FA

```http
POST /api/v1/auth/2fa/verify
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "totp_code": "123456"
}
```

**Response:** `200 OK`
```json
{
  "verified": true
}
```

#### Disable 2FA

```http
POST /api/v1/auth/2fa/disable
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "password": "SecurePassword123!",
  "totp_code": "123456"
}
```

**Response:** `204 No Content`

#### Change Password

```http
POST /api/v1/auth/password/change
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "current_password": "OldPassword123!",
  "new_password": "NewPassword456!"
}
```

**Response:** `204 No Content`

#### Request Password Reset

```http
POST /api/v1/auth/password/reset
```

**Request Body:**
```json
{
  "email": "user@example.com"
}
```

**Response:** `200 OK`
```json
{
  "message": "If the email exists, a reset link has been sent"
}
```

#### Confirm Password Reset

```http
POST /api/v1/auth/password/reset/confirm
```

**Request Body:**
```json
{
  "token": "reset-token-from-email",
  "new_password": "NewPassword789!"
}
```

**Response:** `200 OK`

---

### Device Manager Service

Base: `/api/v1/devices`

#### Register Device

```http
POST /api/v1/devices
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "name": "Workstation-01",
  "device_key": "unique-device-key",
  "os_type": "windows",
  "os_version": "Windows 11 Pro",
  "hostname": "DESKTOP-ABC123",
  "ip_address": "192.168.1.100",
  "mac_address": "00:11:22:33:44:55"
}
```

**Response:** `201 Created`
```json
{
  "id": "device-uuid",
  "name": "Workstation-01",
  "device_key": "unique-device-key",
  "status": "pending_approval",
  "created_at": "2024-01-15T10:30:00Z"
}
```

#### List Devices

```http
GET /api/v1/devices
Authorization: Bearer <access_token>
```

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `per_page` (optional): Items per page (default: 20, max: 100)
- `status` (optional): Filter by status (online, offline, pending_approval)
- `os_type` (optional): Filter by OS (windows, linux, macos)

**Response:** `200 OK`
```json
{
  "devices": [
    {
      "id": "device-uuid",
      "name": "Workstation-01",
      "device_key": "unique-device-key",
      "status": "online",
      "os_type": "windows",
      "os_version": "Windows 11 Pro",
      "last_seen": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 42,
    "total_pages": 3
  }
}
```

#### Get Device Details

```http
GET /api/v1/devices/:id
Authorization: Bearer <access_token>
```

**Response:** `200 OK`
```json
{
  "id": "device-uuid",
  "name": "Workstation-01",
  "device_key": "unique-device-key",
  "status": "online",
  "os_type": "windows",
  "os_version": "Windows 11 Pro",
  "hostname": "DESKTOP-ABC123",
  "ip_address": "192.168.1.100",
  "mac_address": "00:11:22:33:44:55",
  "owner_id": "user-uuid",
  "groups": ["group-uuid-1", "group-uuid-2"],
  "last_seen": "2024-01-15T10:30:00Z",
  "created_at": "2024-01-15T09:00:00Z"
}
```

#### Update Device

```http
PUT /api/v1/devices/:id
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "name": "Updated-Workstation-01",
  "os_version": "Windows 11 Pro 22H2"
}
```

**Response:** `200 OK`

#### Delete Device

```http
DELETE /api/v1/devices/:id
Authorization: Bearer <access_token>
```

**Response:** `204 No Content`

#### Approve Device

```http
POST /api/v1/devices/:id/approve
Authorization: Bearer <access_token>
```

**Response:** `200 OK`

#### Revoke Device

```http
POST /api/v1/devices/:id/revoke
Authorization: Bearer <access_token>
```

**Response:** `200 OK`

#### Device Heartbeat

```http
POST /api/v1/devices/:id/heartbeat
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "status": "online",
  "ip_address": "192.168.1.100",
  "cpu_usage": 45.2,
  "memory_usage": 62.8,
  "disk_usage": 75.1
}
```

**Response:** `200 OK`

#### Update Device Status

```http
PUT /api/v1/devices/:id/status
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "status": "online"
}
```

**Response:** `200 OK`

#### Get Device Groups

```http
GET /api/v1/devices/:id/groups
Authorization: Bearer <access_token>
```

**Response:** `200 OK`
```json
{
  "groups": [
    {
      "id": "group-uuid",
      "name": "Engineering Team",
      "device_count": 15
    }
  ]
}
```

#### Add Device to Group

```http
POST /api/v1/devices/:id/groups/:group_id
Authorization: Bearer <access_token>
```

**Response:** `204 No Content`

#### Remove Device from Group

```http
DELETE /api/v1/devices/:id/groups/:group_id
Authorization: Bearer <access_token>
```

**Response:** `204 No Content`

#### Request Connection

```http
POST /api/v1/devices/:id/connect
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "connection_type": "desktop",
  "quality": "auto"
}
```

**Response:** `200 OK`
```json
{
  "connection_id": "conn-uuid",
  "relay_server": "relay.example.com:21116",
  "connection_token": "eyJhbGci...",
  "expires_at": "2024-01-15T11:30:00Z"
}
```

---

### Policy Engine Service

Base: `/api/v1/policies`

#### Create Policy

```http
POST /api/v1/policies
Authorization: Bearer <access_token>
```

**Request Body:**
```json
{
  "name": "Work Hours Only",
  "description": "Allow connections only during work hours",
  "type": "time_restriction",
  "conditions": {
    "allowed_hours": {
      "start": "09:00",
      "end": "17:00",
      "timezone": "America/New_York",
      "days": ["monday", "tuesday", "wednesday", "thursday", "friday"]
    }
  },
  "actions": {
    "allow": false,
    "notify": true
  },
  "priority": 100
}
```

**Response:** `201 Created`

#### List Policies

```http
GET /api/v1/policies
Authorization: Bearer <access_token>
```

**Response:** `200 OK`
```json
{
  "policies": [
    {
      "id": "policy-uuid",
      "name": "Work Hours Only",
      "type": "time_restriction",
      "enabled": true,
      "priority": 100,
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

#### Apply Policy to Device

```http
POST /api/v1/policies/:id/devices/:device_id
Authorization: Bearer <access_token>
```

**Response:** `204 No Content`

---

### Audit Service

Base: `/api/v1/audit`

#### Get Audit Logs

```http
GET /api/v1/audit/logs
Authorization: Bearer <access_token>
```

**Query Parameters:**
- `start_date` (optional): ISO 8601 date
- `end_date` (optional): ISO 8601 date
- `event_type` (optional): Filter by event type
- `user_id` (optional): Filter by user
- `device_id` (optional): Filter by device
- `page` (optional): Page number
- `per_page` (optional): Items per page

**Response:** `200 OK`
```json
{
  "logs": [
    {
      "id": "log-uuid",
      "event_type": "device.connection.established",
      "user_id": "user-uuid",
      "device_id": "device-uuid",
      "ip_address": "203.0.113.42",
      "details": {
        "duration": 3600,
        "bytes_transferred": 1048576
      },
      "timestamp": "2024-01-15T10:30:00Z"
    }
  ],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 150
  }
}
```

---

### Notification Service

Base: `/api/v1/notifications`

#### Get Notifications

```http
GET /api/v1/notifications
Authorization: Bearer <access_token>
```

**Response:** `200 OK`
```json
{
  "notifications": [
    {
      "id": "notif-uuid",
      "type": "device.approval.required",
      "title": "Device Approval Required",
      "message": "New device 'Workstation-01' requires approval",
      "read": false,
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

#### Mark Notification as Read

```http
PUT /api/v1/notifications/:id/read
Authorization: Bearer <access_token>
```

**Response:** `204 No Content`

---

## Data Models

### User

```typescript
{
  id: string (uuid)
  email: string
  name: string
  role: "admin" | "user" | "viewer"
  organization_id: string (uuid)
  two_factor_enabled: boolean
  created_at: string (ISO 8601)
  updated_at: string (ISO 8601)
}
```

### Device

```typescript
{
  id: string (uuid)
  name: string
  device_key: string
  status: "online" | "offline" | "pending_approval" | "revoked"
  os_type: "windows" | "linux" | "macos"
  os_version: string
  hostname: string
  ip_address: string
  mac_address: string
  owner_id: string (uuid)
  last_seen: string (ISO 8601)
  created_at: string (ISO 8601)
}
```

### Policy

```typescript
{
  id: string (uuid)
  name: string
  description: string
  type: "time_restriction" | "ip_whitelist" | "user_restriction"
  conditions: object
  actions: object
  priority: number
  enabled: boolean
  created_at: string (ISO 8601)
}
```

---

## Error Handling

### Error Response Format

```json
{
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Invalid access token",
    "details": {}
  }
}
```

### Common Error Codes

| HTTP Status | Code | Description |
|-------------|------|-------------|
| 400 | `BAD_REQUEST` | Invalid request parameters |
| 401 | `UNAUTHORIZED` | Missing or invalid authentication |
| 403 | `FORBIDDEN` | Insufficient permissions |
| 404 | `NOT_FOUND` | Resource not found |
| 409 | `CONFLICT` | Resource conflict (e.g., duplicate email) |
| 422 | `VALIDATION_ERROR` | Request validation failed |
| 429 | `RATE_LIMIT_EXCEEDED` | Too many requests |
| 500 | `INTERNAL_ERROR` | Server error |

---

## Rate Limiting

The API implements rate limiting to prevent abuse:

- **Authentication endpoints**: 5 requests per minute per IP
- **Standard endpoints**: 100 requests per minute per user
- **Device heartbeat**: 1 request per minute per device

Rate limit headers:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1234567890
```

When rate limit is exceeded, API returns `429 Too Many Requests`:
```json
{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Too many requests. Please try again in 60 seconds.",
    "retry_after": 60
  }
}
```

---

## Additional Resources

- [Desktop Client Documentation](./DESKTOP_CLIENT.md)
- [Deployment Guide](./DEPLOYMENT.md)
- [Architecture Overview](./ARCHITECTURE.md)

For support, contact: support@scrdesk.com
