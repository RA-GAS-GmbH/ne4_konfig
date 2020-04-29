GUI für die Konfiguration der 'NE4-MOD-BUS' Sensoren

# `NE4-MOD-BUS` Konfiguration

# Installation

## Linux

Die [Gitlab CI] bildet ein Archiv mit allen benötigten Dateien

# Qellcode selber übersetzen

## unter Linux

Neben Rust werden die Gtk und Udev Entwicklungs Bibliotheken verwendet.

Die Installation von Rust wird hier beschrieben: https://rustup.rs/

Die Gtk und Udev Bibliotheken können u.a. so installiert werden:
```bash
# debian/ ubuntu
apt install libudev-dev libgtk-3-dev
```

Die minimale Rust Version ist 1.42.


[Gitlab CI]: https://docs.gitlab.com/ee/ci/
