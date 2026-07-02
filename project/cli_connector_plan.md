# Points

Eine Session, was ist mit resume? oder Rewind?
Multiple antworten, toolabfragen und bestätigung, stdin offen lassen?
Wie agent auswählen?

* Session UUID aus aktueller datei generieren, und mit frontmatter an den anfang schreiben, wenn von hand gelöscht neu generieren. Auch datum/zeit vom sessionstart reinschreiben mit reinschreiben.
* relevante parameter aus panel in frontmatter übernehmen, wird ein MD Frontmatter sessionformat
* Initial subprocess starten und erst beim Session ID wechsel prüfen, beenden und neu starten, oder bei eclipse exit
* Answers auf Liste umstellen und einzeln in datei rendern, mit thinking und tool

* claude-work default 
	* mit profil im key claude-work
	* mit resume wenn session vorhanden ist bei claude und im projektverzeichnis
	
* Prompt, enqueue, batch in capabilities verlagen und alles bis auf Prompt deaktivieren bei Claude code

* output modus in capabillities verlagern claude code unterstützt nur append