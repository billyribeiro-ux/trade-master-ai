# TradeMaster AI - API Documentation

## Base URL
```
http://localhost:8000/api/v1
```

## Authentication

All endpoints (except `/auth/register` and `/auth/login`) require a valid JWT token in the `Authorization` header:

```
Authorization: Bearer <access_token>
```

### Token Refresh
Access tokens expire after 15 minutes. Use the refresh token to obtain a new access token.

---

## Authentication Endpoints

### POST /auth/register
Register a new user account.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:** `201 Created`
```json
{
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "created_at": "2026-02-16T10:00:00Z"
  },
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "expires_in": 900
}
```

**Errors:**
- `400 Bad Request` - Invalid email or password format
- `409 Conflict` - Email already exists

---

### POST /auth/login
Authenticate and receive access tokens.

**Request Body:**
```json
{
  "email": "user@example.com",
  "password": "securepassword123"
}
```

**Response:** `200 OK`
```json
{
  "user": {
    "id": "uuid",
    "email": "user@example.com"
  },
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "expires_in": 900
}
```

**Errors:**
- `401 Unauthorized` - Invalid credentials

---

### POST /auth/refresh
Refresh an expired access token.

**Request Body:**
```json
{
  "refresh_token": "eyJ..."
}
```

**Response:** `200 OK`
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "expires_in": 900
}
```

**Errors:**
- `401 Unauthorized` - Invalid or expired refresh token

---

### POST /auth/logout
Revoke the current refresh token.

**Headers:** `Authorization: Bearer <access_token>`

**Request Body:**
```json
{
  "refresh_token": "eyJ..."
}
```

**Response:** `200 OK`
```json
{
  "message": "Logged out successfully"
}
```

---

### GET /auth/me
Get current authenticated user information.

**Headers:** `Authorization: Bearer <access_token>`

**Response:** `200 OK`
```json
{
  "id": "uuid",
  "email": "user@example.com",
  "created_at": "2026-02-16T10:00:00Z"
}
```

**Errors:**
- `401 Unauthorized` - Invalid or missing token

---

## Trade Endpoints

### POST /trades
Create a new trade.

**Headers:** `Authorization: Bearer <access_token>`

**Request Body:**
```json
{
  "symbol": "AAPL",
  "direction": "long",
  "asset_class": "stock",
  "entry_date": "2026-02-16T09:30:00Z",
  "entry_price": 150.50,
  "quantity": 100,
  "stop_loss": 148.00,
  "take_profit": 155.00,
  "risk_percent": 1.0,
  "conviction": 4,
  "setup_name": "Breakout",
  "timeframe": "5m",
  "thesis": "Breaking above resistance with volume"
}
```

**Response:** `201 Created`
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "symbol": "AAPL",
  "direction": "long",
  "status": "open",
  "entry_price": 150.50,
  "created_at": "2026-02-16T10:00:00Z"
}
```

---

### GET /trades
List all trades with filtering and pagination.

**Headers:** `Authorization: Bearer <access_token>`

**Query Parameters:**
- `page` (integer, default: 1)
- `per_page` (integer, default: 50, max: 100)
- `sort_by` (string: entry_date, exit_date, symbol, pnl)
- `sort_order` (string: asc, desc)
- `status` (string: open, closed, partial)
- `symbol` (string)
- `direction` (string: long, short)
- `from_date` (ISO 8601 date)
- `to_date` (ISO 8601 date)

**Response:** `200 OK`
```json
{
  "trades": [...],
  "total": 150,
  "page": 1,
  "per_page": 50,
  "total_pages": 3
}
```

---

### GET /trades/:id
Get a single trade with all details.

**Headers:** `Authorization: Bearer <access_token>`

**Response:** `200 OK`

### PUT /trades/:id
Update an existing trade.

**Headers:** `Authorization: Bearer <access_token>`

**Response:** `200 OK`

### DELETE /trades/:id
Delete a trade.

**Headers:** `Authorization: Bearer <access_token>`

**Response:** `204 No Content`

### GET /trades/stats
Get trading statistics summary.

**Headers:** `Authorization: Bearer <access_token>`

**Response:** `200 OK`

---

## Analytics Endpoints

### GET /analytics/equity-curve
Get equity curve data points.

### GET /analytics/win-loss-distribution
Get win/loss distribution data.

### GET /analytics/setup-performance
Get performance breakdown by setup type.

---

## Tags Endpoints

### GET /tags
List all user tags.

### POST /tags
Create a new tag.

---

## Error Responses

All endpoints return standard HTTP status codes with JSON error messages.

## Rate Limiting

- Authentication endpoints: 10 requests per minute
- AI endpoints: 10 reviews per hour
- All other endpoints: 100 requests per minute
