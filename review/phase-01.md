# Repetitionslogg: Fas 1

## Sammanfattning

| Mått | Antal |
|---|---:|
| Förutsägelsefrågor besvarade | 36 |
| Klara vid första försöket | 22 |
| Delvis korrekta | 10 |
| Missuppfattningar | 4 |
| Öppna repetitionsobjekt | 6 |
| Förstärkta repetitionsobjekt | 0 |
| Stabila repetitionsobjekt | 0 |
| Återkommande missuppfattningar | 0 |

Klassificeringen avser första försöket och är diagnostisk, inte ett betyg. Relaterade svar kan grupperas i ett enda repetitionsobjekt.

## F1-U0-001: Cargo targets och byggartefakter

- **Enhet:** 0
- **Kategori:** Mental modell
- **Status:** Öppen
- **Ursprung:** Förutsägelsefråga 7–8
- **Nästa tillfälle:** Första naturliga användningen efter ett meningsfullt uppehåll, annars under fasens konsolidering
- **Ska repeteras nu:** Nej

### Observerad modell

Cargo target blandades ihop med katalogen `target/`, och `src/bin/worker.rs` identifierades inte som ett eget binary target.

### Korrekt modell

Ett Cargo target beskriver något Cargo kan bygga. Targetet har en crate root och byggs som en crate. Kompileringen producerar byggartefakter, vilka Cargo normalt lagrar under katalogen `target/`.

`src/main.rs` och `src/bin/worker.rs` upptäcks normalt som två separata binary targets. När flera körbara targets finns kan `cargo run` behöva ett explicit val, exempelvis `cargo run --bin worker`.

### Framtida återkallningsfrågor

1. Förklara skillnaden mellan Cargo target, crate, byggartefakt och katalogen `target/`.
2. Vilka targets upptäcker Cargo från `src/main.rs` och `src/bin/worker.rs`, och hur väljs ett av dem vid körning?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-12 | Ursprunglig förutsägelse | Missuppfattning | Svar på fråga 7–8 |
| 2026-08-12 | Omedelbar tillämpning i mikrolabb | Delvis, target kunde inte identifieras säkert | Klassificering av `src/main.rs` efter `cargo new --bin` |

## F1-U1-001: Blockvärden, semikolon och funktionsretur

- **Enhet:** 1
- **Kategori:** Mental modell
- **Status:** Öppen
- **Ursprung:** Förutsägelsefråga 5–6
- **Nästa tillfälle:** Naturlig tillämpning i enhet 1:s mikrolabb och projektinkrement
- **Ska repeteras nu:** Nej

### Observerad modell

Ett semikolon efter blockets sista expression antogs göra blocket ogiltigt. En funktion vars avslutande expression hade semikolon bedömdes korrekt som icke-kompilerande, men orsaken angavs vara att ett explicit `return` saknades.

### Korrekt modell

Ett block med en avslutande expression utan semikolon producerar expressionens värde. Med semikolon kastas värdet bort och blocket producerar `()`. Det är fortfarande ett giltigt block om sammanhanget accepterar `()`.

Rust kräver inte ett explicit `return` för funktionens slutvärde. En funktion som lovar `i32` kan avslutas med en `i32`-expression utan semikolon. Om semikolon läggs till producerar kroppen `()`, vilket ger typkonflikten `expected i32, found ()`.

### Framtida återkallningsfrågor

1. Vad producerar blocken `{ 4 * 2 }` respektive `{ 4 * 2; }`?
2. Varför kompilerar `fn double(x: i32) -> i32 { x * 2 }` utan `return`?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-13 | Ursprunglig förutsägelse | Missuppfattning och precisionslucka | Svar på fråga 5–6 |
| 2026-08-13 | Projektinkrement | Korrekt omedelbar tillämpning, status lämnas öppen till senare återkallning | `simulate_job` returnerar failure-tuplen som funktionens avslutande expression |

## F1-U1-002: Tuple- och arraytyper

- **Enhet:** 1
- **Kategori:** Typnotation
- **Status:** Öppen
- **Ursprung:** Förutsägelsefråga 10
- **Nästa tillfälle:** Naturlig användning av tuples eller arrays, annars under enhet 1:s avslutning
- **Ska repeteras nu:** Nej

### Observerad modell

Tupletypen identifierades nästan korrekt men Rust-typen `bool` kallades `boolean`. Arraytypen uttrycktes med tuplelik notation `(u32, 3)`.

### Korrekt modell

Tupletypen i exemplet är `(u64, bool, u32)`. En arraytyp skrivs `[T; N]`, där `T` är elementtypen och `N` den fasta längden. Typen för tre `u32`-värden är därför `[u32; 3]`.

### Framtida återkallningsfrågor

1. Vilken typ har `(7_u64, true, 2_u32)`?
2. Hur skiljer sig typnotationerna för `(u32, u32, u32)` och `[u32; 3]`, och vad uttrycker de?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-13 | Ursprunglig förutsägelse | Delvis korrekt | Svar på fråga 10 |
| 2026-08-13 | Projektinkrement | Korrekt omedelbar tillämpning, status lämnas öppen till senare återkallning | Returtypen `(u32, bool, u32)` och arrayen av tuplebaserade testfall används korrekt |

