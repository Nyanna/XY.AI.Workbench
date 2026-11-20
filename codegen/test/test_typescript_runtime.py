import re
import subprocess
import textwrap
from pathlib import Path
import pytest
from xy.cgen.cli import main


SPEC = textwrap.dedent("""\
    openapi: 3.1.0
    info: {title: t, version: '1'}
    paths:
      /things/{thingId}:
        get:
          operationId: getThing
          parameters:
            - {name: thingId, in: path, required: true, schema: {type: integer}}
            - {name: flag, in: query, schema: {type: boolean}}
            - {name: q, in: query, schema: {type: string}}
          responses:
            '200':
              description: ok
              content:
                application/json:
                  schema: {$ref: '#/components/schemas/Thing'}
            '404':
              description: missing
              content:
                application/json:
                  schema: {$ref: '#/components/schemas/Problem'}
      /things:
        post:
          operationId: createThing
          requestBody:
            content:
              application/json:
                schema: {$ref: '#/components/schemas/Thing'}
          responses:
            '201':
              description: created
              content:
                application/json:
                  schema: {$ref: '#/components/schemas/Thing'}
      /things/export:
        get:
          operationId: exportThings
          responses:
            '200':
              description: lines
              content:
                application/x-jsonl:
                  schema: {$ref: '#/components/schemas/Thing'}
    components:
      schemas:
        Kind:
          type: string
          enum: [small, extra-large]
        Problem:
          type: object
          properties:
            message: {type: string}
        Dog:
          type: object
          properties:
            kind: {type: string, enum: [dog]}
        Cat:
          type: object
          properties:
            kind: {type: string, enum: [cat]}
        Pet:
          oneOf:
            - {$ref: '#/components/schemas/Dog'}
            - {$ref: '#/components/schemas/Cat'}
          discriminator: {propertyName: kind}
        Labels:
          type: object
          additionalProperties: {type: string}
        Thing:
          type: object
          properties:
            id: {type: integer}
            name: {type: string}
            kind: {$ref: '#/components/schemas/Kind'}
            pet: {$ref: '#/components/schemas/Pet'}
            labels: {$ref: '#/components/schemas/Labels'}
            constructor: {type: string}
    """)

