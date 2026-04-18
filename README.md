# archivar

`archivar` ist ein Rust-CLI zum Durchlaufen einer festen Ablagestruktur und zum Erfassen alter Vorgangsordner in einer SQLite-Datenbank.

Das Programm arbeitet relativ zum aktuellen Arbeitsverzeichnis (oder einem per `-p` angegebenen Pfad). Es sucht nach bekannten Wurzelverzeichnissen, durchlaeuft Jahresordner, erkennt leere Mandantenordner und berechnet fuer aeltere Eintraege einen SHA-256-Hash eines erzeugten `tar.xz`-Archivs.

## Aktuelles Verhalten

- `kanzlei` wird direkt als Top-Level-Verzeichnis verarbeitet.
- `ablage` wird ebenfalls gesucht. Darunter verarbeitet der aktuelle Code direkt die Unterverzeichnisse `ablage/2`, `ablage/4`, `ablage/6` und `ablage/8`.
- In `kanzlei` und in diesen `ablage`-Buckets werden nur direkte Unterordner mit genau zwei Ziffern als Jahresordner akzeptiert, zum Beispiel `24` oder `25`.
- Innerhalb eines Jahresordners werden nur direkte Unterordner verarbeitet, deren Name mit drei Ziffern beginnt, zum Beispiel `123 Mandant A`.
- Fuer jeden passenden Mandantenordner wird die letzte inhaltliche Aenderung ueber den gesamten Unterbaum bestimmt.
- Leere Mandantenordner werden je nach Optionen nur gemeldet oder geloescht.
- Ordner, die aelter als 2 Jahre sind, werden archiviert: Es wird ein `tar.xz`-Archiv in `$TMPDIR` erzeugt, daraus ein SHA-256-Hash berechnet, das Archiv in das passende `ablage/N`-Verzeichnis verschoben und der Datensatz in SQLite gespeichert.
- Ordner innerhalb der letzten 2 Jahre werden ebenfalls in SQLite eingetragen, aber mit `NULL` in der Hash-Spalte.

## Verzeichnisstruktur

Beispiel fuer `kanzlei`:

```text
.
└── kanzlei
    ├── 24
    │   ├── 123 Mandant A
    │   └── 456 Mandant B
    └── 25
        └── 789 Mandant C
```

Beispiel fuer `ablage`:

```text
.
└── ablage
    ├── 2
    │   └── 24
    │       └── 123 Mandant A
    ├── 4
    │   └── 23
    │       └── 456 Mandant B
    ├── 6
    │   └── 22
    └── 8
        └── 21
```

## Altersklassen

Die Altersbewertung basiert auf der letzten Aenderung innerhalb des Verzeichnisinhalts, nicht auf der Aenderung des Mandantenordners selbst.

Es gibt diese Buckets:

- innerhalb der letzten 2 Jahre
- mehr als 2 Jahre → `ablage/2`
- mehr als 4 Jahre → `ablage/4`
- mehr als 6 Jahre → `ablage/6`
- mehr als 8 Jahre → `ablage/8`

Fuer die Stufen `mehr als 6 Jahre` und `mehr als 8 Jahre` gilt eine Sonderregel: Statt des exakten Aenderungsdatums wird der naechste `1. Januar` nach der letzten Aenderung als Stichtag verwendet.

## Datenbank

Im Arbeitsverzeichnis (oder dem per `-p` angegebenen Pfad) wird `archivar.db` angelegt oder wiederverwendet.

Die Tabelle `archive` hat aktuell diese Struktur:

```sql
CREATE TABLE IF NOT EXISTS archive (
    year INTEGER NOT NULL,
    no INTEGER NOT NULL,
    name TEXT NOT NULL,
    change_time REAL NOT NULL,
    hash BLOB UNIQUE CHECK (length(hash) = 32),
    PRIMARY KEY (year, no)
)
```

Beim Schreiben wird `INSERT ... ON CONFLICT(year, no) DO UPDATE` verwendet.

## CLI

```bash
cargo run
cargo run -- --dry-run
cargo run -- --remove
cargo run -- --dry-run --remove
cargo run -- --path /pfad/zum/archiv
```

Optionen:

- `-d`, `--dry-run`: nur ausgeben, was passieren wuerde
- `-r`, `--remove`: leere Mandantenordner loeschen
- `-p`, `--path`: Arbeitsverzeichnis angeben (Standard: aktuelles Verzeichnis)

## Entwicklung

```bash
cargo build
cargo test
cargo fmt
cargo clippy --all-targets --all-features
```

## Hinweise

- Fehlende Wurzelverzeichnisse werden nur gemeldet und uebersprungen.
- Fehlende Bucket-Unterverzeichnisse in `ablage` werden gemeldet und uebersprungen.
- Archive werden zunaechst in `$TMPDIR` erstellt, dann nach `ablage/N` verschoben; das Zielverzeichnis wird bei Bedarf angelegt.
- Die Programmausgabe ist aktuell eher debug-lastig und enthaelt mehrere `println!`-Meldungen waehrend des Durchlaufs.
