"""Writes tests/data/reference/<model>.json: PyTorch f32 embeddings that the Rust tests compare against.

Usage: python3 scripts/gen_model_reference.py [model_dir ...]
Without arguments every model under model/ is processed. Do not run with `-I`: torch is expected
in the user site-packages. Pooling and normalisation are read from the sentence-transformers
metadata of the model (1_Pooling/config.json, modules.json); prefixes come from PREFIXES.
"""
import json, pathlib, sys, torch
from transformers import AutoModel, AutoTokenizer

ROOT = pathlib.Path(__file__).resolve().parent.parent
PREFIXES = {"multilingual-e5-small": {"query": "query: ", "passage": "passage: "}}


def read_json(path, default=None):
    return json.loads(path.read_text()) if path.exists() else default


def settings(model_dir):
    pooling = read_json(model_dir / "1_Pooling/config.json", {})
    modules = read_json(model_dir / "modules.json", [])
    if pooling.get("pooling_mode_cls_token"):
        mode = "cls"
    elif pooling.get("pooling_mode_mean_tokens", True):
        mode = "mean"
    else:
        sys.exit(f"{model_dir.name}: unsupported pooling mode, extend gen_model_reference.py")
    normalize = any(m["type"].endswith("Normalize") for m in modules) or not modules
    return mode, normalize


def embed(model_dir, texts, mode, normalize):
    tok = AutoTokenizer.from_pretrained(model_dir)
    model = AutoModel.from_pretrained(model_dir, torch_dtype=torch.float32).eval()
    batch = tok(texts, padding=True, truncation=True, max_length=512, return_tensors="pt")
    with torch.no_grad():
        hidden = model(**batch).last_hidden_state
    if mode == "cls":
        out = hidden[:, 0]
    else:
        mask = batch["attention_mask"].unsqueeze(-1).float()
        out = (hidden * mask).sum(1) / mask.sum(1)
    return torch.nn.functional.normalize(out, dim=-1) if normalize else out


def main():
    dirs = [pathlib.Path(a).resolve() for a in sys.argv[1:]]
    if not dirs:
        dirs = sorted(p for p in (ROOT / "model").iterdir() if (p / "config.json").exists())
    items = read_json(ROOT / "scripts/reference_texts.json")
    out_dir = ROOT / "tests/data/reference"
    out_dir.mkdir(parents=True, exist_ok=True)
    for model_dir in dirs:
        prefixes = PREFIXES.get(model_dir.name, {})
        texts = [prefixes.get(i["role"], "") + i["text"] for i in items]
        mode, normalize = settings(model_dir)
        emb = embed(model_dir, texts, mode, normalize)
        record = {"model": model_dir.name, "pooling": mode, "normalize": normalize, "texts": texts, "embeddings": emb.tolist()}
        (out_dir / f"{model_dir.name}.json").write_text(json.dumps(record))
        print(f"{model_dir.name}: {mode} pooling, normalize={normalize}, shape={tuple(emb.shape)}")


main()
