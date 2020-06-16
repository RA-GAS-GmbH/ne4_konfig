# Dokumentation für den Release Prozess.

- [] `rustup update` Rust toolchains aktuell?
- [] `rustup default stable` Rust 'stable' toolchain ist default?
- [] `git checkout development` in den 'development' Branch wechseln
- [] evtl. alle lokalen Branches in development Zweig mergen
- [] stelle sicher das lokal *alle* Tests fehlerfrei durchlaufen werden
  - [] `cargo test` Cargo Tests fehlerfrei?
- [] Rust Code nach den Richtlinien des Rust Projekts formatieren
  - [] `cargo fmt`
- [] Test mit der 'nightly' Rust Version
  - [] `cargo +nightly test` Build und Test unter nighly ok?
- [] Versionsnummer in 'Cargo.toml' erhöht?
  - [] `git commit Cargo.toml -m "Bump Version Nummer"`
- [] Changelog aktuell? Wurde die Datei 'CHANGELOG.md' mit allen wichtigen Änderungen am System gefüllt?
  - [] Update der nächsten Version Nummer im 'CHANGELOG.md' https://keepachangelog.com/en/1.0.0/
    - [] aktuelles Tagesdatum neben der Version im 'CHANGELOG.md' stehen
    - [] `git commit CHANGELOG.md -m "Update Changelog"`
- eventuell muss nun noch einmal die geänderte 'Cargo.lock' in die
  Versionskontrolle aufgenommen werden `git commit -a -m "Finaler Commit vor Release"`
- [] `git push` Branch ins remote Repo pushen
- CI überprüft?
  - [] https://gitlab.com/RA-GAS-GmbH/ne4_konfig/pipelines Ok?
- [] `git checkout master` wechsele in den *master* Branch
- [] `git merge --no-ff development` merge den lokalen 'development' Branch
- [] `git tag vN.N.N` Tagge die Version
- [] `git push --tags`

## Release packen
## Windows Binaries (32 und 64Bit gemeinsam)
- [] `docker start -ai ne4_konfig-build`
