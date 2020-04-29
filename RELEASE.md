Dokumentation für den Release Prozess.

- [] `git checkout development` in den 'development' Branch wechseln
- [] evtl. alle lokalen Branches in development Zweig mergen
- [] stelle sicher das lokal *alle* Tests fehlerfrei durchlaufen werden
  - [] `cargo test` Cargo Tests fehlerfrei?
- [] Rust Code nach den Richtlinien des Rust Projekts formatieren
  - [] `cargo +nightly fmt`
- [] Changelog aktuell? Wurde die Datei 'CHANGELOG.md' mit allen wichtigen Änderungen am System gefüllt?
  - [] Update der nächsten Version Nummer im 'CHANGELOG.md' https://keepachangelog.com/en/1.0.0/
    - [] aktuelles Tagesdatum neben der Version im 'CHANGELOG.md' stehen
    - [] `git commit -a -m "Update Changelog"`
- [] `git push` Branch ins remote Repo pushen
- [] `git checkout master` wechsele in den *master* Branch
- [] `git merge --no-ff development` merge den lokalen 'development' Branch
- [] `git tag vN.N.N` Tagge die Version
- [] `git push`
