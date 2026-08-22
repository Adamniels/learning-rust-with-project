# Progress

Detta är projektets korta operativa återupptagningspunkt. Historisk evidens ligger i `phases/`, `review/`, koden och Git-historiken.

## Nuvarande position

- **Fas:** Fas 2, Rust som designspråk, pågår
- **Konceptenhet:** Enhet 1 avslutad; Enhet 2, traits och standardkontrakt, är nästa
- **Steg i inlärningsloopen:** Enhet 1:s avslutskriterium verifierat; Enhet 2:s mentalmodell återstår
- **Status:** Enhet 1 klar; Enhet 2 inte påbörjad
- **Repetition:** Fas 1 har 3 öppna och 4 förstärkta objekt; Fas 2 har 1 öppet och 2 förstärkta objekt

## Senast slutfört

- Enhet 1:s mentalmodell för package, targets, crates, crate roots, moduler, paths, visibility och facade har genomförts.
- Fyra prediction questions har granskats: tre klara och en delvis korrekt. Tre öppna reviewobjekt följer visibility, invariantgränser och module-scoped imports.
- Package har nu en library target med `src/lib.rs` som crate root och en binary target med `src/main.rs` som separat crate root. Binaryn importerar endast genom `job_server`-facaden.
- Jobbmodellen är samlad i `src/job.rs`; `Job`, `JobKind`, `JobState`, `JobStateKind` och `JobOperation` delar ett sammanhängande moduleansvar. `src/error.rs` definierar endast `JobError`.
- `server` består av privata sibling modules för orchestration och simulation. `server/mod.rs` innehåller endast wiring och re-export av `JobServer`.
- `lib.rs` re-exporterar endast `JobServer`, `Job`, observerande jobbtyper och det nåbara error-kontraktet. Jobbfält, transitions, queue helpers, simulation och retry-policy är inte externt åtkomliga.
- Sex transitionstester ligger i `job.rs`, fjorton orchestrationstester i `server/job_server.rs` och två simulationstester i `server/simulation.rs`.
- Grundläggande rustdoc täcker crate-facaden och samtliga publika items. `cargo rustdoc --lib -- -D missing-docs` och `cargo doc --no-deps` passerar.
- `cargo fmt --check`, `cargo check`, samtliga 22 tester och `cargo run` passerar. Körningen behåller samma resultat: Email-jobbet misslyckas efter tre attempts med total retry delay 6.
- Clippys tidigare `module_inception` är löst. Endast `new_without_default` återstår och behandlas avsiktligt i Unit 2:s standardtraits.
- Den avslutande återkallningen visar en klar target/crate-root-modell och korrekt avsikt att begränsa interna helpers. Facade-reexports beskrevs delvis korrekt men behöver fortsatt precision: en re-export från en privat module skapar en nåbar publik path, inte bara en kortare path.
- Adam ritade module tree och förklarade korrekt att `use` är module-scoped. Den tidigare saknade importen vid uppdelningen av `server` klassificeras därför som en operationell flyttmiss, inte som en kvarvarande felaktig mentalmodell.
- Enhet 1 uppfyller sitt avslutskriterium och är markerad som klar. Det öppna facade-objektet blockerar inte progression och återkommer genom naturlig användning i Unit 2.
- Fas 2 börjar med en library boundary. Execution blir fasens första trait-gräns; registry och queue förblir konkreta tills Fas 3 ger verkliga alternativa implementationer.

## Nästa konkreta handling

Påbörja Enhet 2:s mentalmodell: varför traits finns, inherent methods jämfört med trait methods, `impl Trait for Type`, method resolution, coherence och orphan rule. Knyt modellen till Clippys väntande `Default`-förslag utan att implementera projektkod före prediction questions.

## Aktuell lärdom

`src/lib.rs` och `src/main.rs` är separata crate roots. Filer används för sammanhängande moduleansvar, medan `lib.rs` formar den externa arkitekturgränsen genom avsiktliga re-exports. `use` ändrar inte visibility, och varje typ i en publik signatur måste själv vara nåbar genom facaden.

## Öppna frågor eller blockerare

Inga blockerare. Facade-reachability ligger kvar som öppet reviewobjekt och prövas genom naturlig användning. Clippys `Default`-förslag blir Unit 2:s första konkreta standardtrait-fall.

## Beslut som ska bestå

- Job servern förblir synkron genom Fas 2.
- Adam skriver den substantiva projekt- och övningskoden om han inte uttryckligen delegerar den.
- Exakta prediction questions och labbkrav skapas först när respektive enhet är aktiv.
- Inga nya dependencies, persistence-, HTTP- eller async-abstraktioner införs i Fas 2 utan ett separat beslut.
- Registry och queue får inte traits enbart för arkitektonisk symmetri.
