# archivar

`archivar` ist ein kleines Rust-CLI-Programm zur Analyse einer festen Ablagestruktur.

Aktuell durchsucht das Programm ein Verzeichnis `kanzlei` im aktuellen Arbeitsverzeichnis und meldet fuer passende Unterordner, wie alt deren letzter inhaltlicher Stand ist.

## Erwartete Verzeichnisstruktur

Das Programm wird im Projekt- oder Arbeitsverzeichnis gestartet und erwartet dort mindestens:

```text
.
└── kanzlei
    ├── 24
    │   ├── 123 Mandant A
    │   └── 456 Mandant B
    └── 25
        └── 789 Mandant C
```

Die Regeln sind:

- Unter `kanzlei` werden nur direkte Unterverzeichnisse betrachtet, deren Name aus genau zwei Ziffern besteht, zum Beispiel `24` oder `25`.
- Innerhalb dieser Jahresverzeichnisse werden nur direkte Unterverzeichnisse betrachtet, deren Name mit genau drei Ziffern beginnt, zum Beispiel `123 Mandant A`.

## Aktuelles Verhalten

Fuer jedes passende Verzeichnis innerhalb eines Jahresordners gibt das Programm eine Zeile auf `stdout` aus.

Die Altersbewertung basiert auf der letzten Aenderung im Inhalt des Verzeichnisses:

- Das Aenderungsdatum des Verzeichnisses selbst wird nicht beruecksichtigt.
- Beruecksichtigt werden nur Dateien und Unterverzeichnisse innerhalb des Verzeichnisses.
- Ist ein Verzeichnis leer, wird das explizit gemeldet.

Die Ausgabe verwendet diese Altersstufen:

- `letzte Aenderung innerhalb des letzten Jahres`
- `letzte Aenderung vor mehr als 1 Jahr`
- `letzte Aenderung vor mehr als 3 Jahren`
- `letzte Aenderung vor mehr als 6 Jahren`
- `letzte Aenderung vor mehr als 8 Jahren`

Fuer die Stufen `mehr als 6 Jahren` und `mehr als 8 Jahren` gilt eine Sonderregel:

- Massgeblich ist nicht direkt das Datum der letzten Aenderung.
- Stattdessen wird der naechste `1. Januar` nach dieser letzten Aenderung als Stichtag verwendet.

## Build Und Ausfuehrung

Alle Befehle werden im Repository-Wurzelverzeichnis ausgefuehrt.

```bash
cargo build
cargo run
```

Tests und Pruefungen:

```bash
cargo test
cargo fmt
cargo clippy --all-targets --all-features
```

## Beispielausgabe

```text
kanzlei\24\123 Mandant A: letzte Aenderung vor mehr als 3 Jahren
kanzlei\24\456 Mandant B: Verzeichnis ist leer
kanzlei\25\789 Mandant C: letzte Aenderung innerhalb des letzten Jahres
```

## Hinweise

- Wenn das Verzeichnis `kanzlei` im aktuellen Arbeitsverzeichnis fehlt, bricht das Programm mit einer Fehlermeldung ab.
- Die eigentliche Archivierungslogik ist noch nicht implementiert; der aktuelle Stand analysiert und klassifiziert nur Verzeichnisse.
