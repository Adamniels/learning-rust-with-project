# Progress

Detta är projektets operativa återupptagningspunkt. Filen ska vara kort och uppdateras när arbetets faktiska tillstånd förändras.

## Nuvarande position

- **Fas:** Förberedelse inför fas 1, Rust som språk
- **Konceptenhet:** Verktygskedjan och den grundläggande byggcykeln
- **Steg i inlärningsloopen:** Inte påbörjat
- **Status:** Studiesystemet etableras

## Senast slutfört

- Durable job server är vald som sammanhängande huvudprojekt.
- Fyra kompetensbaserade faser är valda utan veckor, deadlines eller bestämda passlängder.
- Den återkommande inlärningsloopen är fastställd.
- Projektets dokumentationsstruktur är etablerad.

## Nästa konkreta handling

Starta den första konceptenheten med en mental modell av Rusts verktygskedja: relationen mellan `rustc`, Cargo, packages, crates, targets och kompileringscykeln. Kontrollera därefter den lokala installationen innan någon kod eller projektstruktur skapas.

## Aktuell lärdom

Planeringen är tillståndsbaserad. En konceptenhet behöver inte motsvara ett arbetstillfälle, och ett arbetstillfälle behöver inte ha en bestämd längd.

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
