"""Registry of per-language code-generation support (`LanguageSupport`
implementations). Naming/emit look up the target language here instead of
branching on the language string themselves; adding a language means adding
one module (`java.py`, `php.py`, ...) plus a `templates/<language>` dir.
"""
from xy.cgen.lang.base import LanguageSupport
from xy.cgen.lang.rust import RUST
from xy.cgen.lang.java import JAVA
from xy.cgen.lang.php import PHP
from xy.cgen.lang.python import PYTHON
LANGUAGES = {'java': JAVA, 'php': PHP, 'python': PYTHON, 'rust': RUST}

def get_language(language: str) -> LanguageSupport:
    return LANGUAGES[language]