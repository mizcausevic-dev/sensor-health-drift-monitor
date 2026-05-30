// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Summary {
    pub robots: usize,
    pub active_sensors: usize,
    pub drift_alerts: usize,
    pub calibration_gaps: usize,
    pub blind_zones: usize,
    pub signal: &'static str,
}

#[derive(Clone, Serialize)]
pub struct SensorLaneRecord {
    pub robot_id: &'static str,
    pub mission: &'static str,
    pub lane: &'static str,
    pub sensor_focus: &'static str,
    pub owner: &'static str,
    pub calibration_hours: i32,
    pub status: &'static str,
    pub next_action: &'static str,
}

#[derive(Clone, Serialize)]
pub struct DriftFinding {
    pub finding_id: &'static str,
    pub robot_id: &'static str,
    pub sensor_type: &'static str,
    pub actor: &'static str,
    pub drift_score: i32,
    pub evidence_state: &'static str,
    pub status: &'static str,
    pub note: &'static str,
}

#[derive(Clone, Serialize)]
pub struct VerificationGate {
    pub packet_id: &'static str,
    pub audience: &'static str,
    pub completeness: i32,
    pub blocker: &'static str,
    pub status: &'static str,
    pub next_action: &'static str,
}

#[derive(Clone, Serialize)]
pub struct Payload {
    pub summary: Summary,
    pub sensor_lane: Vec<SensorLaneRecord>,
    pub drift_findings: Vec<DriftFinding>,
    pub verification: Vec<VerificationGate>,
}

pub fn sample_payload() -> Payload {
    Payload {
        summary: Summary {
            robots: 8,
            active_sensors: 42,
            drift_alerts: 6,
            calibration_gaps: 3,
            blind_zones: 2,
            signal: "Sensor drift usually shows up as mission confidence erosion before it becomes an incident. This surface keeps that pressure visible early.",
        },
        sensor_lane: vec![
            SensorLaneRecord {
                robot_id: "RB-204",
                mission: "warehouse pick fleet",
                lane: "depth / lidar",
                sensor_focus: "Lidar variance after overnight dock exposure",
                owner: "fleet reliability",
                calibration_hours: 36,
                status: "watch",
                next_action: "Re-run calibration bundle before the next night shift.",
            },
            SensorLaneRecord {
                robot_id: "RB-318",
                mission: "yard inspection route",
                lane: "thermal / camera",
                sensor_focus: "Thermal camera blind spot on left-side sweep",
                owner: "field robotics ops",
                calibration_hours: 14,
                status: "critical",
                next_action: "Hold the route and replace the thermal module now.",
            },
            SensorLaneRecord {
                robot_id: "RB-411",
                mission: "lab courier shuttle",
                lane: "imu / odometry",
                sensor_focus: "IMU drift affecting docking accuracy",
                owner: "controls engineering",
                calibration_hours: 18,
                status: "healthy",
                next_action: "Keep current tuning and preserve the calibration artifact.",
            },
            SensorLaneRecord {
                robot_id: "RB-527",
                mission: "fulfillment arm cell",
                lane: "force / torque",
                sensor_focus: "Force sensor jitter during fragile-item handling",
                owner: "cell automation lead",
                calibration_hours: 8,
                status: "watch",
                next_action: "Raise threshold review before the next packaging run.",
            },
            SensorLaneRecord {
                robot_id: "RB-633",
                mission: "facility patrol drone",
                lane: "gps / barometer",
                sensor_focus: "Altitude confidence loss during indoor-outdoor transitions",
                owner: "autonomy review board",
                calibration_hours: 5,
                status: "critical",
                next_action: "Disable autonomous transitions until the sensor fusion patch lands.",
            },
        ],
        drift_findings: vec![
            DriftFinding {
                finding_id: "DF-901",
                robot_id: "RB-204",
                sensor_type: "lidar",
                actor: "fleet reliability",
                drift_score: 78,
                evidence_state: "partial",
                status: "watch",
                note: "Variance rose after humid dock exposure; recalibration artifact still pending.",
            },
            DriftFinding {
                finding_id: "DF-918",
                robot_id: "RB-318",
                sensor_type: "thermal camera",
                actor: "field robotics ops",
                drift_score: 94,
                evidence_state: "missing",
                status: "critical",
                note: "Blind-side sweep confidence below safety threshold and module replacement proof is still open.",
            },
            DriftFinding {
                finding_id: "DF-930",
                robot_id: "RB-411",
                sensor_type: "imu",
                actor: "controls engineering",
                drift_score: 31,
                evidence_state: "ready",
                status: "healthy",
                note: "Docking variance narrowed after the last tune set; evidence packet preserved.",
            },
            DriftFinding {
                finding_id: "DF-947",
                robot_id: "RB-527",
                sensor_type: "force / torque",
                actor: "cell automation lead",
                drift_score: 68,
                evidence_state: "partial",
                status: "watch",
                note: "Force jitter is visible in fragile-item runs; operator note exists but threshold proof is incomplete.",
            },
            DriftFinding {
                finding_id: "DF-963",
                robot_id: "RB-633",
                sensor_type: "gps / barometer",
                actor: "autonomy review board",
                drift_score: 97,
                evidence_state: "missing",
                status: "critical",
                note: "Transition confidence collapsed at doorway boundaries and override guidance is not yet attached.",
            },
            DriftFinding {
                finding_id: "DF-972",
                robot_id: "RB-204",
                sensor_type: "depth camera",
                actor: "telemetry reviewer",
                drift_score: 52,
                evidence_state: "ready",
                status: "watch",
                note: "Depth jitter is tolerable but trending the wrong way; replay evidence is archived for review.",
            },
        ],
        verification: vec![
            VerificationGate {
                packet_id: "PK-17",
                audience: "fleet safety committee",
                completeness: 84,
                blocker: "Thermal replacement evidence still open",
                status: "watch",
                next_action: "Attach module swap proof and recalibration log.",
            },
            VerificationGate {
                packet_id: "PK-31",
                audience: "operations leadership",
                completeness: 93,
                blocker: "No active blocker",
                status: "healthy",
                next_action: "Hold for governed publication and next audit cycle.",
            },
            VerificationGate {
                packet_id: "PK-44",
                audience: "autonomy review board",
                completeness: 61,
                blocker: "Override and transition evidence chain still fragmented",
                status: "critical",
                next_action: "Reconcile sensor-fusion patch, override note, and route hold evidence before sign-off.",
            },
        ],
    }
}
