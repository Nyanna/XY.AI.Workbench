Schreibe einen eigenen Codegenerator um aus OpenAPI YAML Schemes, java Code zu generieren.
Das Konzept basiert auf einer State Machine für die Serialisierung und einem symmetrischen Interface für die Abbildung von Client und Server.
Das ganze ist angelehnt an die SDK Generatoren von Stainless (Antropic/OpenAI).

- Zielverzeichnis ist ein neu zu erstellendes Python Projekt in `/home/user/xyan/xy.ai.workbench/codegen`
- Vorerst keine Tests
- Kommentare knapp und signifikant halten
- Zu verwendendes Beispielschema: `/home/user/xyan/xy.ai.workbench/libs/openapi/filters/deepseek.filtered.yaml`
- Ziel Rootpackage: "xy.api.codegen"

## Model
Das Python-Skript extrahiert aus dem Schema den reinen Baum udn Relationen, erzeugt jedoch niemals Code direkt.
Stattdessen rendert er pro Knoten ein Template, reicht Variablen rein und platziert den Output im richtigen Ausgabeverzeichnis.
Das Model ist vollkommen unabhängig von der verwendeten HTTP-Client/Server Bibliothek.

- Das openapi schema in einen Baum zerlegen
	- ein Request-Objekt pro Path und HTTP Method (Request + Method, also "*RequestGet")
		- URL-Parameter werden von der Client Methodensignatur abgebildet, das RequestObject bildet den JSON Body ab
	- ein Response-Objekt pro Path (*Response)
		- Jeder Response Code ein Knoten (*Code + Nummer also "Code404")
			- Jeder Content-Type ein Knoten (Kurzform also *Json oder *Txt)
	- Jeder Knoten wird in eine Klasse übersetzt in einem 
	- Pro Knoten wird ein Template verwendet
		- Das Template bekommt Variablen
			- Klassenname, Liste der Untertypen pro Type Name, FQND, Feldname im JSON
	- oneOf, anyOf, allOf, not sind eigene Knoten und erzeugen eine Klasse pro Ausprägung
		- Beispiel oneOf Dog/Cat erzeugt die Klasse -> "OneOfDogCat"
		- Identische Ausprägungen werden gemeinsam genutzt und kommen ins package "*.operators"
	- Validatoren (minimum/maximum... String Format) werden aktuell nicht unterstützt
	- Metadaten, Authorization, Server werden nicht unterstützt
	- Dictionaries, HashMaps and Associative Arrays bilden eigene Knoten und Klassen
		- gemeinsam genutzt bei identischer Ausprägung, kommen dann in das package "*.dictionaries"
	- Enums bilden eigene Knoten und Klassen, gemeinsam genutzt bei identischer Ausprägung, dann im Package "*.enums"
	- Arrays oder MixedArrays und Listen bilden einen eigenen Knoten und Klassen
		- gemeinsam genutzt bei identischer Ausprägung, kommen dann in das package "*.lists"
		- Beispiel: "StringList" oder abgeleitet vom Key "UserList" bei `user: schema": { $ref: #/components/schemas/user }`
	- Objects bilden einen eigenen Knoten und Klassen
		- gemeinsam genutzt bei identischer Ausprägung, kommen dann in das package "*.objects"
	
- Das Package für eine Klasse
	- ein Base-Package ist konfigurierbar
	- "*.components" für alle von mehreren Pfaden referenzierten Klassen
	- "*.request.*" Nach einem von "Path" abgeleiteten Package, Beispiel "*.request.<PATH>.<METHOD>.<CONTENT_TYPE>.<TREENAME>*"
		- Beispielweise: "*.request.users.get.json" für die Klasse "UsersRequestGet"
	- "*.response.*" Nach einem von "Path" abgeleiteten Package, Beispiel "*.response.<PATH>.code<HTTPCODE>.<CONTENT_TYPE>*"
		- Beispielweise: "*.response.users.code200.json" für die Klasse "UsersResponse"


## Generierung
Die Generierung erzeugt ein typsicheres symetrisches Model das JSON auf basis von JSONNode lesen und schreiben kann.

- Jackson Version 2 wird für Serialisierung verwendet
- Eine Klasse/Knoten erhält eine interne Liste, alter validen Kinder Propertyname+Typ (primitiver Typ oder Referenz)
- Jede Klasse/Knoten hält eine Referenz udn seinen JSON Knoten und initialisiert seine Kinder nur nach Bedarf
	- Schreiben: Ein kompletter Teilbaum oder primitive wird in die Instanz gegeben, diese fügt ihn, wenn laut Kindliste zulässig, sich selbst an.
	- Lesend: eine Property wird erfragt, die Instanz prüft den nächsten möglichen und lesbaren Datentyp und instanziiert das Kind mit seinem Knoten 
	- Beispiel: Es wird eine Liste erwartet -> Kind wird erzeugt, Kind füllt sich selbst "StringIntegerList"
- Die Klasse/Knoten selbst hält intern keine Kopien vom primitiven der JSON Daten, sondern nur Referenzen auf Kinder. Das entspricht einem Proxy Pattern.
- Primitive Werte werden direkt aus dem JSONNode gelesen oder geschrieben 
- Jede Klasse bekommt zwei arten von Methoden (Lesend/Schreibend)
	- Setter oder Add/Remove(Listen) Methoden
	- Getter für Primitive und Komplexe Objekte
	- Getter und Setter übernehmen Description und Example aus dem OpenAPI Schema 
- Request/Response Objekte erhalten eine toString()/fromString() Methode und kapseln Serialisierung selbst


## Http Client und Server
Server und Client sind unabhängig von der im Model genutzten JSON Serialisierung

- Java HTTPClient wird für den Client verwendet. Ein Server-Stack wird vorerst nicht generiert.
- eine Client Facade wird mittel Template gerendert und enthält eine Methode pro "Path"
	- Die Methode bildet URL Parameter ab
	- Header werden nicht unterstützt
	- Die Methode wird mit dem Root Request-Objekt aufgerufen und liefert in jedem Fall das Response-Objekt mit Ausnahme von Exceptions
	- Die Methode implementiert den Request mittels HTTPClient und verwendet die gekapselte Serialisierung der Root-Objekte
- der Client wird Zweiteilig generiert eine Implementierung Passung zu deinem Interface
- Das Interface übernimmt Description und Example aus dem Schema für "Path"