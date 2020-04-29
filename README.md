GUI für die Konfiguration der 'NE4-MOD-BUS' Sensoren

[![GitLab CI status](https://gitlab.com/RA-GAS-GmbH/ne4_konfig/badges/master/pipeline.svg)](https://gitlab.com/RA-GAS-GmbH/ne4_konfig/pipelines)
[![Appveyor CI status](https://ci.appveyor.com/api/projects/status/sqhnkrgqba67o4m4/branch/master?svg=true)](https://ci.appveyor.com/project/zzeroo/ne4-konfig/branch/master)


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
