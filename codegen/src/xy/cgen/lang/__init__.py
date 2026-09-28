"""Registry of per-language code-generation support (`LanguageSupport`
implementations). Naming/emit look up the target language here instead of
branching on the language string themselves; adding a language means adding
one module (`java.py`, `php.py`, ...) plus a `templates/<language>` dir.
"""
from xy.cgen.lang.base import LanguageSupport
from xy.cgen.lang.java import JAVA
from xy.cgen.lang.php import PHP
LANGUAGES = {'java': JAVA, 'php': PHP}

def get_language(language: str) -> LanguageSupport:
    return LANGUAGES[language]