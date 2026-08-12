# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Fas 1, Rust som språk
- **Konceptenhet:** Enhet 0, verktygskedjan och kompileringsmodellen
- **Steg i inlärningsloopen:** Lokal installationskontroll före mikrolabb
- **Status:** Mental modell och förutsägelse genomförda
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

## Nästa konkreta handling

Kontrollera den lokala Rust-installationen genom att identifiera aktiv toolchain och versionerna av `rustup`, `rustc` och Cargo. Tolka resultaten innan `labs/` eller `job-server/` initieras som Cargo-packages.

## Aktuell lärdom

Cargo target betyder något Cargo kan bygga, inte katalogen `target/`. Ett target byggs från en crate root som en crate och producerar byggartefakter, vilka normalt lagras under `target/`.

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
