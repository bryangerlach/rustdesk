lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Ihr Desktop"),
        ("desk_tip", "Mit dieser ID und diesem Passwort kann auf Ihren Desktop zugegriffen werden."),
        ("Password", "Passwort"),
        ("Ready", "Bereit"),
        ("Established", "Verbunden"),
        ("connecting_status", "Verbinden mit dem RustDesk-Netzwerk …"),
        ("Enable Service", "Vermittlungsdienst aktivieren"),
        ("Start Service", "Vermittlungsdienst starten"),
        ("Service is running", "Vermittlungsdienst aktiv"),
        ("Service is not running", "Vermittlungsdienst deaktiviert"),
        ("not_ready_status", "Nicht bereit. Bitte überprüfen Sie Ihre Netzwerkverbindung."),
        ("Control Remote Desktop", "Entfernten Desktop steuern"),
        ("Transfer File", "Datei übertragen"),
        ("Connect", "Verbinden"),
        ("Recent Sessions", "Letzte Sitzungen"),
        ("Address Book", "Adressbuch"),
        ("Confirmation", "Bestätigung"),
        ("TCP Tunneling", "TCP-Tunnelung"),
        ("Remove", "Entfernen"),
        ("Refresh random password", "Zufälliges Passwort erzeugen"),
        ("Set your own password", "Eigenes Passwort setzen"),
        ("Enable Keyboard/Mouse", "Tastatur und Maus aktivieren"),
        ("Enable Clipboard", "Zwischenablage aktivieren"),
        ("Enable File Transfer", "Dateiübertragung aktivieren"),
        ("Enable TCP Tunneling", "TCP-Tunnelung aktivieren"),
        ("IP Whitelisting", "IP-Whitelist"),
        ("ID/Relay Server", "ID/Relay-Server"),
        ("Import Server Config", "Serverkonfiguration importieren"),
        ("Export Server Config", "Serverkonfiguration exportieren"),
        ("Import server configuration successfully", "Serverkonfiguration erfolgreich importiert"),
        ("Export server configuration successfully", "Serverkonfiguration erfolgreich exportiert"),
        ("Invalid server configuration", "Ungültige Serverkonfiguration"),
        ("Clipboard is empty", "Zwischenablage ist leer"),
        ("Stop service", "Vermittlungsdienst stoppen"),
        ("Change ID", "ID ändern"),
        ("Your new ID", "Ihre neue ID"),
        ("length %min% to %max%", "Länge %min% bis %max%"),
        ("starts with a letter", "Beginnt mit Buchstabe"),
        ("allowed characters", "Erlaubte Zeichen"),
        ("id_change_tip", "Nur die Zeichen a-z, A-Z, 0-9 und _ (Unterstrich) sind erlaubt. Der erste Buchstabe muss a-z, A-Z sein und die Länge zwischen 6 und 16 Zeichen betragen."),
        ("Website", "Webseite"),
        ("About", "Über"),
        ("Slogan_tip", "Mit Herzblut programmiert - in einer Welt, die im Chaos versinkt!"),
        ("Privacy Statement", "Datenschutz"),
        ("Mute", "Stummschalten"),
        ("Build Date", "Erstelldatum"),
        ("Version", "Version"),
        ("Home", "Startseite"),
        ("Audio Input", "Audioeingang"),
        ("Enhancements", "Verbesserungen"),
        ("Hardware Codec", "Hardware-Codec"),
        ("Adaptive Bitrate", "Bitrate automatisch anpassen"),
        ("ID Server", "ID-Server"),
        ("Relay Server", "Relay-Server"),
        ("API Server", "API-Server"),
        ("invalid_http", "Muss mit http:// oder https:// beginnen"),
        ("Invalid IP", "Ungültige IP-Adresse"),
        ("Invalid format", "Ungültiges Format"),
        ("server_not_support", "Diese Funktion wird noch nicht vom Server unterstützt."),
        ("Not available", "Nicht verfügbar"),
        ("Too frequent", "Zu häufig"),
        ("Cancel", "Abbrechen"),
        ("Skip", "Überspringen"),
        ("Close", "Schließen"),
        ("Retry", "Erneut versuchen"),
        ("OK", "OK"),
        ("Password Required", "Passwort erforderlich"),
        ("Please enter your password", "Bitte geben Sie Ihr Passwort ein"),
        ("Remember password", "Passwort merken"),
        ("Wrong Password", "Falsches Passwort"),
        ("Do you want to enter again?", "Erneut verbinden?"),
        ("Connection Error", "Verbindungsfehler"),
        ("Error", "Fehler"),
        ("Reset by the peer", "Verbindung wurde von der Gegenstelle zurückgesetzt."),
        ("Connecting...", "Verbindung wird hergestellt …"),
        ("Connection in progress. Please wait.", "Die Verbindung wird hergestellt. Bitte warten …"),
        ("Please try 1 minute later", "Bitte versuchen Sie es später erneut"),
        ("Login Error", "Anmeldefehler"),
        ("Successful", "Erfolgreich"),
        ("Connected, waiting for image...", "Verbindung hergestellt. Warte auf anderen Bildschirm …"),
        ("Name", "Name"),
        ("Type", "Typ"),
        ("Modified", "Geändert"),
        ("Size", "Größe"),
        ("Show Hidden Files", "Versteckte Dateien anzeigen"),
        ("Receive", "Empfangen"),
        ("Send", "Senden"),
        ("Refresh File", "Datei aktualisieren"),
        ("Local", "Lokal"),
        ("Remote", "Entfernt"),
        ("Remote Computer", "Entfernter Computer"),
        ("Local Computer", "Dieser Computer"),
        ("Confirm Delete", "Löschen bestätigen"),
        ("Delete", "Löschen"),
        ("Properties", "Eigenschaften"),
        ("Multi Select", "Mehrfachauswahl"),
        ("Select All", "Alles auswählen"),
        ("Unselect All", "Alles abwählen"),
        ("Empty Directory", "Leerer Ordner"),
        ("Not an empty directory", "Ordner ist nicht leer."),
        ("Are you sure you want to delete this file?", "Sind Sie sicher, dass Sie diese Datei löschen wollen?"),
        ("Are you sure you want to delete this empty directory?", "Sind Sie sicher, dass Sie diesen leeren Ordner löschen möchten?"),
        ("Are you sure you want to delete the file of this directory?", "Sind Sie sicher, dass Sie die Datei dieses Ordners löschen möchten?"),
        ("Do this for all conflicts", "Für alle Konflikte merken"),
        ("This is irreversible!", "Dies kann nicht rückgängig gemacht werden!"),
        ("Deleting", "Löschen"),
        ("files", "Dateien"),
        ("Waiting", "Warten"),
        ("Finished", "Fertiggestellt"),
        ("Speed", "Geschwindigkeit"),
        ("Custom Image Quality", "Benutzerdefinierte Bildqualität"),
        ("Privacy mode", "Datenschutz-Modus"),
        ("Block user input", "Benutzereingaben blockieren"),
        ("Unblock user input", "Benutzereingaben freigeben"),
        ("Adjust Window", "Fenster anpassen"),
        ("Original", "Original"),
        ("Shrink", "Verkleinern"),
        ("Stretch", "Strecken"),
        ("Scrollbar", "Scroll-Leiste"),
        ("ScrollAuto", "Automatisch scrollen"),
        ("Good image quality", "Hohe Bildqualität"),
        ("Balanced", "Ausgeglichen"),
        ("Optimize reaction time", "Geschwindigkeit"),
        ("Custom", "Benutzerdefiniert"),
        ("Show remote cursor", "Entfernten Cursor anzeigen"),
        ("Show quality monitor", "Qualitätsüberwachung anzeigen"),
        ("Disable clipboard", "Zwischenablage deaktivieren"),
        ("Lock after session end", "Nach Sitzungsende sperren"),
        ("Insert", "Einfügen"),
        ("Insert Lock", "Win+L (Sperren) senden"),
        ("Refresh", "Aktualisieren"),
        ("ID does not exist", "Diese ID existiert nicht."),
        ("Failed to connect to rendezvous server", "Verbindung zum Rendezvous-Server fehlgeschlagen"),
        ("Please try later", "Bitte versuchen Sie es später erneut."),
        ("Remote desktop is offline", "Entfernter Desktop ist offline."),
        ("Key mismatch", "Schlüssel stimmen nicht überein."),
        ("Timeout", "Zeitüberschreitung"),
        ("Failed to connect to relay server", "Verbindung zum Relay-Server ist fehlgeschlagen"),
        ("Failed to connect via rendezvous server", "Verbindung über Rendezvous-Server ist fehlgeschlagen"),
        ("Failed to connect via relay server", "Verbindung über Relay-Server ist fehlgeschlagen"),
        ("Failed to make direct connection to remote desktop", "Direkte Verbindung zum entfernten Desktop ist fehlgeschlagen"),
        ("Set Password", "Passwort festlegen"),
        ("OS Password", "Betriebssystem-Passwort"),
        ("install_tip", "Aufgrund der Benutzerkontensteuerung (UAC) kann RustDesk in manchen Fällen nicht ordnungsgemäß funktionieren. Um die Benutzerkontensteuerung zu umgehen, klicken Sie bitte auf die Schaltfläche unten und installieren RustDesk auf dem System."),
        ("Click to upgrade", "Upgrade"),
        ("Click to download", "Zum Herunterladen klicken"),
        ("Click to update", "Update"),
        ("Configure", "Konfigurieren"),
        ("config_acc", "Um Ihren PC aus der Ferne zu steuern, müssen Sie RustDesk Zugriffsrechte erteilen."),
        ("config_screen", "Um aus der Ferne auf Ihren PC zugreifen zu können, müssen Sie RustDesk die Berechtigung \"Bildschirmaufnahme\" erteilen."),
        ("Installing ...", "Wird installiert …"),
        ("Install", "Installieren"),
        ("Installation", "Installation"),
        ("Installation Path", "Installationspfad"),
        ("Create start menu shortcuts", "Verknüpfung im Startmenü erstellen"),
        ("Create desktop icon", "Desktop-Verknüpfung erstellen"),
        ("agreement_tip", "Durch die Installation akzeptieren Sie die Lizenzvereinbarung."),
        ("Accept and Install", "Akzeptieren und Installieren"),
        ("End-user license agreement", "Lizenzvereinbarung für Endbenutzer"),
        ("Generating ...", "Wird generiert …"),
        ("Your installation is lower version.", "Ihre Version ist veraltet."),
        ("not_close_tcp_tip", "Schließen Sie dieses Fenster nicht, solange Sie den Tunnel benutzen."),
        ("Listening ...", "Lauschen …"),
        ("Remote Host", "Entfernter PC"),
        ("Remote Port", "Entfernter Port"),
        ("Action", "Aktion"),
        ("Add", "Hinzufügen"),
        ("Local Port", "Lokaler Port"),
        ("Local Address", "Lokale Adresse"),
        ("Change Local Port", "Lokalen Port ändern"),
        ("setup_server_tip", "für eine schnellere Verbindung richten Sie bitte Ihren eigenen Server ein."),
        ("Too short, at least 6 characters.", "Zu kurz, mindestens 6 Zeichen."),
        ("The confirmation is not identical.", "Die Passwörter stimmen nicht überein."),
        ("Permissions", "Berechtigungen"),
        ("Accept", "Akzeptieren"),
        ("Dismiss", "Ablehnen"),
        ("Disconnect", "Verbindung trennen"),
        ("Allow using keyboard and mouse", "Verwendung von Maus und Tastatur zulassen"),
        ("Allow using clipboard", "Verwendung der Zwischenablage zulassen"),
        ("Allow hearing sound", "System-Audio übertragen"),
        ("Allow file copy and paste", "Kopieren und Einfügen von Dateien zulassen"),
        ("Connected", "Verbunden"),
        ("Direct and encrypted connection", "Direkte und verschlüsselte Verbindung"),
        ("Relayed and encrypted connection", "Vermittelte und verschlüsselte Verbindung"),
        ("Direct and unencrypted connection", "Direkte und unverschlüsselte Verbindung"),
        ("Relayed and unencrypted connection", "Vermittelte und unverschlüsselte Verbindung"),
        ("Enter Remote ID", "Remote-ID eingeben"),
        ("Enter your password", "Geben Sie Ihr Passwort ein"),
        ("Logging in...", "Anmelden …"),
        ("Enable RDP session sharing", "RDP-Sitzungsfreigabe aktivieren"),
        ("Auto Login", "Automatisch anmelden (nur gültig, wenn Sie \"Nach Sitzungsende sperren\" aktiviert haben)"),
        ("Enable Direct IP Access", "Direkten IP-Zugang aktivieren"),
        ("Rename", "Umbenennen"),
        ("Space", "Speicherplatz"),
        ("Create Desktop Shortcut", "Desktop-Verknüpfung erstellen"),
        ("Change Path", "Pfad ändern"),
        ("Create Folder", "Ordner erstellen"),
        ("Please enter the folder name", "Bitte geben Sie den Ordnernamen ein"),
        ("Fix it", "Reparieren"),
        ("Warning", "Warnung"),
        ("Login screen using Wayland is not supported", "Anmeldebildschirm mit Wayland wird nicht unterstützt."),
        ("Reboot required", "Neustart erforderlich"),
        ("Unsupported display server", "Nicht unterstützter Anzeigeserver"),
        ("x11 expected", "X11 erwartet"),
        ("Port", "Port"),
        ("Settings", "Einstellungen"),
        ("Username", "Benutzername"),
        ("Invalid port", "Ungültiger Port"),
        ("Closed manually by the peer", "Von der Gegenstelle manuell geschlossen."),
        ("Enable remote configuration modification", "Änderung der Konfiguration aus der Ferne zulassen"),
        ("Run without install", "Ohne Installation ausführen"),
        ("Connect via relay", "Über Relay-Server verbinden"),
        ("Always connect via relay", "Immer über Relay-Server verbinden"),
        ("whitelist_tip", "Nur IPs auf der Whitelist können zugreifen."),
        ("Login", "Anmelden"),
        ("Verify", "Überprüfen"),
        ("Remember me", "Login merken"),
        ("Trust this device", "Diesem Gerät vertrauen"),
        ("Verification code", "Verifizierungscode"),
        ("verification_tip", "Es wurde ein neues Gerät erkannt und ein Verifizierungscode an die registrierte E-Mail-Adresse gesendet. Geben Sie den Verifizierungscode ein, um sich erneut anzumelden."),
        ("Logout", "Abmelden"),
        ("Tags", "Schlagworte"),
        ("Search ID", "ID suchen"),
        ("whitelist_sep", "Getrennt durch Komma, Semikolon, Leerzeichen oder Zeilenumbruch"),
        ("Add ID", "ID hinzufügen"),
        ("Add Tag", "Stichwort hinzufügen"),
        ("Unselect all tags", "Alle Stichworte abwählen"),
        ("Network error", "Netzwerkfehler"),
        ("Username missed", "Benutzername fehlt"),
        ("Password missed", "Passwort fehlt"),
        ("Wrong credentials", "Falsche Anmeldedaten"),
        ("Edit Tag", "Schlagwort bearbeiten"),
        ("Unremember Password", "Gespeichertes Passwort löschen"),
        ("Favorites", "Favoriten"),
        ("Add to Favorites", "Zu Favoriten hinzufügen"),
        ("Remove from Favorites", "Aus Favoriten entfernen"),
        ("Empty", "Keine Einträge"),
        ("Invalid folder name", "Ungültiger Ordnername"),
        ("Socks5 Proxy", "SOCKS5-Proxy"),
        ("Hostname", "Hostname"),
        ("Discovered", "Im LAN erkannt"),
        ("install_daemon_tip", "Um mit System zu starten, muss der Systemdienst installiert sein."),
        ("Remote ID", "Entfernte ID"),
        ("Paste", "Einfügen"),
        ("Paste here?", "Hier einfügen?"),
        ("Are you sure to close the connection?", "Möchten Sie diese Verbindung wirklich schließen?"),
        ("Download new version", "Neue Version herunterladen"),
        ("Touch mode", "Touch-Modus"),
        ("Mouse mode", "Mausmodus"),
        ("One-Finger Tap", "1-Finger-Tipp"),
        ("Left Mouse", "Linksklick"),
        ("One-Long Tap", "1-Finger-Halten"),
        ("Two-Finger Tap", "2-Finger-Tipp"),
        ("Right Mouse", "Rechtsklick"),
        ("One-Finger Move", "Einen Finger bewegen"),
        ("Double Tap & Move", "Doppeltippen und bewegen"),
        ("Mouse Drag", "Maus bewegen"),
        ("Three-Finger vertically", "Drei Finger vertikal bewegen"),
        ("Mouse Wheel", "Mausrad"),
        ("Two-Finger Move", "Zwei Finger bewegen"),
        ("Canvas Move", "Sichtfeld bewegen"),
        ("Pinch to Zoom", "2-Finger-Zoom"),
        ("Canvas Zoom", "Sichtfeld-Zoom"),
        ("Reset canvas", "Sichtfeld zurücksetzen"),
        ("No permission of file transfer", "Keine Berechtigung für die Dateiübertragung"),
        ("Note", "Hinweis"),
        ("Connection", "Verbindung"),
        ("Share Screen", "Bildschirm freigeben"),
        ("Chat", "Chat"),
        ("Total", "Gesamt"),
        ("items", "Einträge"),
        ("Selected", "Ausgewählt"),
        ("Screen Capture", "Bildschirmaufnahme"),
        ("Input Control", "Eingabesteuerung"),
        ("Audio Capture", "Audioaufnahme"),
        ("File Connection", "Dateiverbindung"),
        ("Screen Connection", "Bildschirmverbindung"),
        ("Do you accept?", "Verbindung zulassen?"),
        ("Open System Setting", "Systemeinstellung öffnen"),
        ("How to get Android input permission?", "Wie erhalte ich eine Android-Eingabeberechtigung?"),
        ("android_input_permission_tip1", "Damit ein entferntes Gerät Ihr Android-Gerät steuern kann, müssen Sie RustDesk erlauben, den Dienst \"Barrierefreiheit\" zu verwenden."),
        ("android_input_permission_tip2", "Bitte gehen Sie zur nächsten Systemeinstellungsseite, suchen Sie \"Installierte Dienste\" und schalten Sie den Dienst \"RustDesk Input\" ein."),
        ("android_new_connection_tip", "möchte Ihr Gerät steuern."),
        ("android_service_will_start_tip", "Durch das Aktivieren der Bildschirmfreigabe wird der Dienst automatisch gestartet, sodass andere Geräte dieses Android-Gerät steuern können."),
        ("android_stop_service_tip", "Durch das Deaktivieren des Dienstes werden automatisch alle hergestellten Verbindungen getrennt."),
        ("android_version_audio_tip", "Ihre Android-Version unterstützt keine Audioaufnahme, bitte aktualisieren Sie auf Android 10 oder höher, falls möglich."),
        ("android_start_service_tip", "Tippen Sie auf \"Vermittlungsdienst starten\" oder aktivieren Sie die Berechtigung \"Bildschirmaufnahme\", um den Bildschirmfreigabedienst zu starten."),
        ("android_permission_may_not_change_tip", "Die Berechtigungen für bestehende Verbindungen können nicht sofort geändert werden, bis die Verbindung wiederhergestellt ist."),
        ("Account", "Konto"),
        ("Overwrite", "Überschreiben"),
        ("This file exists, skip or overwrite this file?", "Diese Datei existiert; überspringen oder überschreiben?"),
        ("Quit", "Beenden"),
        ("doc_mac_permission", "https://rustdesk.com/docs/de/manual/mac/#berechtigungen-aktivieren"),
        ("Help", "Hilfe"),
        ("Failed", "Fehlgeschlagen"),
        ("Succeeded", "Erfolgreich"),
        ("Someone turns on privacy mode, exit", "Jemand hat den Datenschutzmodus aktiviert, wird beendet …"),
        ("Unsupported", "Nicht unterstützt"),
        ("Peer denied", "Die Gegenstelle hat die Verbindung abgelehnt."),
        ("Please install plugins", "Bitte installieren Sie Plugins"),
        ("Peer exit", "Die Gegenstelle hat die Verbindung getrennt."),
        ("Failed to turn off", "Ausschalten fehlgeschlagen"),
        ("Turned off", "Ausgeschaltet"),
        ("In privacy mode", "Datenschutzmodus aktivieren"),
        ("Out privacy mode", "Datenschutzmodus deaktivieren"),
        ("Language", "Sprache"),
        ("Keep RustDesk background service", "RustDesk im Hintergrund ausführen"),
        ("Ignore Battery Optimizations", "Akkuoptimierung ignorieren"),
        ("android_open_battery_optimizations_tip", "Möchten Sie die Einstellungen zur Akkuoptimierung öffnen?"),
        ("Start on Boot", "Beim Booten starten"),
        ("Start the screen sharing service on boot, requires special permissions", "Bildschirmfreigabedienst beim Booten starten, erfordert zusätzliche Berechtigungen"),
        ("Connection not allowed", "Verbindung abgelehnt"),
        ("Legacy mode", "Kompatibilitätsmodus"),
        ("Map mode", "Kartenmodus"),
        ("Translate mode", "Übersetzungsmodus"),
        ("Use permanent password", "Permanentes Passwort verwenden"),
        ("Use both passwords", "Beide Passwörter verwenden"),
        ("Set permanent password", "Permanentes Passwort setzen"),
        ("Enable Remote Restart", "Entfernten Neustart aktivieren"),
        ("Allow remote restart", "Entfernten Neustart erlauben"),
        ("Restart Remote Device", "Entferntes Gerät neu starten"),
        ("Are you sure you want to restart", "Möchten Sie das entfernte Gerät wirklich neu starten?"),
        ("Restarting Remote Device", "Entferntes Gerät wird neu gestartet"),
        ("remote_restarting_tip", "Entferntes Gerät startet neu, bitte schließen Sie diese Meldung und verbinden Sie sich mit dem permanenten Passwort erneut."),
        ("Copied", "Kopiert"),
        ("Exit Fullscreen", "Vollbild beenden"),
        ("Fullscreen", "Vollbild"),
        ("Mobile Actions", "Mobile Aktionen"),
        ("Select Monitor", "Bildschirm auswählen"),
        ("Control Actions", "Aktionen"),
        ("Display Settings", "Anzeigeeinstellungen"),
        ("Ratio", "Verhältnis"),
        ("Image Quality", "Bildqualität"),
        ("Scroll Style", "Scroll-Stil"),
        ("Show Menubar", "Menüleiste anzeigen"),
        ("Hide Menubar", "Menüleiste ausblenden"),
        ("Direct Connection", "Direkte Verbindung"),
        ("Relay Connection", "Relay-Verbindung"),
        ("Secure Connection", "Sichere Verbindung"),
        ("Insecure Connection", "Unsichere Verbindung"),
        ("Scale original", "Keine Skalierung"),
        ("Scale adaptive", "Anpassbare Skalierung"),
        ("General", "Allgemein"),
        ("Security", "Sicherheit"),
        ("Theme", "Farbgebung"),
        ("Dark Theme", "Dunkle Farbgebung"),
        ("Light Theme", "Helle Farbgebung"),
        ("Dark", "Dunkel"),
        ("Light", "Hell"),
        ("Follow System", "Systemstandard"),
        ("Enable hardware codec", "Hardware-Codec aktivieren"),
        ("Unlock Security Settings", "Sicherheitseinstellungen entsperren"),
        ("Enable Audio", "Audio aktivieren"),
        ("Unlock Network Settings", "Netzwerkeinstellungen entsperren"),
        ("Server", "Server"),
        ("Direct IP Access", "Direkter IP-Zugriff"),
        ("Proxy", "Proxy"),
        ("Apply", "Anwenden"),
        ("Disconnect all devices?", "Alle Geräte trennen?"),
        ("Clear", "Zurücksetzen"),
        ("Audio Input Device", "Audioeingabegerät"),
        ("Deny remote access", "Fernzugriff verbieten"),
        ("Use IP Whitelisting", "IP-Whitelist verwenden"),
        ("Network", "Netzwerk"),
        ("Enable RDP", "RDP aktivieren"),
        ("Pin menubar", "Menüleiste anpinnen"),
        ("Unpin menubar", "Menüleiste lösen"),
        ("Recording", "Aufnahme"),
        ("Directory", "Verzeichnis"),
        ("Automatically record incoming sessions", "Eingehende Sitzungen automatisch aufzeichnen"),
        ("Change", "Ändern"),
        ("Start session recording", "Sitzungsaufzeichnung starten"),
        ("Stop session recording", "Sitzungsaufzeichnung beenden"),
        ("Enable Recording Session", "Sitzungsaufzeichnung aktivieren"),
        ("Allow recording session", "Sitzungsaufzeichnung erlauben"),
        ("Enable LAN Discovery", "LAN-Erkennung aktivieren"),
        ("Deny LAN Discovery", "LAN-Erkennung verbieten"),
        ("Write a message", "Nachricht schreiben"),
        ("Prompt", "Meldung"),
        ("Please wait for confirmation of UAC...", "Bitte auf die Bestätigung des Nutzers warten …"),
        ("elevated_foreground_window_tip", "Das aktuell geöffnete Fenster des ferngesteuerten Computers erfordert höhere Rechte. Deshalb ist es derzeit nicht möglich, die Maus und die Tastatur zu verwenden. Bitten Sie den Nutzer, dessen Computer Sie fernsteuern, das Fenster zu minimieren oder die Rechte zu erhöhen. Um dieses Problem zukünftig zu vermeiden, wird empfohlen, die Software auf dem ferngesteuerten Computer zu installieren."),
        ("Disconnected", "Verbindung abgebrochen"),
        ("Other", "Weitere Einstellungen"),
        ("Confirm before closing multiple tabs", "Nachfragen, wenn mehrere Tabs geschlossen werden"),
        ("Keyboard Settings", "Tastatureinstellungen"),
        ("Full Access", "Vollzugriff"),
        ("Screen Share", "Bildschirmfreigabe"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland erfordert Ubuntu 21.04 oder eine höhere Version."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland erfordert eine höhere Version der Linux-Distribution. Bitte versuchen Sie den X11-Desktop oder ändern Sie Ihr Betriebssystem."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Bitte wählen Sie den freizugebenden Bildschirm aus (Bedienung auf der Gegenseite)."),
        ("Show RustDesk", "RustDesk anzeigen"),
        ("This PC", "Dieser PC"),
        ("or", "oder"),
        ("Continue with", "Fortfahren mit"),
        ("Elevate", "Zugriff gewähren"),
        ("Zoom cursor", "Cursor vergrößern"),
        ("Accept sessions via password", "Sitzung mit Passwort bestätigen"),
        ("Accept sessions via click", "Sitzung mit einem Klick bestätigen"),
        ("Accept sessions via both", "Sitzung mit Klick und Passwort bestätigen"),
        ("Please wait for the remote side to accept your session request...", "Bitte warten Sie, bis die Gegenseite Ihre Sitzungsanfrage akzeptiert hat …"),
        ("One-time Password", "Einmalpasswort"),
        ("Use one-time password", "Einmalpasswort verwenden"),
        ("One-time password length", "Länge des Einmalpassworts"),
        ("Request access to your device", "Zugriff auf Ihr Gerät anfordern"),
        ("Hide connection management window", "Fenster zur Verwaltung der Verbindung verstecken"),
        ("hide_cm_tip", "Dies ist nur möglich, wenn der Zugriff über ein permanentes Passwort erfolgt."),
        ("wayland_experiment_tip", "Die Unterstützung von Wayland ist nur experimentell. Bitte nutzen Sie X11, wenn Sie einen unbeaufsichtigten Zugriff benötigen."),
        ("Right click to select tabs", "Register mit rechtem Mausklick auswählen"),
        ("Skipped", "Übersprungen"),
        ("Add to Address Book", "Zum Adressbuch hinzufügen"),
        ("Group", "Gruppe"),
        ("Search", "Suchen"),
        ("Closed manually by web console", "Manuell über die Webkonsole geschlossen"),
        ("Local keyboard type", "Lokaler Tastaturtyp"),
        ("Select local keyboard type", "Lokalen Tastaturtyp auswählen"),
        ("software_render_tip", "Wenn Sie eine Nvidia-Grafikkarte haben und sich das entfernte Fenster sofort nach dem Herstellen der Verbindung schließt, kann es helfen, den Nouveau-Treiber zu installieren und Software-Rendering zu verwenden. Ein Neustart der Software ist erforderlich."),
        ("Always use software rendering", "Software-Rendering immer verwenden"),
        ("config_input", "Um den entfernten Desktop mit der Tastatur steuern zu können, müssen Sie RustDesk die Berechtigung \"Input Monitoring\" erteilen."),
        ("config_microphone", "Um aus der Ferne sprechen zu können, müssen Sie RustDesk die Berechtigung \"Audio aufzeichnen\" erteilen."),
        ("request_elevation_tip", "Sie können auch erhöhte Rechte anfordern, wenn sich jemand auf der Gegenseite befindet."),
        ("Wait", "Warten"),
        ("Elevation Error", "Berechtigungsfehler"),
        ("Ask the remote user for authentication", "Den entfernten Benutzer zur Authentifizierung auffordern"),
        ("Choose this if the remote account is administrator", "Wählen Sie dies, wenn das entfernte Konto Administrator ist."),
        ("Transmit the username and password of administrator", "Übermitteln Sie den Benutzernamen und das Passwort des Administrators"),
        ("still_click_uac_tip", "Der entfernte Benutzer muss immer noch im UAC-Fenster von RustDesk auf \"Ja\" klicken."),
        ("Request Elevation", "Erhöhte Rechte anfordern"),
        ("wait_accept_uac_tip", "Bitte warten Sie, bis der entfernte Benutzer den UAC-Dialog akzeptiert hat."),
        ("Elevate successfully", "Erhöhung der Rechte erfolgreich"),
        ("uppercase", "Großbuchstaben"),
        ("lowercase", "Kleinbuchstaben"),
        ("digit", "Ziffern"),
        ("special character", "Sonderzeichen"),
        ("length>=8", "Länge ≥ 8"),
        ("Weak", "Schwach"),
        ("Medium", "Mittel"),
        ("Strong", "Stark"),
        ("Switch Sides", "Seiten wechseln"),
        ("Please confirm if you want to share your desktop?", "Bitte bestätigen Sie, wenn Sie Ihren Desktop freigeben möchten."),
        ("Display", "Anzeige"),
        ("Default View Style", "Standard-Ansichtsstil"),
        ("Default Scroll Style", "Standard-Scroll-Stil"),
        ("Default Image Quality", "Standard-Bildqualität"),
        ("Default Codec", "Standard-Codec"),
        ("Bitrate", "Bitrate"),
        ("FPS", "fps"),
        ("Auto", "Automatisch"),
        ("Other Default Options", "Weitere Standardeinstellungen"),
        ("Voice call", "Sprachanruf"),
        ("Text chat", "Text-Chat"),
        ("Stop voice call", "Sprachanruf beenden"),
        ("relay_hint_tip", "Wenn eine direkte Verbindung nicht möglich ist, können Sie versuchen, eine Verbindung über einen Relay-Server herzustellen. \nWenn Sie eine Relay-Verbindung beim ersten Versuch herstellen möchten, können Sie das Suffix \"/r\" an die ID anhängen oder die Option \"Immer über Relay-Server verbinden\" auf der Gegenstelle auswählen."),
        ("Reconnect", "Erneut verbinden"),
        ("Codec", "Codec"),
        ("Resolution", "Auflösung"),
        ("No transfers in progress", "Keine Übertragungen im Gange"),
        ("Set one-time password length", "Länge des Einmalpassworts festlegen"),
        ("idd_driver_tip", "Installieren Sie den virtuellen Anzeigetreiber, der verwendet wird, wenn Sie keine physischen Anzeigen haben."),
        ("confirm_idd_driver_tip", "Die Option zur Installation des virtuellen Anzeigetreibers ist aktiviert. Beachten Sie, dass ein Testzertifikat installiert wird, um dem virtuellen Anzeigetreiber zu vertrauen. Dieses Testzertifikat wird nur verwendet, um Rustdesk-Treibern zu vertrauen."),
        ("RDP Settings", "RDP-Einstellungen"),
        ("Sort by", "Sortieren nach"),
        ("New Connection", "Neue Verbindung"),
        ("Restore", "Verkleinern"),
        ("Minimize", "Minimieren"),
        ("Maximize", "Maximieren"),
        ("Your Device", "Ihr Gerät"),
        ("empty_recent_tip", "Ups, keine aktuellen Sitzungen!\nZeit, eine neue zu planen."),
        ("empty_favorite_tip", "Noch keine favorisierte Gegenstelle?\nLassen Sie uns jemanden finden, mit dem wir uns verbinden können und fügen Sie ihn zu Ihren Favoriten hinzu!"),
        ("empty_lan_tip", "Oh nein, es sieht so aus, als hätten wir noch keine Gegenstelle entdeckt."),
        ("empty_address_book_tip", "Oh je, es scheint, dass in Ihrem Adressbuch derzeit keine Gegenstellen aufgeführt sind."),
        ("eg: admin", "z. B.: admin"),
        ("Empty Username", "Leerer Benutzername"),
        ("Empty Password", "Leeres Passwort"),
        ("Me", "Ich"),
        ("identical_file_tip", "Diese Datei ist identisch mit der Datei der Gegenstelle."),
        ("show_monitors_tip", "Monitore in der Symbolleiste anzeigen"),
        ("View Mode", "Ansichtsmodus"),
        ("enter_rustdesk_passwd_tip", "RustDesk-Passwort eingeben."),
        ("remember_rustdesk_passwd_tip", "RustDesk-Passwort merken."),
        ("login_linux_tip", "Anmeldung am entfernten Linux-Konto"),
        ("login_linux_tooltip_tip", "Sie müssen sich an einem entfernten Linux-Konto anmelden, um eine X-Desktop-Sitzung zu eröffnen."),
    ].iter().cloned().collect();
}
