; Installer definition for  ne4_konfig
; Written by Stefan Müller <co@zzeroo.com>
; makensis.exe ne4_konfig-64bit.nsi
!makensis "CargoAttributes.nsi"
!system "CargoAttributes.exe"
!system "CargoAttributes.sh"
!include "CargoAttributes.txt"
; optional cleanup
!delfile "CargoAttributes.exe"
!delfile "CargoAttributes.sh"
!delfile "CargoAttributes.txt"

!include "MUI2.nsh"

Unicode true
SetCompressor /SOLID lzma

;--------------------------------
;Configuration
; 64bit version
!define BITS 32
!define ARCH "i686"

!ifndef OUTFILE
  !define OUTFILE "${CARGO_PKG_NAME}-${CARGO_PKG_VERSION}-windows-${BITS}bit-setup.exe"
!endif

OutFile "${OUTFILE}"
Name "NE4 Konfig"
Caption "NE4 Konfig ${CARGO_PKG_VERSION} ${BITS}Bit Setup"

;Default installation folder
InstallDir "$PROGRAMFILES\RA-GAS GmbH\${CARGO_PKG_NAME}"

;Get installation folder from registry if available
InstallDirRegKey HKCU "Software\RA-GAS GmbH\${CARGO_PKG_NAME}" ""

;Request application privileges for Windows Vista
RequestExecutionLevel admin

;--------------------------------
;Interface Settings

;!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\nsis3-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\nsis3-uninstall.ico"

!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Header\nsis3-branding.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "${NSISDIR}\Contrib\Graphics\Wizard\nsis3-branding.bmp"

;--------------------------------
;Pages

!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_COMPONENTS
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

;--------------------------------
;Languages

!insertmacro MUI_LANGUAGE "German"

;--------------------------------
;Installer Sections

Section "NE4 Konfig" SecNE4
  SetOutPath "$INSTDIR"

  File /r "${CARGO_PKG_NAME}-${CARGO_PKG_VERSION}-windows-${ARCH}\"

  ;Store installation folder
  WriteRegStr HKCU "Software\RA-GAS GmbH\${CARGO_PKG_NAME}" "" $INSTDIR

  ;Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; Desktop symbols
  CreateShortcut "$DESKTOP\NE4 Konfig.lnk" "$INSTDIR\${CARGO_PKG_NAME}.exe" "" "$INSTDIR\resources\${CARGO_PKG_NAME}.ico" 0

  ; Start menu
  CreateDirectory "$SMPROGRAMS\RA-GAS GmbH"
  CreateShortcut "$SMPROGRAMS\RA-GAS GmbH\Uninstall.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\uninstall.exe" 0
  CreateShortcut "$SMPROGRAMS\RA-GAS GmbH\${CARGO_PKG_NAME}.lnk" "$INSTDIR\${CARGO_PKG_NAME}.exe" "" "$INSTDIR\resources\${CARGO_PKG_NAME}.ico" 0

SectionEnd

;--------------------------------
;Descriptions

  ;Language strings
  LangString DESC_SecNE4 ${LANG_GERMAN} "Hauptprogramm 'NE4 Konfig'"

  ;Assign language strings to sections
  !insertmacro MUI_FUNCTION_DESCRIPTION_BEGIN
    !insertmacro MUI_DESCRIPTION_TEXT ${SecNE4} $(DESC_SecNE4)
  !insertmacro MUI_FUNCTION_DESCRIPTION_END

;--------------------------------
;Uninstaller Section

Section "Uninstall"
  Delete "$INSTDIR\resources\*.*"
  RMDIR /r "$INSTDIR\resources"
  Delete "$INSTDIR\share\*.*"
  RMDIR /r "$INSTDIR\share"
  Delete "$INSTDIR\Uninstall.exe"

  ; Remove shortcuts
  Delete "$DESKTOP\NE4 Konfig.lnk"
  Delete "$SMPROGRAMS\RA-GAS GmbH\*.*"

  ; Remove directories used
  RMDir "$SMPROGRAMS\RA-GAS GmbH"

  Delete "$INSTDIR\*.*"
  RMDir "$INSTDIR"

  DeleteRegKey /ifempty HKCU "Software\RA-GAS GmbH\${CARGO_PKG_NAME}"

SectionEnd