SCRIPT = textwrap.dedent("""\
    import assert from 'node:assert/strict';
    import { pathToFileURL } from 'node:url';

    const api = (name) => import(pathToFileURL(process.argv[2] + '/gen/api/' + name + '.ts').href);
    const { ThingsServer } = await api('ThingsServer');
    const { ThingsClientImpl } = await api('ThingsClientImpl');
    const JsonSupport = await api('JsonSupport');
    const { Thing } = await api('components/Thing');
    const { KindEnum } = await api('components/KindEnum');
    const { OneOfPet } = await api('components/OneOfPet');
    const { LabelsDict } = await api('components/LabelsDict');
    const { ThingsThingIdResponse } = await api('response/things/thingid/ThingsThingIdResponse');
    const { Problem } = await api('response/things/thingid/code404/json/Problem');
    const { ThingsResponse } = await api('response/things/ThingsResponse');

    const seen = {};
    class Impl extends ThingsServer {
      async getThing(thingId, flag, q) {
        seen.args = { thingId, flag, q };
        const response = new ThingsThingIdResponse();
        if (thingId === 404) {
          const problem = new Problem();
          problem.setMessage('missing');
          response.setCode404(problem);
        } else {
          const thing = new Thing();
          thing.setId(thingId);
          thing.setKind(KindEnum.EXTRA_LARGE);
          thing.setConstructor('c');
          response.setCode200(thing);
        }
        return response;
      }
      async createThing(request) {
        seen.body = String(request);
        const response = new ThingsResponse();
        response.setCode201(request);
        return response;
      }
      async exportThings() { throw new Error('boom'); }
    }

    const server = await new Impl().start(0);
    const base = 'http://127.0.0.1:' + server.address().port;
    try {
      const client = new ThingsClientImpl(base);
      const ok = await client.getThing(7, true, 'a b&c');
      assert.deepEqual(seen.args, { thingId: 7, flag: true, q: 'a b&c' });
      assert.equal(ok.statusCode, '200');
      assert.equal(ok.getCode404(), null);
      const thing = ok.getCode200().getJson();
      assert.equal(thing.getId(), 7);
      assert.equal(thing.getKind(), 'extra-large');
      assert.equal(thing.getConstructor(), 'c');
      assert.equal(thing.getName(), null);

      const missing = await client.getThing(404, null, null);
      assert.deepEqual(seen.args, { thingId: 404, flag: null, q: null });
      assert.equal(missing.getCode404().getJson().getMessage(), 'missing');

      const input = new Thing();
      input.setName('n');
      const created = await client.createThing(input);
      assert.equal(seen.body, '{"name":"n"}');
      assert.equal(created.getCode201().getJson().getName(), 'n');

      assert.equal((await fetch(base + '/nope')).status, 404);
      const failed = await fetch(base + '/things/export');
      assert.equal(failed.status, 500);
      assert.equal(await failed.text(), 'boom');

      assert.ok(new OneOfPet({ kind: 'dog' }).isDog());
      assert.ok(!new OneOfPet({ kind: 'dog' }).isCat());
      assert.ok(!new OneOfPet(null).isDog());
      const labels = new LabelsDict();
      labels.put('__proto__', 'x');
      assert.equal(labels.get('toString'), null);
      assert.equal(labels.get('__proto__'), 'x');

      assert.deepEqual(JsonSupport.parseBody('{"a":1}\\n\\n{"a":2}\\n', 'Application/X-JSONL; charset=utf-8'), [{ a: 1 }, { a: 2 }]);
      assert.deepEqual(JsonSupport.parseBody('', 'application/x-jsonl'), []);
      assert.deepEqual(JsonSupport.parseBody('[1,2]', null), [1, 2]);
      assert.equal(JsonSupport.writeBody([{ a: 1 }, { b: 2 }], 'application/x-jsonl'), '{"a":1}\\n{"b":2}\\n');
      assert.equal(JsonSupport.writeBody([{ a: 1 }], 'application/json'), '[{"a":1}]');
      assert.ok(JsonSupport.isContentType('application/json; charset=utf-8', 'application/json'));
      assert.ok(!JsonSupport.isContentType(null, 'application/json'));
    } finally {
      server.close();
      server.closeAllConnections();
    }
    """)


@pytest.fixture(scope='module')
def generated(tmp_path_factory) -> Path:
    root = tmp_path_factory.mktemp('ts')
    schema = root / 'openapi.yaml'
    schema.write_text(SPEC, encoding='utf-8')
    out = root / 'out'
    main(['--schema', str(schema), '--out', str(out), '--base-package', 'gen.api', '--language', 'typescript'])
    (root / 'run.mjs').write_text(SCRIPT, encoding='utf-8')
    return root


def test_generated_typescript_runs_on_node(generated):
    result = subprocess.run(['node', str(generated / 'run.mjs'), str(generated / 'out')],
                            capture_output=True, text=True, timeout=60)
    assert result.returncode == 0, result.stderr


def test_imports_are_relative_ts_files(generated):
    sources = list((generated / 'out').rglob('*.ts'))
    assert sources
    for source in sources:
        for specifier in re.findall(r'from "([^"]+)"', source.read_text(encoding='utf-8')):
            if specifier.startswith('node:'):
                continue
            assert specifier.startswith('.') and specifier.endswith('.ts'), (source, specifier)
            assert (source.parent / specifier).resolve().is_file(), (source, specifier)


def test_interface_is_imported_type_only(generated):
    impl, = (generated / 'out').rglob('ThingsClientImpl.ts')
    assert re.search(r'^import type \{ ThingsClient as ', impl.read_text(encoding='utf-8'), re.MULTILINE)
