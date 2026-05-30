// SPDX-License-Identifier: AGPL-3.0-or-later

use sensor_health_drift_monitor::core::sample_payload;

fn main() {
    let payload = sample_payload();
    println!(
        "robots={} active_sensors={} drift_alerts={} calibration_gaps={} blind_zones={}",
        payload.summary.robots,
        payload.summary.active_sensors,
        payload.summary.drift_alerts,
        payload.summary.calibration_gaps,
        payload.summary.blind_zones
    );
}
