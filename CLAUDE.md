# claude.md — Vorgaben für Claude Code in Rust-Projekten

## Zielgruppe

* Erfahrener Entwickler mit Backend-Schwerpunkt (Java, Rust, etwas C++, Python, NixOS).
* Wert legt auf **strukturierte, nachvollziehbare Lösungen**, **Clean Code**, sowie **testgetriebene Entwicklung**.
* Interesse an kleinen Arcade-Spielen und Tools, gelegentlich auch Backend-Services oder CLIs.
* Ziel: Rasche, aber nachhaltige Entwicklung mit Fokus auf **Wartbarkeit**, **Lesbarkeit** und **Lernpotenzial**.

---

## Claude Code Vorgaben

### 1. Allgemeiner Stil und Struktur

* **Code stets modular aufbauen.**

    * Verwende sinnvolle Modulstrukturen (`mod`), keine monolithischen Dateien.
    * Jede Funktion/Struktur hat eine klar umrissene Verantwortung.
* **Kommentare nur wo sinnvoll, keine offensichtlichen Dinge kommentieren.**
* Variablen, Funktionen und Structs sollten **aussagekräftige Namen** tragen, im Rust-typischen Stil (`snake_case`).
* **Erklärende Kommentare** nur bei komplexer Logik, Randfällen oder spezifischen Rust-Eigenheiten.

---

### 2. Entwicklung und Testing

* **Testgetrieben entwickeln**:
  Bei jeder neuen Funktionalität zuerst ein Minimalbeispiel als Test (mit `#[cfg(test)]` und `#[test]`), dann Implementierung.
* Nutze **rustdoc-Kommentare** für öffentliche Funktionen und Strukturen (`/// Beschreibung...`).
* Beim Implementieren neuer Features immer auch an Edge Cases und Fehlerbehandlung denken – Rust-typisch mit `Result` und eigenen Error-Typen.

---

### 3. Vorgaben für Claude-Prompts

#### a) Promptaufbau

Jeder Prompt für Claude Code soll so aufgebaut sein:

* **Kontext** (Was ist bereits da? Was ist das Ziel? Ggf. Beispiel für aktuelle API)
* **Klarer Schritt** (Was genau soll realisiert werden?)
* **Achtung auf Tests und Integration** (Wie wird das Ergebnis getestet? Wie wird es in bestehende Module eingebaut?)

#### b) Beispiel-Prompt

```text
Du bist Rust-Experte und unterstützt bei der Entwicklung eines Arcade-2D-Games mit Macroquad.
1. Implementiere eine Komponente, die den aktuellen Tankfüllstand verwaltet und als Statusbalken rendert.
2. Schreibe zuerst Unit-Tests für das Management des Tankfüllstands.
3. Die Lösung soll modular, gut testbar und Macroquad-kompatibel sein.
4. Vermeide nicht-rusttypische Patterns und setze auf idiomatische Lösungen.
5. Füge die Komponente ins bestehende `game`-Modul ein, mit Doku und Beispielaufruf im Test.
```

---

### 4. Umgang mit Fehlern, TODOs & Dokumentation

* Fehler möglichst klar und Rust-idiomatisch behandeln (`Result<T, MyError>`).
* Für noch offene Punkte **TODO-Kommentare** mit kurzer Begründung, ggf. kurzer Hinweis, wie zu lösen.
* Keine ungenutzten Imports, keine toten Codeabschnitte im Haupt-Branch.
* Die Hauptdatei (`main.rs`) bleibt möglichst schlank und ist lediglich der "Wiring"-Ort.

---

### 5. Typische Anforderungen an Claude

* Wenn Unsicherheit bezüglich eines Rust-Patterns besteht, bitte **Explizit einen Hinweis oder Alternativ-Vorschlag** ausgeben.
* Bei Einbindung von Dritt-Bibliotheken wie Macroquad, Serde etc. stets kurze Hinweise zur Kompatibilität und etwaigen Stolpersteinen.
* Falls möglich, Integrationstest oder Demo-`main()` anbieten.

---

## Beispiel-Prompt für Claude Code (als Vorlage für eigene Prompts):

```text
Entwickle in Rust eine Komponente, die ...
- ... den Zustand X verwaltet
- ... per Unit-Test abgedeckt ist
- ... nur Public-API bereitstellt, keine I/O
- ... idiomatisch mit Error-Handling arbeitet
- ... in das bestehende Modul Y integrierbar ist

Füge zu jeder öffentlichen Funktion rustdoc-Dokumentation hinzu. Führe am Ende einen Beispielaufruf im Testmodul vor. Erkläre Abweichungen von Best Practices.
```

---

## Hinweise für Claude Code

* Code sollte **so klein wie möglich, aber so groß wie nötig** ausfallen.
* Lieber mehrere kleine, klar abgegrenzte Prompts als einen zu großen Schritt.
* Verweise auf bestehende Module, nenne ggf. Dateinamen für den Kontext.
* Ergebnisse mit kurzen Beispiel-Outputs/Testfällen.

---

**Kurzfassung für Prompts:**
„Bitte schreibe sauberen, testgetriebenen Rust-Code, modular, dokumentiert, gut integrierbar, und mit Fokus auf Lesbarkeit und Wartbarkeit.“

---

### Spezifische Wünsche (aus deinen bisherigen Gesprächen)

* **Keine magische Abkürzungen**, lieber Klartext und nachvollziehbare Patterns.
* **Fehler immer kenntlich machen** (auch im Prompt: „Wenn du dir unsicher bist, bitte kennzeichnen!“)
* „Immer erklären, wenn Rust-spezifische Eigenheiten oder Best Practices abweichen.“
