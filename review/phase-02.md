# Repetitionslogg: Fas 2

## Sammanfattning

| Mått | Antal |
|---|---:|
| Prediction questions besvarade | 4 |
| Klara vid första försöket | 3 |
| Delvis korrekta | 1 |
| Missuppfattningar | 0 |
| Öppna reviewobjekt | 1 |
| Förstärkta reviewobjekt | 2 |
| Stabila reviewobjekt | 0 |
| Återkommande missuppfattningar | 1 |

Klassificeringen är diagnostisk, inte ett betyg. Fas 1:s kvarvarande objekt ligger kvar i `phase-01.md` och återkallas genom naturlig användning under Fas 2.

## F2-U1-001: Publika signaturer kräver nåbara typer

- **Enhet:** 1
- **Kategori:** Visibility och API-design
- **Status:** Öppen
- **Ursprung:** Prediction question 3
- **Nästa tillfälle:** Naturlig användning av publika signaturer i Unit 2
- **Ska repeteras nu:** Nej

### Observerad modell

En `pub` funktion som returnerar en `pub(crate)` typ bedömdes korrekt kunna kompileras, men API:t bedömdes samtidigt vara problemfritt. `use` och den lokala namnåtkomsten skildes därmed inte fullt ut från typens externa visibility.

### Korrekt modell

`use` binder endast ett namn i aktuell scope och ändrar inte itemets visibility. Libraryn kan kompilera, men rustc varnar genom linten `private_interfaces` när en publikt nåbar funktion exponerar en mindre synlig returtyp. Signaturen utgör därför inte ett användbart externt kontrakt. Typen måste göras nåbar genom den publika facaden, eller funktionens visibility måste begränsas till samma nivå som typen.

### Framtida återkallningsfrågor

1. För varje publik facade-metod, är samtliga parameter- och returtyper nåbara för en extern crate, och vilken path använder anroparen?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-20 | Ursprunglig prediction | Delvis korrekt | Compile-resultatet identifierades, men `private_interfaces`-varningen och den oanvändbara API-signaturen missades |
| 2026-08-20 | Första modulextraktionen | Återkommande theory-to-application-gap | `JobKind` gjordes `pub(crate)` samtidigt som både `job`-modulen och `lib.rs` försökte re-exportera typen offentligt; rustc gav E0365 |
| 2026-08-20 | Omedelbar korrigering i `job`-facaden | Korrekt, status lämnas öppen till senare återkallning | `Job`, `JobKind` och `JobState` är publikt nåbara genom facaden, fälten är privata och interna transitions är begränsade till library crate |
| 2026-08-21 | Slutlig facade-audit | Strukturen är korrekt; återkallning återstår | Samtliga typer i publika signaturer re-exporteras från crate root och strikt rustdoc passerar, men slutrefaktoreringen delegerades till Codex och räknas därför inte ensam som lärande-evidens |
| 2026-08-22 | Avslutande återkallning | Delvis korrekt, status förblir öppen | Typerna identifierades behöva vara nåbara från andra crates, men re-export beskrevs främst som ett sätt att slippa en full path; eftersom modulerna är privata skapar re-exporten den nåbara externa pathen |

## F2-U1-002: Publik visibility måste motsvara ett externt kontrakt

- **Enhet:** 1
- **Kategori:** Visibility och invariantgränser
- **Status:** Förstärkt
- **Ursprung:** Första servermodulextraktionen
- **Nästa tillfälle:** Naturlig visibility-granskning i ett senare projektinkrement
- **Ska repeteras nu:** Nej

### Observerad modell

`next_queued`, `simulate_job` och `retry_delay_seconds` gjordes publika när serverimplementationen flyttades. De behövs endast inom `server`-modulen. Särskilt kombinationen av `next_queued` och `simulate_job` låter en extern anropare processa ett mutabelt jobb utan att `JobServer::process_next` får fullfölja köuppdateringen.

### Korrekt modell

`pub` uttrycker ett avsiktligt externt kontrakt, inte bara att ett item behöver anropas någonstans. Items som endast samarbetar inom samma module förblir privata. Libraryns facade ska exponera use cases som bevarar hela invarianten, här `process_next`, inte deloperationer som kan kombineras till ett inkonsistent state.

### Framtida återkallningsfrågor

1. Kan någon publik kombination av servermetoder ändra ett jobb utan motsvarande köuppdatering, och vilka helpers behöver verkligen vara nåbara utanför `server`?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-20 | Första servermodulextraktionen | Theory-to-application-gap | Tre interna helpers gjordes publika; två av dem skapar tillsammans en extern väg runt `process_next`-invarianten |
| 2026-08-20 | Omedelbar facade-korrigering | Korrekt, status lämnas öppen till senare återkallning | `next_queued`, `simulate_job` och `retry_delay_seconds` är privata; endast fullständiga server-use-cases exponeras |
| 2026-08-21 | Slutlig visibility-audit | Strukturen är korrekt; återkallning återstår | Endast kompletta use cases och nödvändiga observationer är publika; implementationen utfördes delvis av Codex och behöver därför försvaras av Adam innan status ändras |
| 2026-08-22 | Avslutande återkallning | Korrekt avsikt med mindre scope-precision | Adam motiverade att `simulate_job` endast ska vara nåbar för serverns interna samarbete och inte för externa crates; `pub(super)` betyder exakt visibility inom parent-modulen `server`, inte särskild visibility enbart för `job_server` |

## F2-U1-003: Imports tillhör en module, inte en katalog

- **Enhet:** 1
- **Kategori:** Modulträd och paths
- **Status:** Förstärkt
- **Ursprung:** Uppdelningen av `server` i sibling modules
- **Nästa tillfälle:** Naturlig modulextraktion senare i fasen
- **Ska repeteras nu:** Nej

### Observerad modell

`simulate_job` och retry-policyn flyttades korrekt till `server::simulation`, men deras `Job`, `JobError` och `JobState`-imports lämnades i sibling-modulen `server::job_server`. De två simulationstesterna låg också kvar under `job_server` och försökte importera `simulate_job` genom sin parent trots att funktionen finns i en sibling module.

### Korrekt modell

Varje module har en egen namespace och scope. `use` binder namn endast i modulen där deklarationen står och ärvs inte av sibling modules bara för att deras filer ligger i samma katalog. `simulation.rs` måste importera sina egna crate-beroenden, och tester av privata simulation-items ska vara descendants till `simulation` för att få rätt visibility.

### Framtida återkallningsfrågor

1. Vilken module motsvarar varje fil under `server/`, och vilka namn måste importeras separat i respektive scope?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-20 | Första uppdelningen av `server` | Theory-to-application-gap | Sibling-modulen `simulation` saknade egna imports och dess privata funktionstester låg kvar under `job_server` |
| 2026-08-20 | Omedelbar korrigering av `server::simulation` | Korrekt, status lämnas öppen till senare återkallning | `simulation` importerar sina egna beroenden, `simulate_job` är `pub(super)` och dess två tester ligger som descendants till rätt module |
| 2026-08-21 | Slutlig module-audit | Strukturen är korrekt; återkallning återstår | `server::job_server` och `server::simulation` har separata imports och rätt test-descendants; Adam behöver förklara scopes och sibling paths utan stöd innan status ändras |
| 2026-08-22 | Avslutande återkallning | Korrekt; tidigare händelse omklassificerad som operationell flyttmiss | Adam förklarade direkt att en import i en sibling-fil inte importerar namnet i den andra. Den underliggande Rust-regeln uttrycks som att varje module har eget scope, även om flera modules skulle skrivas i samma fysiska fil |
