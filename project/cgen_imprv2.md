Wir wollen die Codegenerierung in `/home/user/xyan/xy.ai.workbench/codegen` korrigieren.
- Testschema verwenden: `/home/user/xyan/xy.ai.workbench/libs/openapi/filters/deepseek.filtered.yaml`
- Warum werden diese nicht dedupliziert: "CompletedInProgressIncompleteEnum" und "StatusEnum"
- "InputContent" oder "Tool" unter "*.lists" ohne Liste zu sein, sind sie Exklusiv gehören sie zum Besitzer, werden sie geteilt bestimmt ihr Typ wo sie landen. in diesem Fall unter "*.objects"
- "CustomToolCallOutputResourceAllOf" ist ein Operator und sollte unter "*.operators" liegen wenn er geteilt ist
- Es scheint einen Fehler beim "AllOf" zu geben. Dieses müsste ein Prefix sein so wie "AnyOf" und "OneOf"
