# Fas 1: Rust som språk

## Syfte

Fasen bygger en synkron job server-kärna samtidigt som Rusts modeller för data, ägarskap och fel introduceras i beroendeordning. Projektet växer efter varje konceptenhet. Vi väntar alltså inte med all domänkod till slutet av fasen.

Fasplanen anger vad som ska läras och vilket beteende som ska tillföras. Exakta förutsägelsefrågor, labbuppgifter och lösningar skapas först när respektive enhet börjar.

## Resultat efter fasen

Den synkrona kärnan ska kunna:

```text
submit(payload, kind) -> job_id
get(job_id) -> job
process_next() -> outcome
```

Ett register äger jobben. En FIFO-kö innehåller kopierbara jobb-ID:n och bestämmer behandlingsordningen. Jobbens tillstånd och fel representeras explicit.

Fasen innehåller inte nätverk, async, parallella workers, databas eller produktionsfärdig CLI. Persistence och concurrency kommer senare, så systemet är ännu inte durable efter denna fas.

## Arbetsmodell för varje enhet

Varje enhet går genom följande stoppbara steg:

1. Mental modell och avgränsad teori
2. Förutsägelsefrågor utan att koden först körs
3. Ett isolerat mikrolabb
4. Genomgång av lösningen och relevanta kompilatorfel
5. Ett projektinkrement beskrivet som krav, inte färdig kod
6. Test och idiomatisk refaktorering
7. Uppdatering av `PROGRESS.md`

Enheten är tillräckligt avslutad för att gå vidare när Adam kan förklara kärnmodellen, motivera de viktiga kodvalen och implementera inkrementet med begränsat stöd. Kvarvarande osäkerheter dokumenteras och återkallas senare. Fullständig perfektion krävs inte innan nästa enhet.

Grundläggande tester introduceras så snart den första rena funktionen finns och växer med projektet. Fas 2 behandlar djupare testdesign, integrationstester och arkitektoniska testgränser.

## Enhet 0: Verktygskedjan och kompileringsmodellen

### Koncept

- Roller för `rustup`, `rustc` och Cargo
- Package, crate, binary target och library target
- Källkod, kompilering och byggartefakter
- Skillnaden mellan `cargo check`, `cargo build`, `cargo run` och `cargo test`
- Grundläggande läsning av kompilatorfel

### Förutsägelse och labbfokus

Förutsäg vilka filer och artefakter Cargo använder eller skapar, och vilken del av kedjan som upptäcker olika typer av fel. Skapa därefter ett minimalt labb, introducera ett avsiktligt kompileringsfel och reparera det genom att läsa diagnostiken.

### Projektinkrement

Initiera `job-server` som ett minimalt Cargo-package. Verifiera byggcykeln och lägg till ett första trivialt test, utan att ännu designa domänarkitektur.

### Avslutskriterium

Kunna förklara relationen mellan package, crate och target samt välja rätt Cargo-kommando för snabb kontroll, körning respektive testning.

## Enhet 1: Bindningar, typer, uttryck, funktioner och kontrollflöde

### Koncept

- Immutability som standard, `mut` och shadowing
- Skalära typer, tuples och arrays
- Statements jämfört med expressions
- Funktioner, parametrar och returvärden
- `if`, `match` på enkla värden och loop-konstruktionerna
- Grundläggande typinferens och explicita typer vid API-gränser

### Förutsägelse och labbfokus

Förutsäg typer och returvärden, särskilt hur semikolon förändrar expressions, varför grenar måste ge kompatibla typer och hur shadowing skiljer sig från mutation. Labbet använder rena funktioner som är enkla att testa.

### Projektinkrement

Representera ett enda hårdkodat jobbförsök med enkla värden. Extrahera testbara funktioner som avgör om ett jobb får köras utifrån aktuellt och maximalt antal försök. Representationen är avsiktlig byggställning och ersätts i senare enheter.

### Avslutskriterium

Kunna läsa och skriva små Rust-funktioner, förutsäga deras typer och förklara när ett block producerar ett värde.

## Enhet 2: Ownership, ägd data och structs

### Koncept

- Ownershipreglerna och deterministisk destruktion
- Moves, `Copy`, explicit `Clone` och scopes
- Skillnaden mellan värdet `String` och en strängslice
- Structs som ägare av sammanhörande data
- Field access, struct update syntax och destructuring på grundnivå

### Förutsägelse och labbfokus

Följ ägarskapet för heap-allokerad data genom bindningar, structs och funktionsanrop. Förutsäg use-after-move-fel och avgör när kopiering, kloning eller ägarskapsöverföring uttrycker rätt avsikt.

### Projektinkrement

Ersätt de lösa värdena med en `Job` som äger sin payload. Flytta ett jobb mellan skapande och en enkel exekveringsfunktion så att ägarskapsöverföringen blir synlig.

### Avslutskriterium

Kunna peka ut ägaren till varje centralt värde, förutsäga när ett move sker och undvika `clone()` som reflexmässig lösning på ownershipproblem.

## Enhet 3: Borrowing, slices och metoder

### Koncept

