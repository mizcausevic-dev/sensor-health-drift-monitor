# Architecture

`sensor-health-drift-monitor` is a Rust + Axum operator surface for turning raw robotics telemetry pressure into durable drift, calibration, and override-readiness signals.

Core layers:

- `overview`
  - aggregate fleet posture
  - sensor confidence pressure
  - drift and blind-zone summary
- `sensor-lane`
  - robot-level sensor health rows
  - ownership and calibration age
  - next-action posture
- `drift-findings`
  - anomaly evidence history
  - sensor-specific drift scores
  - operator-safe status transitions
- `verification`
  - fleet review packet readiness
  - blocker and next-action posture for autonomy sign-off

The repo complements `robot-fleet-exception-board`:

- `robots.kineticgain.com` shows broader fleet exception posture
- `sensor.kineticgain.com` shows the narrow telemetry and drift layer

Together they keep the robotics observability primitive coherent without duplicating the same operator story.
