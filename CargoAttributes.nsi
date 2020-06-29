; CargoAttributes.nsi
;
; Helper .nsi script to extract information from Rust's Cargo.toml
; Put this in your main script:
;
; !makensis "CargoAttributes.nsi"
; !system "CargoAttributes.exe"
; !system "CargoAttributes.sh"
; !include "CargoAttributes.txt"
; ; optional cleanup
; !delfile "CargoAttributes.exe"
; !delfile "CargoAttributes.sh"
; !delfile "CargoAttributes.txt"
!include "TextFunc.nsh"
Unicode true

OutFile CargoAttributes.exe
SilentInstall silent
RequestExecutionLevel user

Section
  ${ConfigRead} Cargo.toml "name =" $R1
  ${ConfigRead} Cargo.toml "version =" $R2

  ## Write it to a !define for use in main script
  FileOpen $R0 "$EXEDIR\CargoAttributes.txt" w
    FileWrite $R0 '!define CARGO_PKG_NAME $R1$\r$\n'
    FileWrite $R0 '!define CARGO_PKG_VERSION $R2'
  FileClose $R0
SectionEnd