- Delade och exklusiva lån
- `&T`, `&mut T` och Rusts aliasingregler
- Slices som lånade vyer över sammanhängande data
- Reborrowing på grundnivå
- Associerade funktioner samt metoder med `self`, `&self` och `&mut self`

### Förutsägelse och labbfokus

Förutsäg låns giltighet och konflikter mellan delad läsning och mutation. Labbet separerar operationer som observerar, förändrar och konsumerar data.

### Projektinkrement

Inför `Job::new` och metoder som inspekterar eller förändrar jobbet utan onödiga ägarskapsöverföringar. Gör skillnaden mellan observerande, muterande och konsumerande operationer explicit i signaturerna.

### Avslutskriterium

Kunna välja mellan ägt värde, delad referens och mutabel referens utifrån operationens semantik samt förklara varför motstridiga lån nekas.

## Enhet 4: Enums, pattern matching, `Option` och tillståndsmodellering

### Koncept

- Enums med och utan associerad data
- Exhaustive pattern matching
- `if let` och när fullständig `match` är tydligare
- `Option<T>` som explicit frånvaro
- Tillståndsmaskiner och att göra ogiltiga representationer svårare

### Förutsägelse och labbfokus

Identifiera saknade match-armar, bindningar i patterns och skillnaden mellan frånvaro och ett godtyckligt sentinelvärde. Modellera en liten tillståndsmaskin utan kombinationer av boolska flaggor.

### Projektinkrement

Inför `JobKind` och `JobState`. Representera åtminstone köat, pågående, lyckat och misslyckat arbete, inklusive output eller felorsak där det hör hemma. Tillståndsoperationer kan tillfälligt använda `Option` tills rikare fel introduceras.

### Avslutskriterium

Kunna modellera domäntillstånd med enums, hantera alla varianter explicit och förklara vilka tidigare ogiltiga representationer som inte längre går att skapa.

## Enhet 5: Collections och iteration

### Koncept

- Ägarskap av element i `Vec`, `VecDeque` och `HashMap`
- Indexering jämfört med säkra uppslag
- Iteration genom ägande, delade lån och mutabla lån
- Iterator-adaptrar på grundnivå
- Val av samling utifrån åtkomstmönster och invarianter

### Förutsägelse och labbfokus

Förutsäg elementens och samlingens ägarskap för `iter`, `iter_mut` och konsumerande iteration. Jämför samlingar genom operationerna de ska stödja, inte genom vana från andra språk.

### Projektinkrement

Skapa en synkron kärna där ett `HashMap` äger jobben och en `VecDeque` innehåller jobb-ID:n i FIFO-ordning. Lägg till operationer för att skicka in, hämta och välja nästa jobb.

### Avslutskriterium

Kunna motivera registrets och köns separata ansvar samt iterera utan onödiga kloner eller ägarskapsförluster.

## Enhet 6: `Result`, felmodellering och felpropagering

### Koncept

- `Result<T, E>` och skillnaden mot `Option<T>`
- `?` som tidig retur och felkonvertering
- Egna feltyper med enums
- Återställbara fel jämfört med brutna invarianter och `panic!`
- Fel vid domän-, API- och systemgränser

### Förutsägelse och labbfokus

Klassificera fel och förutsäg kontrollflödet genom kedjor av `Result`. Ersätt informationsfattig frånvaro eller boolska resultat med fel som hjälper anroparen fatta beslut.

### Projektinkrement

Inför en domänspecifik feltyp. Gör saknade jobb, tom kö och ogiltiga tillståndsövergångar explicita där de faktiskt är fel, och propagera dem utan panik eller informationsförlust.

### Avslutskriterium

Kunna välja mellan `Option`, `Result` och panik utifrån kontraktet samt utforma fel som bevarar relevant domäninformation.

## Enhet 7: Konsolidering av den synkrona kärnan

### Koncept

- Repetition av fasens mentala modeller
- Invarianter och ansvar i ett litet API
- Grundläggande testfall för normalt flöde, gränsfall och fel
- Refaktorering som förbättrar semantik utan förtida abstraktion

### Förutsägelse och labbfokus

Återskapa centrala ownership-, borrowing- och tillståndsexempel utan anteckningar. Lös därefter ett mindre sammanhängande problem där rätt representation och signaturer måste väljas utan styrning per kodrad.

### Projektinkrement

Färdigställ och granska fasens synkrona job server-kärna. Demonstrera att jobb får unika ID:n, behandlas i FIFO-ordning, följer giltiga tillståndsövergångar och returnerar explicita fel utan att systemet lämnas i ett inkonsistent tillstånd.

### Avslutskriterium

Kunna implementera ett mindre tillägg självständigt, förklara vem som äger varje central datastruktur och försvara API:ets val av ägande, lån, optionalitet och fel.

## Fasens kvalitetsgrind

Innan fas 2 detaljplaneras ska:

- all kod formateras och passera kompilatorns kontroller,
- alla tester passera,
- inga onödiga `clone()` eller paniker dölja oklar ägarskap eller felmodellering,
- domäntillstånden sakna kända ogiltiga representationer som rimligen kan förhindras med fasens koncept,
- Adam kunna förklara lösningen utan att bara hänvisa till att kompilatorn accepterar den,
- kvarvarande kunskapsluckor vara noterade i `PROGRESS.md`.
