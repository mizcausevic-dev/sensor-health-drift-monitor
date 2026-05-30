// SPDX-License-Identifier: AGPL-3.0-or-later

use sensor_health_drift_monitor::core::sample_payload;
use sensor_health_drift_monitor::render::{
    render_docs, render_drift_findings, render_overview, render_sensor_lane, render_verification,
};
use std::fs;
use std::path::Path;

fn main() {
    let site = Path::new("site");
    let _ = fs::remove_dir_all(site);
    fs::create_dir_all(site.join("api").join("dashboard").join("summary")).unwrap();

    write("site/index.html", &render_overview());
    write("site/sensor-lane.html", &render_sensor_lane());
    write("site/drift-findings.html", &render_drift_findings());
    write("site/verification.html", &render_verification());
    write("site/docs.html", &render_docs());

    let payload = sample_payload();
    write("site/api/dashboard/summary/index.json", &serde_json::to_string_pretty(&payload.summary).unwrap());
    write("site/api/sensor-lane.json", &serde_json::to_string_pretty(&payload.sensor_lane).unwrap());
    write("site/api/drift-findings.json", &serde_json::to_string_pretty(&payload.drift_findings).unwrap());
    write("site/api/verification.json", &serde_json::to_string_pretty(&payload.verification).unwrap());
    write("site/api/sample.json", &serde_json::to_string_pretty(&payload).unwrap());

    if let Ok(cname) = fs::read_to_string("CNAME") {
        write("site/CNAME", &cname);
    }
}

fn write(path: &str, content: &str) {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}
