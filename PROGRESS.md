# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Fas 1, Rust som språk
- **Konceptenhet:** Enhet 1, bindningar, typer, uttryck, funktioner och kontrollflöde
- **Steg i inlärningsloopen:** Mental modell är nästa steg
- **Status:** Enhet 0 är avslutad och verifierad
- **Repetition:** 1 öppet objekt, inget ska repeteras omedelbart

## Senast slutfört

- Durable job server är vald som sammanhängande huvudprojekt.
- Fyra kompetensbaserade faser är valda utan veckor, deadlines eller bestämda passlängder.
- Den återkommande inlärningsloopen är fastställd.
- Projektets dokumentationsstruktur är etablerad.
- Fas 1 är detaljplanerad konceptenhet för konceptenhet och kopplad till inkrement i job servern.
- `job-server/` och `labs/` finns som tomma arbetsytor, och projektet är ett Git-repo.
- Enhet 0:s mentala modell och åtta förutsägelsefrågor är genomförda och granskade.
- Anteckningsytan och repetitionssystemet är etablerade. Ett target-relaterat repetitionsobjekt är öppet för senare återkallning.
- Den lokala installationen är verifierad: stable-toolchain för Apple Silicon, med `rustc` och Cargo 1.97.1.
- `labs/00-toolchain-basics` är skapat som ett binary Cargo-package. Manifest, crate root och förväntad artefakt har inspekterats.
- `cargo check`, `cargo build`, `cargo run` och `cargo test` har körts och deras olika artefakter och beteenden har observerats.
- Ett avsiktligt typfel i mikrolabbet identifierades och reparerades genom att ändra typannoteringen från `i32` till `&str`; `cargo check` passerar igen.
- `job-server` är initierat som ett binary Cargo-package. Byggcykeln passerar och det tillfälliga smoke-testet körs som ett separat testprogram.
- Enhet 0 uppfyller sitt avslutskriterium och är markerad som klar i roadmapen.

## Nästa konkreta handling

Starta enhet 1 med den mentala modellen för bindningar, immutability, `mut`, shadowing, typer, statements, expressions och funktioners returvärden. Följ sedan med förutsägelsefrågor innan kod körs.

## Aktuell lärdom

Cargo arbetar från ett package-manifest, identifierar targets och anropar `rustc` för deras crates. `cargo check`, `build`, `run` och `test` representerar olika delar av byggcykeln och kan producera olika artefakter.

## Öppna frågor eller blockerare

Inga.

## Beslut som ska bestå

- Job servern börjar som en synkron domänkärna utan nätverk.
- Persistence, HTTP och async introduceras först när deras språkliga förkunskaper finns.
- Huvudprojektet bär merparten av lärandet. Isolerade labbar används när det ger bättre förståelse.
- Adam skriver huvuddelen av övnings- och projektkoden. Codex ska inte automatiskt implementera lärmoment åt honom.
- Kodmappar och arkitektur skapas när de behövs, inte som tom spekulation i förväg.

## Innan ett arbetstillfälle avslutas

Uppdatera vid behov:

1. nuvarande position,
2. senast slutfört,
3. nästa konkreta handling,
4. aktuell lärdom,
5. öppna frågor eller blockerare.
