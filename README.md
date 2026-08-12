# Learning Rust

Detta projekt är en sammanhängande lärmiljö för att lära sig Rust på djupet genom att bygga en durable job server. Målet är inte att färdigställa systemet så snabbt som möjligt, utan att förstå Rusts modeller för ägarskap, fel, abstraktion och samtidighet genom att använda dem i ett verkligt sammanhang.

## Grundprinciper

- Planen är kompetensbaserad, inte tidsbaserad. Det finns inga veckomål eller bestämda passlängder.
- Arbetet ska kunna pausas efter varje avslutat delsteg och återupptas direkt.
- Job servern är huvudprojektet. Små mikrolabbar används när ett koncept behöver isoleras.
- Språket lärs före ramverken. Domänkärnan byggs synkront innan persistence, HTTP och async tillkommer.
- Adam skriver huvuddelen av koden. Codex fungerar främst som lärare, diskussionspartner, reviewer och debugger.
- Att förstå varför kod kompilerar eller inte kompilerar är en del av resultatet.

## Inlärningsloopen

Varje konceptenhet följer samma loop. En enhet kan sträcka sig över valfritt antal arbetstillfällen.

1. **Mental modell:** Förstå varför konceptet finns och vilket problem Rust löser.
2. **Förutsägelse:** Läs små kodexempel och förutsäg om de kompilerar och varför.
3. **Mikrolabb:** Lös ett isolerat problem där konceptet inte kan döljas bakom andra abstraktioner.
4. **Projektinkrement:** Använd konceptet till en funktion i job servern.
5. **Compiler-driven debugging:** Dokumentera kompilatorns invändning, den brutna regeln och varför lösningen fungerar.
6. **Återkallning:** Förklara konceptet och återskapa en liten variant utan anteckningar efter ett meningsfullt uppehåll.
7. **Refaktorering och test:** Granska om lösningen bara kompilerar eller också uttrycker ägarskap, fel och ansvar idiomatiskt.

Stegen är en lärordning, inte en sessionsmall. Ett arbetstillfälle får sluta efter vilket avslutat steg eller delsteg som helst. Återkallning används efter ett tillräckligt långt uppehåll för att vara meningsfull, inte mekaniskt varje gång projektet öppnas.

## Så återupptas arbetet

1. Öppna [PROGRESS.md](PROGRESS.md).
2. Läs endast aktuell enhet, senaste lärdom och nästa handling.
3. Gör nästa handling eller uppdatera den om förutsättningarna har förändrats.
4. Innan arbetet avslutas, lämna en konkret nästa handling som inte kräver att sammanhanget rekonstrueras.

[ROADMAP.md](ROADMAP.md) beskriver progressionen och kriterierna för att gå vidare. `PROGRESS.md` är däremot den enda operativa källan till var arbetet befinner sig just nu.

## Planerad kodstruktur

Kodmapparna skapas först när de behövs:

```text
learning-rust/
├── README.md
├── ROADMAP.md
├── PROGRESS.md
├── job-server/    # Det sammanhängande huvudprojektet
└── labs/          # Små, isolerade konceptövningar
```

Vi undviker att skapa en detaljerad mapphierarki i förväg. Strukturen får växa när Rusts modulmodell och projektets faktiska behov motiverar den.
