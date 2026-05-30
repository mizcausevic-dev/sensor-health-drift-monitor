// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod core;
pub mod render;

#[cfg(test)]
mod tests {
    use super::core::sample_payload;

    #[test]
    fn payload_has_expected_shape() {
        let payload = sample_payload();
        assert_eq!(payload.summary.robots, 8);
        assert_eq!(payload.sensor_lane.len(), 5);
        assert_eq!(payload.drift_findings.len(), 6);
        assert_eq!(payload.verification.len(), 3);
        assert!(payload.summary.drift_alerts > 0);
    }
}
