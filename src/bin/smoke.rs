// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;

fn main() {
    let required = [
        ("site/index.html", "Sensor health drift monitor"),
        ("site/sensor-lane.html", "Sensor lane"),
        ("site/drift-findings.html", "Drift findings"),
        ("site/verification.html", "Verification posture"),
        ("site/docs.html", "System docs"),
        ("site/api/sample.json", "warehouse pick fleet"),
    ];

    for (path, needle) in required {
        let data = fs::read_to_string(path).unwrap();
        assert!(data.contains(needle), "{path} missing {needle}");
    }

    println!("smoke ok");
}
