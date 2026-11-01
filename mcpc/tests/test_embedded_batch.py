import pytest
pytest.importorskip('websockets')
from xy.ai.mcpc.embedded import EmbeddedMcpc, Effort, Model
PROMPTS = ['What is 2+3? Answer with the number only.',
           'What is the capital of France? Answer with one word.',
           "Spell the word 'cat' backwards. Answer with the word only."]
EXPECTED = ['5', 'paris', 'tac']

def test_haiku_batch_over_three_prompts():
    with EmbeddedMcpc(builtin_tools=False, auto_approve=True) as mcpc:
        results = mcpc.run_batch(
            PROMPTS,
            'Answer as briefly as possible.',
            model=Model.HAIKU,
            effort=Effort.DISABLED,
            cc_profile='personal',
            max_workers=3)
    assert len(results) == len(PROMPTS)
    for result, expected in zip(results, EXPECTED):
        assert not result.is_error, result.text
        assert expected in result.text.lower()