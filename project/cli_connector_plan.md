# Points

* Session UUID aus aktueller datei generieren, und mit frontmatter an den anfang schreiben, wenn von hand gelöscht neu generieren 
	* Eine Session, was ist mit resume? oder Rewind?
	* Initial subprocess starten und erst beim Session ID wechsel prüfen, beenden und neu starten
	* mit resume wenn session vorhanden ist bei claude und im projektverzeichnis
	* /resume setzt session hash aus datei fort
* subagent interleaing -> gibt es nicht mit MCP Controller -> should no problem at all
* automatisch allow in erwähnten dateien
* datum/zeit vom sessionstart reinschreiben mit reinschreiben. in json datei als meta systen nachricht
	* relevante parameter aus panel in frontmatter übernehmen, wird ein MD Frontmatter sessionformat, ebenfalls als systemnachricht übernehmen CLI path, profil usw