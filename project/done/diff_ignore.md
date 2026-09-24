Implementiere in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/view/diff/OpSnapshotter.java` einen Filter für den Diff.
Dazu soll eine Datei (wenn vorhanden) ".diffignore" im Repository Root im Format einer ".gitignore" verwendet werden.

Beispiel:
```java
DirCacheEditor editor = cache.editor();
for (String pathToExclude : excludedPaths) {
    editor.add(new DirCacheEditor.DeletePath(pathToExclude));
    // für ganze Verzeichnisse stattdessen:
    // editor.add(new DirCacheEditor.DeleteTree(pathToExclude));
}
editor.finish();

IgnoreNode diffIgnore = new IgnoreNode();
try (InputStream in = new FileInputStream(new File(repoDir, ".diffignore"))) {
    diffIgnore.parse(in);
}

try (Git git = new Git(repository);
     ObjectReader reader = repository.newObjectReader()) {

    List<DiffEntry> diffs = git.diff()
        .setOldTree(oldTreeIter)
        .setNewTree(newTreeIter)
        .call();

    List<DiffEntry> filtered = diffs.stream()
        .filter(entry -> {
            String path = entry.getChangeType() == DiffEntry.ChangeType.DELETE
                ? entry.getOldPath()
                : entry.getNewPath();
            MatchResult r = diffIgnore.isIgnored(path, false);
            return r != MatchResult.IGNORED;
        })
        .collect(Collectors.toList());
}
```

