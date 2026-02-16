# Production Monitoring Setup

## Overview

This document outlines the production monitoring, logging, and alerting strategy for TradeMaster AI.

---

## Error Tracking

### Recommended: Sentry

**Setup:**
1. Create a Sentry account at https://sentry.io
2. Create a new project for the frontend
3. Add Sentry DSN to environment variables

**Frontend Integration:**
```bash
cd apps/web
pnpm add @sentry/sveltekit
```

**Configuration (`apps/web/src/hooks.client.ts`):**
```typescript
import * as Sentry from '@sentry/sveltekit';

Sentry.init({
  dsn: import.meta.env.VITE_SENTRY_DSN,
  environment: import.meta.env.MODE,
  tracesSampleRate: 0.1,
  replaysSessionSampleRate: 0.1,
  replaysOnErrorSampleRate: 1.0,
});
```

**Backend Integration:**
```bash
cd apps/api
cargo add sentry sentry-tracing
```

**Configuration (`apps/api/src/main.rs`):**
```rust
let _guard = sentry::init((
    std::env::var("SENTRY_DSN").unwrap_or_default(),
    sentry::ClientOptions {
        release: sentry::release_name!(),
        environment: Some(std::env::var("ENVIRONMENT").unwrap_or("development".into()).into()),
        traces_sample_rate: 0.1,
        ..Default::default()
    },
));
```

---

## Performance Monitoring

### Web Vitals Tracking

**Core Web Vitals to Track:**
- **LCP (Largest Contentful Paint):** < 2.5s
- **FID (First Input Delay):** < 100ms
- **CLS (Cumulative Layout Shift):** < 0.1

**Implementation:**
```typescript
// apps/web/src/lib/services/web-vitals.ts
import { onCLS, onFID, onLCP } from 'web-vitals';

export function trackWebVitals() {
  onCLS((metric) => monitoring.trackPerformance({
    name: 'cls',
    value: metric.value,
    unit: 'count'
  }));
  
  onFID((metric) => monitoring.trackPerformance({
    name: 'fid',
    value: metric.value,
    unit: 'ms'
  }));
  
  onLCP((metric) => monitoring.trackPerformance({
    name: 'lcp',
    value: metric.value,
    unit: 'ms'
  }));
}
```

### API Response Time Monitoring

**Rust Middleware:**
```rust
// apps/api/src/middleware/metrics.rs
use axum::middleware::Next;
use axum::response::Response;
use std::time::Instant;

pub async fn track_request_duration(
    req: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let path = req.uri().path().to_string();
    let method = req.method().to_string();
    
    let response = next.run(req).await;
    
    let duration = start.elapsed();
    tracing::info!(
        method = %method,
        path = %path,
        status = %response.status(),
        duration_ms = %duration.as_millis(),
        "Request completed"
    );
    
    response
}
```

---

## Logging

### Frontend Logging

**Use the monitoring service:**
```typescript
import { monitoring } from '$lib/services/monitoring';

// Error logging
try {
  await riskyOperation();
} catch (error) {
  monitoring.logError(error as Error, {
    route: '/trades',
    component: 'TradeForm',
    action: 'create_trade'
  });
}

// Info logging
monitoring.logInfo('User completed onboarding', {
  user_id: user.id,
  timestamp: new Date().toISOString()
});
```

### Backend Logging

**Structured logging with tracing:**
```rust
use tracing::{info, warn, error};

// Info level
info!(
    user_id = %user_id,
    trade_id = %trade_id,
    "Trade created successfully"
);

// Warning level
warn!(
    user_id = %user_id,
    error = ?e,
    "Failed to send email notification"
);

// Error level
error!(
    user_id = %user_id,
    error = ?e,
    "Database query failed"
);
```

**Log aggregation with Datadog/LogRocket:**
- Collect logs from both frontend and backend
- Set up log retention policies
- Create dashboards for log analysis

---

## Health Checks

### Endpoints

**Basic Health Check:**
```
GET /api/health
```

**Detailed Health Check:**
```
GET /api/health/detailed
```

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "database": {
    "status": "healthy",
    "connections": 10,
    "response_time_ms": 5
  },
  "uptime_seconds": 86400,
  "timestamp": "2026-02-16T10:00:00Z"
}
```

### Monitoring Service Integration

**Configure health check monitoring:**
- **UptimeRobot:** Free tier, 5-minute intervals
- **Pingdom:** More advanced monitoring
- **Datadog Synthetics:** Comprehensive monitoring

**Alert on:**
- Health check failures (3 consecutive failures)
- Response time > 5 seconds
- Database connection failures

---

## Alerting

### Critical Alerts (PagerDuty/Opsgenie)

**Trigger on:**
- API downtime > 1 minute
- Database connection failures
- Error rate > 5% over 5 minutes
- Memory usage > 90%
- CPU usage > 90% for 5 minutes

### Warning Alerts (Slack/Email)

**Trigger on:**
- Error rate > 1% over 15 minutes
- API response time > 1 second (p95)
- Failed authentication attempts > 100/hour
- Disk usage > 80%

---

## Metrics to Track

### Application Metrics

**Frontend:**
- Page load time (by route)
- API request duration (by endpoint)
- Error rate (by component)
- User session duration
- Feature usage (trades created, analytics viewed)

**Backend:**
- Request rate (requests/second)
- Response time (p50, p95, p99)
- Error rate (by endpoint)
- Database query time
- Active connections
- Memory usage
- CPU usage

### Business Metrics

- Daily active users (DAU)
- Trades created per day
- AI reviews requested per day
- User retention rate
- Feature adoption rate

---

## Dashboard Setup

### Recommended Tools

1. **Grafana** - Open-source dashboards
2. **Datadog** - All-in-one monitoring
3. **New Relic** - APM and monitoring

### Key Dashboards

**1. Application Health:**
- Request rate
- Error rate
- Response time (p50, p95, p99)
- Active users

**2. Infrastructure:**
- CPU usage
- Memory usage
- Disk usage
- Network I/O

**3. Business Metrics:**
- User signups
- Trades created
- AI reviews requested
- Feature usage

---

## Environment Variables

Add to `.env`:
```bash
# Sentry
VITE_SENTRY_DSN=https://...@sentry.io/...
SENTRY_DSN=https://...@sentry.io/...

# Datadog (optional)
DD_API_KEY=...
DD_SITE=datadoghq.com

# Environment
ENVIRONMENT=production
```

---

## Next Steps

1. ✅ Set up Sentry for error tracking
2. ✅ Configure health check monitoring
3. ✅ Set up log aggregation
4. ✅ Create Grafana/Datadog dashboards
5. ✅ Configure alerting rules
6. ✅ Test alert notifications
7. ✅ Document runbooks for common issues

