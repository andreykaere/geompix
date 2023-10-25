
[Setup]
AppId={{39A2CF87-F636-4D1A-97E8-7CB1A0CA4FA8}
AppName=Geompix
AppVersion=0.0.1
DisableProgramGroupPage=yes
LicenseFile=LICENSE.md
; Uncomment the following line to run in non administrative install mode (install for current user only.)
PrivilegesRequired=lowest
DefaultDirName={autopf}\Geompix
; LicenseFile=LICENSE.md
PrivilegesRequiredOverridesAllowed=dialog
OutputBaseFilename=geompix
Compression=lzma
SolidCompression=yes
WizardStyle=modern


[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "package\geompix.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "package\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{autoprograms}\Focus Annotator"; Filename: "{app}\geompix.exe"
Name: "{autodesktop}\Focus Annotator"; Filename: "{app}\geompix.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\geompix.exe"; Description: "{cm:LaunchProgram,Focus Annotator}"; Flags: nowait postinstall skipifsilent
