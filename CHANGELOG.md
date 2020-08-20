# Changelog
Alle erwähnenswert Änderungen am Projekt werden in dieser Datei dokumentiert.
All notable changes to this project will be documented in this file.

Das Format der Datei basiert auf [Führe ein CHANGELOG](https://keepachangelog.com/de/1.0.0/),
außerdem befolgt dieses Projekt die [Semantische Versionierung](https://semver.org/lang/de/spec/v2.0.0.html)
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.5.0] - 2020-08-20
### Added
- RA-GAS Version `--features="ra-gas"`
- Rwreg Register können nun mit ausgelesen werden
## Changed
- Viel bessere Stabilität des Programms erreicht
  - Wärend des Arbeitens mit den seriellen Schnittstellen wird nun viel mehr
    auf die Stabilität des Programms geachtet. Das Programm stürzt nun viel
    weniger ab.

## [v1.0.1] - 2020-06-26
### Added
- 4-20mA Anzeige in der Sensoransicht
- NSIS Installer für 32/64Bit Windows Versionen
- About Dialog mit Informationen zur Software
## Changed
- Release Anweisungen präzisiert
- Screenshots in README aufgenommen
- Funktionen für Nullpunkt und Messgas Abgleich sind nicht mehr wärend der Live
    Ansicht aufrufbar
- "erweiterte Ansicht" umbenannt,
    es werden nur noch die lesbaren Register angezeigt

## [v1.0.0] - 2020-06-16
### Added
- Abfrage der Sensordaten komplett
- Modbus Adresse kann gesetzt werden
- Arbeitsweise des Sensors kann gesetzt werden
- erweiterte Ansicht mit alle Rreg Registern
- Statusleiste am unteren Bildschirmrand informiert über aktuelle Funktion der GUI

## [v0.3.0] - 2020-05-10
- Sensordaten werden nun fast vollständig angezeigt

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

## [v0.1.0-alpha] - 2020-04-29
### Added
- Erstes Release

## [v0.1.0] - 2020-04-29
### Added
- Gitlab CI wird für Linux Builds und Tests verwendet
- Appveyor CI wird für Windows Builds und Tests verwendet
-

[v1.0.1]: https://gitlab.com/RA-GAS-GmbH/ne4_konfig/-/tags/v1.0.1
[v1.0.0]: https://gitlab.com/RA-GAS-GmbH/ne4_konfig/-/tags/v1.0.0
[v0.3.0]: https://gitlab.com/RA-GAS-GmbH/ne4_konfig/-/releases#v0.3.0
[v0.2.0]: https://gitlab.com/RA-GAS-GmbH/ne4_konfig/-/releases#v0.2.0
[v0.1.0-alpha]: https://gitlab.com/RA-GAS-GmbH/ne4_konfig/-/releases#v0.1.0-alpha
[v0.1.0]: https://gitlab.com/RA-GAS-GmbH/ne4_konfig/-/tags/v0.1.0
