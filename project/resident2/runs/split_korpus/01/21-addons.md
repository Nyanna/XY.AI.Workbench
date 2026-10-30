## Addons

- Ein Task kann den Scope eines Folge-Tasks treiben. Iteration und Rekursion entstehen allein durch ein Addon in einer Subengine, der Anwendung oder einer Ressourcenimplementierung (Index, Modell).
- Ein Addon enqueued Tasks über den normalen Weg, mit geklonten oder geänderten Parametern. Gezogen werden sie über die vorhandenen Pull-Filter (Priorität, Affinity).
- Addon-Tasks sind von Anwendungs-Tasks nicht unterscheidbar. Der Kern braucht dafür nur das erweiterbare Parameterobjekt.
- Beispiele: ein Centroid-Task gegen die Summary-Struktur mit einem Folge-Task über die Mitglieder der gewählten Zellen; eine Textanfrage als Embedding-Task gefolgt von einem Similarity-Task; ein Reranking als zweite Similarity auf dem verengten Scope.
