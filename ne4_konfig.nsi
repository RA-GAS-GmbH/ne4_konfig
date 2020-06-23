; Installer definition for  ne4_konfig
; Written by Stefan Müller <co@zzeroo.com>
; makensis.exe ne4_konfig.nsi

;--------------------------------
;Include Modern UI

  !include "MUI2.nsh"

;--------------------------------
;General
  !pragma warning error all
  !pragma warning warning 7010 ; File /NonFatal

  !ifdef VER_MAJOR & VER_MINOR
    !define /ifndef VER_REVISION 0
    !define /ifndef VER_BUILD 0
  !endif

  !define /ifndef VERSION 'test-build'

  ;--------------------------------
  ;Configuration

  !if ${NSIS_PTR_SIZE} > 4
    !define BITS 64
    !define NAMESUFFIX " (64 bit)"
    !define ARCH "x86_64"
  !else
    !define BITS 32
    !define NAMESUFFIX ""
    !define ARCH "i686"
  !endif

  !ifndef OUTFILE
    !define OUTFILE "ne4_konfig-${VERSION}-windows-${BITS}bit-setup.exe"
  !endif

  OutFile "${OUTFILE}"
  Unicode true
  SetCompressor /SOLID lzma

  Name "NE4 Konfig"
  Caption "NE4 Konfig ${VERSION}${NAMESUFFIX} Setup"

  ;Default installation folder
  InstallDir "$PROGRAMFILES\RA-GAS GmbH\ne4_konfig"

  ;Get installation folder from registry if available
  InstallDirRegKey HKCU "Software\RA-GAS GmbH\ne4_konfig" ""

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

  File /a /r "ne4_konfig-1.0.1-windows-${ARCH}\"

  ;Store installation folder
  WriteRegStr HKCU "Software\RA-GAS GmbH\ne4_konfig" "" $INSTDIR

  ;Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; Desktop symbols
  CreateShortcut "$DESKTOP\NE4 Konfig${NAMESUFFIX}.lnk" "$INSTDIR\ne4_konfig.exe"

  ; Start menu
  CreateDirectory "$SMPROGRAMS\RA-GAS GmbH"
  CreateShortcut "$SMPROGRAMS\RA-GAS GmbH\Uninstall.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\uninstall.exe" 0
  CreateShortcut "$SMPROGRAMS\RA-GAS GmbH\ne4_konfig.lnk" "$INSTDIR\ne4_konfig.exe" "" "$INSTDIR\ne4_konfig.exe" 0

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

  ;ADD YOUR OWN FILES HERE...

  Delete "$INSTDIR\Uninstall.exe"

  ; Remove shortcuts
  Delete "$DESKTOP\NE4 Konfig${NAMESUFFIX}.lnk"
  Delete "$SMPROGRAMS\RA-GAS GmbH\*.*"

  ; Remove directories used
  RMDir "$SMPROGRAMS\RA-GAS GmbH"
  RMDir "$INSTDIR"

  DeleteRegKey /ifempty HKCU "Software\RA-GAS GmbH\ne4_konfig"

SectionEnd
