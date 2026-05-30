# Origin

We already had a broader robotics fleet surface, but the telemetry primitive was still too bundled into general exception handling.

The right split was not “another fleet dashboard in another language.” The right split was:

- fleet exception posture in C#
- sensor drift, calibration freshness, and evidence durability in Rust

That makes the Rust repo a real systems complement instead of a portfolio clone.