## F1-U2-001: Struct update flyttar icke-`Copy`-fält

- **Enhet:** 2
- **Kategori:** Ownership
- **Status:** Öppen
- **Ursprung:** Förutsägelsefråga 6
- **Nästa tillfälle:** Naturlig användning av struct update syntax, annars under fasens konsolidering
- **Ska repeteras nu:** Nej

### Observerad modell

Efter `Job { max_attempts: 5, ..original }` antogs både `original.max_attempts` och `original.payload` fortfarande vara användbara.

### Korrekt modell

Struct update syntax klonar inte hela ursprungsvärdet. Fält som inte anges explicit hämtas individuellt från ursprungsstructen: `Copy`-fält kopieras och icke-`Copy`-fält flyttas. I exemplet sätts `max_attempts` explicit i det nya värdet, så `original.max_attempts` förblir orört och användbart. `payload` hämtas däremot från `original` och flyttas, vilket gör `original.payload` ogiltigt.

### Framtida återkallningsfrågor

1. Efter en struct update, vilka fält är fortfarande användbara från ursprungsstructen och varför?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-14 | Ursprunglig förutsägelse | Delvis korrekt | Första utskriften bedömdes implicit rätt, men det flyttade `String`-fältet bedömdes fortfarande som användbart |

## F1-U3-001: Ett lån äger inte den lånade datan

- **Enhet:** 3
- **Kategori:** Borrowing
- **Status:** Öppen
- **Ursprung:** Förutsägelsefråga 4
- **Nästa tillfälle:** Naturlig tillämpning av `&self` och `&mut self` i enhet 3:s projektinkrement
- **Ska repeteras nu:** Nej

### Observerad modell

`view` från en metod med returtypen `&str` beskrevs som att den tog ownership av `job.payload`. Konflikten med en senare `&mut self`-metod identifierades korrekt, men förklarades som en ownershipöverföring genom en pekare.

### Korrekt modell

Bindningen `view` innehåller ett referensvärde men äger inte strängens bytes. Den håller ett delat lån av data som ägs av `job.payload`. Eftersom `view` används efter anropet till `replace_payload`, är det delade lånet fortfarande aktivt när metoden begär ett exklusivt `&mut self`-lån av hela jobbet. Lånen kan inte samexistera. Att ersätta payloaden skulle dessutom droppa den gamla `String` vars buffer `view` visar.

### Framtida återkallningsfrågor

1. Vem äger texten när en metod returnerar `&str`, och varför kan en senare `&mut self`-operation behöva vänta tills vyns sista användning?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-15 | Ursprunglig förutsägelse | Delvis korrekt | Compile-resultatet identifierades, men lånet beskrevs som en ownershipöverföring |
| 2026-08-15 | Projektinkrement | Korrekt omedelbar tillämpning, status lämnas öppen till senare återkallning | `payload(&self) -> &str`, `record_attempt(&mut self)` och `simulate_job(&mut Job, ...)` skiljer observerande och muterande lån utan ownershipöverföring |

## F1-U5-001: `iter_mut` lånar collectionen exklusivt

- **Enhet:** 5
- **Kategori:** Borrowing och iteration
- **Status:** Öppen
- **Ursprung:** Förutsägelsefråga 3
- **Nästa tillfälle:** Naturlig tillämpning under enhet 5:s projektinkrement; det separata mikrolabbet hoppades över efter svårighetskalibrering
- **Ska repeteras nu:** Nej

### Observerad modell

Mutation av ett element genom `&mut T` och en samtidig strukturell mutation genom `push_back` bedömdes kunna ske tillsammans inuti en `iter_mut`-loop.

### Korrekt modell

`iter_mut()` tar ett exklusivt `&mut`-lån av collectionen och iteratorn behåller lånet medan den används. Iteratorn kan säkert ge exklusiva lån till element, men collectionen kan inte samtidigt lånas mutabelt igen för en strukturell operation som `push_back`. En sådan operation kan dessutom ändra collectionens lagring och skulle kunna göra utestående elementreferenser ogiltiga. Den strukturella mutationen måste ske efter iteratorns sista användning.

### Framtida återkallningsfrågor

1. Varför får ett element ändras genom värdet från `iter_mut`, medan `push_back` på samma collection nekas inne i loopen?

### Historik

| Datum | Sammanhang | Resultat | Evidens |
|---|---|---|---|
| 2026-08-17 | Ursprunglig förutsägelse | Missuppfattning | Samtidig elementmutation och strukturell mutation bedömdes kompilera |
| 2026-08-17 | Omedelbar relaterad tillämpning i projektinkrement | Korrekt avgränsning av ett mutabelt lån från `next_queued`; exakt `iter_mut`-konflikt återtestades inte, status lämnas öppen | Ett explicit block avslutar `&mut Job` innan `JobServer::get` och nästa `next_queued` |
