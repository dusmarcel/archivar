# archivar

`archivar` ist ein kleines Rust-CLI-Programm zur Analyse einer festen Ablagestruktur.

Aktuell durchsucht das Programm feste Top-Level-Verzeichnisse im aktuellen Arbeitsverzeichnis und meldet fuer passende Unterordner, wie alt deren letzter inhaltlicher Stand ist.

## Erwartete Verzeichnisstruktur

Das Programm wird im Projekt- oder Arbeitsverzeichnis gestartet und verarbeitet diese Top-Level-Verzeichnisse, falls sie vorhanden sind:

- `kanzlei`
- `ablage2`
- `ablage4`
- `ablage6`
- `ablage8`

Beispielstruktur:

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

- Unter jedem vorhandenen Top-Level-Verzeichnis werden nur direkte Unterverzeichnisse betrachtet, deren Name aus genau zwei Ziffern besteht, zum Beispiel `24` oder `25`.
- Innerhalb dieser Jahresverzeichnisse werden nur direkte Unterverzeichnisse betrachtet, deren Name mit genau drei Ziffern beginnt, zum Beispiel `123 Mandant A`.

## Aktuelles Verhalten

Fuer jedes passende Verzeichnis innerhalb eines Jahresordners gibt das Programm eine Zeile auf `stdout` aus.

Die Altersbewertung basiert auf der letzten Aenderung im Inhalt des Verzeichnisses:

- Das Aenderungsdatum des Verzeichnisses selbst wird nicht beruecksichtigt.
- Beruecksichtigt werden nur Dateien und Unterverzeichnisse innerhalb des Verzeichnisses.
- Ist ein Verzeichnis leer, wird das explizit gemeldet oder optional entfernt.

Die Ausgabe verwendet diese Altersstufen:

- `letzte Aenderung innerhalb der letzten 2 Jahre`
- `letzte Aenderung vor mehr als 2 Jahren`
- `letzte Aenderung vor mehr als 4 Jahren`
- `letzte Aenderung vor mehr als 6 Jahren`
- `letzte Aenderung vor mehr als 8 Jahren`

Fuer die Stufen `mehr als 6 Jahren` und `mehr als 8 Jahren` gilt eine Sonderregel:

- Massgeblich ist nicht direkt das Datum der letzten Aenderung.
- Stattdessen wird der naechste `1. Januar` nach dieser letzten Aenderung als Stichtag verwendet.

Leere Verzeichnisse verhalten sich so:

- Ohne `--remove` werden sie nur gemeldet.
- Mit `--remove` werden sie geloescht.
- Mit `--dry-run --remove` wird nur ausgegeben, was geloescht wuerde.

## Build Und Ausfuehrung

Alle Befehle werden im Repository-Wurzelverzeichnis ausgefuehrt.

```bash
cargo build
cargo run
cargo run -- --dry-run
cargo run -- --remove
```

Tests und Pruefungen:

```bash
cargo test
cargo fmt
cargo clippy --all-targets --all-features
```

## Beispielausgabe

```text
kanzlei/24/123 Mandant A: letzte Aenderung vor mehr als 4 Jahren
kanzlei/24/456 Mandant B: Empty directory (not removed): kanzlei/24/456 Mandant B
ablage2/25/789 Mandant C: letzte Aenderung innerhalb der letzten 2 Jahre
```

## Hinweise

- Fehlende Top-Level-Verzeichnisse fuehren nicht zum Abbruch. Stattdessen gibt das Programm eine Meldung wie `Directory 'kanzlei' not found, skipping.` aus.
- Beim Start wird eine SQLite-Datenbank `archivar.db` im aktuellen Arbeitsverzeichnis angelegt oder wiederverwendet.
- Die eigentliche Archivierungslogik ist noch nicht implementiert; der aktuelle Stand analysiert und klassifiziert nur Verzeichnisse.
