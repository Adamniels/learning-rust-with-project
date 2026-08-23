# Repetitionslogg: Fas 2

## Sammanfattning

| Mått | Antal |
|---|---:|
| Prediction questions besvarade | 15 |
| Klara vid första försöket | 9 |
| Delvis korrekta | 5 |
| Missuppfattningar | 1 |
| Öppna reviewobjekt | 4 |
| Förstärkta reviewobjekt | 3 |
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

## F2-U2-001: Orphan rule kan uppfyllas genom en lokal trait

- **Enhet:** 2
- **Kategori:** Traits, coherence och orphan rule
- **Status:** Öppen
- **Ursprung:** Prediction question 3
- **Nästa tillfälle:** Naturlig trait- eller newtype-design senare i Fas 2
- **Ska repeteras nu:** Nej

### Observerad modell

`Display for Job` identifierades korrekt som tillåten och `Display for Vec<Job>` som förbjuden. Däremot bedömdes även den lokala traiten `LocalLabel` vara förbjuden för `Vec<Job>` eftersom `Vec` är extern. Traitens ägarskap räknades därmed inte som den andra tillåtna vägen genom orphan rule.

### Korrekt modell

En implementation tillåts på denna nivå när antingen traiten eller typen är lokal till aktuell crate. Därför är både `Display for Job` och `LocalLabel for Vec<Job>` tillåtna. `Display for Vec<Job>` är förbjuden eftersom både standardtraiten och den implementerade yttertypen `Vec` är externa; att dess typparameter `Job` är lokal gör inte `Vec<Job>` lokal.

### Framtida återkallningsfrågor

1. Vem äger traiten och yttertypen i en föreslagen implementation, och vilken av dem gör implementationen tillåten?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-22 | Ursprunglig prediction | Delvis korrekt | Lokal typ med extern trait och helt extern kombination klassificerades korrekt; lokal trait för extern typ klassificerades felaktigt |

## F2-U2-002: `Default` komponerar fältens standardkontrakt

- **Enhet:** 2
- **Kategori:** Standardtraits och konstruktion
- **Status:** Förstärkt
- **Ursprung:** Prediction question 4
- **Nästa tillfälle:** Unit 2:s projektinkrement när `JobServer` får ett motiverat `Default`-kontrakt
- **Ska repeteras nu:** Nej

### Observerad modell

Det identifierades korrekt att en härledd default kanske inte motsvarar den avsiktliga `JobServer::new()`. Förklaringen använde däremot C#-modellen "null values" och fastställde inte att programmet kompilerar, att `u64::default()` är `0` eller att collectionernas defaults är tomma värden.

### Korrekt modell

Rust har ingen generell nullinitialisering. `#[derive(Default)]` anropar `Default::default()` för varje fälttyp och kräver att samtliga fält implementerar traiten. Här blir `next_job_id` exakt `0`, medan `HashMap` och `VecDeque` blir giltiga tomma collections. Den härledda servern skiljer sig därför semantiskt från `JobServer::new()`, som börjar ID-sekvensen på `1`.

### Framtida återkallningsfrågor

1. Vilket konkret standardvärde får varje fält, och bevarar den sammansatta defaulten typens konstruktorinvarianter?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-22 | Ursprunglig prediction | Delvis korrekt | Den semantiska risken identifierades, men resultatet och Rusts kompositionsmodell ersattes av en oprecis nullmodell |
| 2026-08-23 | `JobServer`-projektinkrement | Korrekt tillämpning | Adam implementerade `Default` manuellt genom den kanoniska `new()`-konstruktionen; beteendetest verifierar tom server och första jobb-ID `1`, och Clippys `new_without_default` försvann |

## F2-U2-003: `Copy` kräver att varje ägt fält är `Copy`

- **Enhet:** 2
- **Kategori:** Derive, ownership och standardtraits
- **Status:** Öppen
- **Ursprung:** Prediction question 6
- **Nästa tillfälle:** Naturlig derive-granskning i Unit 2:s projektinkrement
- **Ska repeteras nu:** Nej

### Observerad modell

En struct som äger en `String` bedömdes kunna härleda `Copy` eftersom `String` antogs implementera traiten. Kravet att varje fält måste vara `Copy` uttrycktes korrekt, men fälttypens faktiska ownership-semantik klassificerades fel.

### Korrekt modell

`String` äger en heap-allokering och implementerar inte `Copy`. Implicit bitvis kopiering skulle ge två ägare till samma allocation och bryta ägarskapsmodellen. `String` implementerar `Clone`, som utför en explicit djup kopiering av dess innehåll. Därför kan `Job` härleda `Clone` men inte `Copy` så länge den äger en `String`.

### Framtida återkallningsfrågor

1. Vilka resurser äger varje fält, och kan hela värdet dupliceras implicit utan dubbel ownership eller dold kostnad?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-22 | Ursprunglig prediction | Felaktig | `String` antogs vara `Copy`; derive-kravet var känt men tillämpades på felaktig kunskap om fälttypen |

## F2-U2-004: `ToString` ges genom en blanket implementation

- **Enhet:** 2
- **Kategori:** Standardtraits och trait resolution
- **Status:** Öppen
- **Ursprung:** Prediction question 7
- **Nästa tillfälle:** Naturlig användning av `Display` i Unit 2:s projektinkrement
- **Ska repeteras nu:** Nej

### Observerad modell

Programmet bedömdes korrekt kompilera och kopplingen mellan `Display` och `to_string()` identifierades. `to_string()` beskrevs däremot som en metod som `Display` själv innehåller, och den exakta outputen angavs inte.

### Korrekt modell

`Display` deklarerar endast formateringsmetoden `fmt`. Standardbiblioteket har en blanket implementation av `ToString` för varje typ `T` som implementerar `Display`. Metoden `to_string()` kommer alltså från den separata traiten `ToString`, vars implementation blir tillämplig genom `Display`-kontraktet.

### Framtida återkallningsfrågor

1. Vilken trait deklarerar metoden som anropas, och vilken implementation gör metoden tillgänglig för den konkreta typen?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-22 | Ursprunglig prediction | Delvis korrekt | Beteendet förutsades korrekt, men traiten som faktiskt äger metoden sammanblandades med boundet bakom blanket implementationen |
