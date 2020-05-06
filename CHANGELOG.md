# Changelog
Alle erwähnenswert Änderungen am Projekt werden in dieser Datei dokumentiert.
All notable changes to this project will be documented in this file.

Das Format der Datei basiert auf [Führe ein CHANGELOG](https://keepachangelog.com/de/1.0.0/),
außerdem befolgt dieses Projekt die [Semantische Versionierung](https://semver.org/lang/de/spec/v2.0.0.html)
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.2.0] - 2020-05-06
### Added
- [Platformunabhängigkeit] die Anwendung wird für Windows und Linux Systeme entwickelt und getestet
- Automatische Überprüfung der seriellen Schnittstellen
    - beim Start der Anwendung werden die verfügbaren Schnittstellen am System gesucht
    der Anwender kann die verfügbaren Schnittstellen einfach über die Toolbar wechseln
- werden weitere Schnittstellen an den PC angeschlossen, stehen diese automatisch
  dem Anwender zur Verfügung.
### Changed
- Gitlab CI benutzt nun das `rust:latest` Image für `cargo fmt` und die Tests

## [v0.1.0] - 2020-04-29
### Added
- Gitlab CI wird für Linux Builds und Tests verwendet
- Appveyor CI wird für Windows Builds und Tests verwendet
