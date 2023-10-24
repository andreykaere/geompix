; Script generated by the Inno Setup Script Wizard.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{0AD7C555-49E0-47FC-A6ED-22677FD5AD28}
AppName=Focus Annotator
AppVersion=0.1.1
;AppVerName=Focus Annotator 0.1.0
AppPublisher=Hannes Fredrik Kuchelmeister
AppPublisherURL=https://github.com/13hannes11/focus_annotator
AppSupportURL=https://github.com/13hannes11/focus_annotator
AppUpdatesURL=https://github.com/13hannes11/focus_annotator
DefaultDirName={autopf}\Focus Annotator
DisableProgramGroupPage=yes
LicenseFile=LICENSE.md
; Uncomment the following line to run in non administrative install mode (install for current user only.)
;PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog
OutputBaseFilename=focus_annotator_setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "package\focus-annotator.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "package\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{autoprograms}\Focus Annotator"; Filename: "{app}\focus-annotator.exe"
Name: "{autodesktop}\Focus Annotator"; Filename: "{app}\focus-annotator.exe"; Tasks: desktopicon

[Run]
Filename: "{app}\focus-annotator.exe"; Description: "{cm:LaunchProgram,Focus Annotator}"; Flags: nowait postinstall skipifsilent
