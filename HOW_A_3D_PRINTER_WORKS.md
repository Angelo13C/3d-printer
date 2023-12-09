# Come funziona una stampante 3D?

## Tecnologie esistenti
Ci son varie tecnologie di stampanti 3D, tra cui quelle a resina (i cui pezzi stampati però hanno bisogno di trattamenti chimici), quelle che usano i laser (tra cui le stampanti SLA, 
che permettono di stampare anche i metalli, ma hanno prezzi elevati) e poi quelle a filamento fuso, che son le più diffuse.

Questa pagina è una spiegazione (molto semplificata) di come funzionano quest'ultime.

![Render](https://github.com/Angelo13C/3d-printer/assets/55251189/13c3a8bd-d774-41ac-b44b-a460dceae459)
_Modello 3D della stampante che sto progettando_

## Stampante FFF (Fused Filament Fabrication)
Son dette a filamento fuso perché la materia prima è una plastica a forma di filamento, come questa:

![Filamento](https://github.com/Angelo13C/3d-printer/assets/55251189/9808f0af-c6db-4d6f-9140-d8de171dbae8)

Si possono stampare varie [plastiche](https://www.simplify3d.com/resources/materials-guide/), ognuna con le sue proprietà: ci son quelle che resistono meglio alle temperature alte, 
quelle flessibili, quelle che resistono agli agenti chimici...

![Tabella materiali](https://github.com/Angelo13C/3d-printer/assets/55251189/7af19ae8-7014-4994-9e0e-5629d9d01331)
_Tabella con i vari materiali stampabili e le loro proprietà_

Gli oggetti 3D vengono stampati strato per strato: ad esempio per riprodurre un cubo vengono stampati tanti quadrati uno sopra l'altro.

## Ma come viene stampato uno strato?
Il filamento va nell'estrusore che è un componente con un motore e degli ingranaggi. Quando il motore è acceso, il filo viene spinto verso il basso, fino a quando non arriva nell'hotend.
L'hotend contiene un collegamento elettrico che si scalda e gli fa raggiungere la temperatura di 200/250/300 gradi. La plastica si fonde e fuoriesce dall'ugello.

![Estrusore + Hotend](https://github.com/Angelo13C/3d-printer/assets/55251189/eea26bc2-ab4b-49b6-a289-87d96defe6b0)

Una volta fuoriuscito dall'ugello, il materiale si solidifica anche grazie a delle ventole che vengono puntate su di esso.

Per stampare un cubo, come detto prima, c'è bisogno che la plastica fuoriuscita componga la forma di un quadrato, e per fare ciò l'hotend si deve spostare nel frattempo che il filamento viene estruso. Questo viene fatto grazie a 2 motori, uno per ogni asse che grazie ad un sistema di trasmissione con delle cinghie spostano il carrello su cui c'è l'hotend.

![image](https://github.com/Angelo13C/3d-printer/assets/55251189/f4f3208a-aa6d-457a-a4f9-05c478058811)
_Uno dei 2 motori (l'altro è sfocato in lontananza) della stampante che sto costruendo_

## E gli altri strati?
Una volta che uno strato viene completato, il letto (che è la superficie base su cui viene stampato l'oggetto) si abbassa un po' grazie ad un altro motore che è in basso alla macchina e così si possono stampare tutti gli strati sopra quelli precedenti.

![Letto](https://github.com/Angelo13C/3d-printer/assets/55251189/4e1a503e-8439-48c8-b5e6-6eea154cb199)
_Motore che fa spostare il letto (in questa immagine manca la cinghia che collega il motore alle pulegge)_

## Come fa la stampante a capire gli specifici movimenti che devono fare i suoi componenti per stampare un oggetto 3D?
Per dire alla macchina cosa stampare, un file di un modello 3D (che può essere realizzato in un programma di modellazione 3D come Blender, può essere scaricato da internet...) viene importato
in un programma chiamato Slicer. Quest'ultimo prende il modello e lo suddivide in tanti strati e lo converte in un linguaggio che la stampante può capire, che è chiamato G-code. Un file G-code 
contiene una lista di comandi che la stampante deve eseguire per realizzare l'oggetto, ad esempio il comando `G1` contiene le coordinate dove l'hotend si deve spostare, mentre `M104` imposta la 
temperatura che l'hotend deve raggiungere.

![image](https://github.com/Angelo13C/3d-printer/assets/55251189/cbb25920-4819-4299-a9db-3b5a37899231)

_Esempio di un file G-code_

Questo è un video di un'anteprima di una stampa di una barchetta fatto da uno slicer ([Ultimaker Cura](https://ultimaker.com/software/ultimaker-cura/)):

https://github.com/Angelo13C/3d-printer/assets/55251189/fe157767-07f8-41d3-a10f-a2c60290169f
