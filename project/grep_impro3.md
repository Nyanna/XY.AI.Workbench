line_numbers anpassen -> für welchen grund werden linenumbers benötigt, unterscheiden nach suche und zielsuche
- duplicate wenn locate aus matchliste entfernen

```yaml
id: fc3c-e6
result:
  structuredContent:
    results:
    - directory:
      - /home/user/xyan/xy.ai.workbench/rag/src/resident/manager
      matches:
      - path: upload.rs
        matches:
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(feature = \"cuda\")]"
        - "#[cfg(test)]"
      - path: swap.rs
        matches:
        - "#[cfg(loom)]"
        - "#[cfg(not(loom))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(feature = \"cuda\", not(loom)))]"
        - "#[cfg(all(test, not(loom)))]"
        - "#[cfg(loom)]"
        - "#[cfg(all(test, feature = \"cuda\", not(loom)))]"
      - path: test/upload.rs
        matches:
        - "#[cfg(feature = \"cuda\")]"
      warning: "Limit of 15 matches reached; further results may exist. Narrow the pattern, directory or include/exclude filters, or raise limit."
```