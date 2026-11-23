## Punkt 1: `first_resource_kind_of_task`

Meine Umsetzung stützt sich auf keine bestehende Konvention, und die Funktion kann ihren Zweck nicht erfüllen.

- **Keine Konvention für `params.resource`:** `TaskParams` ist laut Doku "not interpreted". Im Code wertet nichts `params.resource` aus. Meine Annahme, dort stehe der Resource-Name des ersten Schritts, habe ich damit nicht belegt. Die Tests in `task_model` benutzen den Wert `"gpu"`, also eher eine Geräteklasse. Eine Engine, die das Feld anders belegt, bekäme von meiner Funktion still `None`.
- **Aufruf zur falschen Zeit:** `Processor::submit(data, params, first_kind)` legt den `Task` erst selbst an. Der Wert wird also gebraucht, bevor es einen `Task` gibt. Nach dem Push lässt sich `TaskMeta.first_kind` nicht mehr ändern. Eine Funktion, die einen `&Task` nimmt, kann den Wert für die Task-Queue nie liefern.
- **Brauchbarer Weg:** `first_resource_kind(&TaskSpec)` ist rein und stimmt laut Test mit der ersten Operation des Expanders überein. Die Engine kann sie vor `submit` aufrufen, sofern sie den Spec schon kennt.

Optionen:
- **A:** `first_resource_kind_of_task` entfällt, und die Engine nutzt `first_resource_kind(&spec)` vor `submit`. Das ist mein Vorschlag, weicht aber von der Spec-Schnittstelle ab.
- **B:** Die Funktion bleibt als Spec-Element, liefert aber immer `None`, mit einem Hinweis in der Doku. Das erfindet nichts, ist aber wertlos.
- **C:** Mein jetziger Stand bleibt, mit der festgehaltenen Annahme zum Feld.

## Punkt 3: Doppelte `ModelId` und `TaskId`

- **`ModelId`:** Der Expander überbrückt die zwei Typen selbst über den String (`ModelId::new(task.model.0.as_str())`), und `From`-Impls dazwischen gibt es nicht. Mein Planner verfährt genauso und schlägt das Modell mit demselben Schlüssel nach. Der Test `model_is_always_the_default_model` deckt Embedding und Similarity ab, auch mit einem unbekannten Modellnamen im Spec.
- **`TaskId`:** Die Doku in `operation.rs` verlangt die explizite Umwandlung mit `.0` ausdrücklich. `OpMeta.task_id` ist `operation::TaskId`, wie `Operation`. `OperationNode::new` und `TaskContext::new` nehmen `u64`. Der Referenz-`ValueBuilder` im Test wandelt so um.

Hier ist nichts zu ändern.

## Punkt 4: Umbenennung des Traits

- **Aufwand:** Nur `processor.rs` (Trait und die Bedingung `PL: Planner`) und `test/processor.rs` (ein `impl`) nutzen den Trait-Namen. `ExpandError::Planner` ist ein Variantenname und bleibt. In meinem Modul entfiele der Alias `Planner as TaskPlanner`.
- **Risiko:** Die Spec nennt den Trait `processor::Planner`. Die noch offenen Specs, etwa die der Engine, werden ihn vermutlich auch so nennen.

Ich würde nur umbenennen, wenn Sie die offenen Specs anpassen oder dafür sorgen, dass sie den neuen Namen verwenden. Sonst ist der Alias die risikoärmere Lösung.