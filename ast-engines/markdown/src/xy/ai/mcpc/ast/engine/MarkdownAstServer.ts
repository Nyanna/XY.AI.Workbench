import { availableParallelism } from 'node:os';
import { createServer, type Server } from 'node:http';
import { AppendInfoNodesValidateServer } from '../openapi/AppendInfoNodesValidateServer.ts';
import { CandidatesList } from '../openapi/components/CandidatesList.ts';
import type { CodeRequest } from '../openapi/components/CodeRequest.ts';
import { Error as ErrorBody } from '../openapi/components/Error.ts';
import { MutationResponse as MutationBody } from '../openapi/components/MutationResponse.ts';
import { Node as NodeBody } from '../openapi/components/Node.ts';
import type { SourceRequest } from '../openapi/components/SourceRequest.ts';
import type { InsertRequest } from '../openapi/request/nodes/nodeid/insert/post/json/InsertRequest.ts';
import type { LocateRequest } from '../openapi/request/nodes/post/json/LocateRequest.ts';
import { AppendResponse } from '../openapi/response/append/AppendResponse.ts';
import { AppendResponse as AppendBody } from '../openapi/response/append/code200/json/AppendResponse.ts';
import { EngineInfo } from '../openapi/response/info/code200/json/EngineInfo.ts';
import { InfoResponse } from '../openapi/response/info/InfoResponse.ts';
import { LocateResponse as LocateBody } from '../openapi/response/nodes/code200/json/LocateResponse.ts';
import { NodesList } from '../openapi/response/nodes/code200/json/NodesList.ts';
import { NodesNodeIdResponse } from '../openapi/response/nodes/nodeid/NodesNodeIdResponse.ts';
import { NodesNodeIdDeleteResponse } from '../openapi/response/nodes/nodeid/delete/NodesNodeIdDeleteResponse.ts';
import { InsertResponse as InsertBody } from '../openapi/response/nodes/nodeid/insert/code200/json/InsertResponse.ts';
import { NodesNodeIdInsertResponse } from '../openapi/response/nodes/nodeid/insert/NodesNodeIdInsertResponse.ts';
import { NodesNodeIdReplaceResponse } from '../openapi/response/nodes/nodeid/replace/NodesNodeIdReplaceResponse.ts';
import { NodesResponse } from '../openapi/response/nodes/NodesResponse.ts';
import { ValidateResponse as ValidateBody } from '../openapi/response/validate/code200/json/ValidateResponse.ts';
import { ValidateResponse } from '../openapi/response/validate/ValidateResponse.ts';
import { AstEngineException } from './AstEngineException.ts';
import type { TaskExecutor } from './AstTask.ts';
import { InlineExecutor, WorkerPool } from './TaskExecutors.ts';

type ErrorSink = (error: ErrorBody) => void;

export interface MarkdownAstServerOptions {
  /** Worker threads for document work; `0` runs everything on the main thread. Defaults to the number of cores. */
  workers?: number;
}

/**
 * Wires the generated server contract to the document operations. HTTP and JSON stay on the main
 * thread; parsing, locating and printing run on a {@link WorkerPool} in which a file is only ever
 * handled by one thread at a time. A request either names a `path` (cached, persisted on every
 * mutation) or carries a stateless `source` (in-memory only); both together mismatching is a 409.
 */
export class MarkdownAstServer extends AppendInfoNodesValidateServer {
  private readonly executor: TaskExecutor;

  constructor(options: MarkdownAstServerOptions = {}) {
    super();
    const workers = options.workers ?? availableParallelism();
    this.executor = workers > 0 ? new WorkerPool(workers) : new InlineExecutor();
  }

  /** Same as the generated `start`, but bound to `host` (loopback by default) instead of every interface. */
  override start(port: number = 8788, host: string = '127.0.0.1'): Promise<Server> {
    const server = createServer((req, res) => {
      void this.handle(req, res);
    });
    return new Promise((resolve, reject) => {
      server.once('error', reject);
      server.listen(port, host, () => {
        server.off('error', reject);
        resolve(server);
      });
    });
  }

  /** Stops the worker threads. */
  close(): Promise<void> {
    return this.executor.close();
  }

  override async appendTopLevel(request: CodeRequest): Promise<AppendResponse> {
    const response = new AppendResponse();
    try {
      const result = await this.executor.submit({ op: 'append', path: request.getPath(), source: request.getSource(), code: request.getCode() });
      const body = new AppendBody();
      body.setSource(result.source);
      body.setUnitsAppended(result.units);
      response.setCode200(body);
    } catch (e) {
      applyError(e, null, null, (err) => response.setCode422(err));
    }
    return response;
  }

  override getEngineInfo(): Promise<InfoResponse> {
    const response = new InfoResponse();
    const info = new EngineInfo();
    info.setName('markdown-remark');
    info.setValidatesSyntax(false);
    response.setCode200(info);
    return Promise.resolve(response);
  }

  override async listNodes(request: LocateRequest): Promise<NodesResponse> {
    const response = new NodesResponse();
    try {
      const result = await this.executor.submit({ op: 'list', path: request.getPath(), source: request.getSource(), includeCode: request.getIncludeCode() === true });
      const body = new LocateBody();
      body.setNodes(new NodesList(result.nodes));
      response.setCode200(body);
    } catch (e) {
      applyError(e, null, null, (err) => response.setCode422(err));
    }
    return response;
  }

  override async getNode(nodeId: string, request: SourceRequest): Promise<NodesNodeIdResponse> {
    const response = new NodesNodeIdResponse();
    try {
      response.setCode200(new NodeBody(await this.executor.submit({ op: 'get', path: request.getPath(), source: request.getSource(), nodeId })));
    } catch (e) {
      applyError(e, (err) => response.setCode404(err), (err) => response.setCode409(err), (err) => response.setCode422(err));
    }
    return response;
  }

  override async deleteNode(nodeId: string, request: SourceRequest): Promise<NodesNodeIdDeleteResponse> {
    const response = new NodesNodeIdDeleteResponse();
    try {
      const result = await this.executor.submit({ op: 'delete', path: request.getPath(), source: request.getSource(), nodeId });
      const body = new MutationBody();
      body.setSource(result.source);
      response.setCode200(body);
    } catch (e) {
      applyError(e, (err) => response.setCode404(err), (err) => response.setCode409(err), (err) => response.setCode422(err));
    }
    return response;
  }

  override async insertRelativeToNode(nodeId: string, request: InsertRequest): Promise<NodesNodeIdInsertResponse> {
    const response = new NodesNodeIdInsertResponse();
    try {
      const result = await this.executor.submit({ op: 'insert', path: request.getPath(), source: request.getSource(), nodeId, code: request.getCode(), position: request.getPosition() });
      const body = new InsertBody();
      body.setSource(result.source);
      body.setUnitsInserted(result.units);
      response.setCode200(body);
    } catch (e) {
      applyError(e, (err) => response.setCode404(err), (err) => response.setCode409(err), (err) => response.setCode422(err));
    }
    return response;
  }

  override async replaceNode(nodeId: string, request: CodeRequest): Promise<NodesNodeIdReplaceResponse> {
    const response = new NodesNodeIdReplaceResponse();
    try {
      const result = await this.executor.submit({ op: 'replace', path: request.getPath(), source: request.getSource(), nodeId, code: request.getCode() });
      const body = new MutationBody();
      body.setSource(result.source);
      response.setCode200(body);
    } catch (e) {
      applyError(e, (err) => response.setCode404(err), (err) => response.setCode409(err), (err) => response.setCode422(err));
    }
    return response;
  }

  override async validateSource(request: SourceRequest): Promise<ValidateResponse> {
    const response = new ValidateResponse();
    const body = new ValidateBody();
    const result = await this.executor.submit({ op: 'validate', path: request.getPath(), source: request.getSource() });
    if (result.error !== null)
      body.setError(result.error);
    response.setCode200(body);
    return response;
  }
}

/** Maps an engine error to the response codes an operation supports; operations lacking 404/409 report 422. */
function applyError(e: unknown, code404: ErrorSink | null, code409: ErrorSink | null, code422: ErrorSink): void {
  if (!(e instanceof AstEngineException))
    throw e;
  const err = new ErrorBody();
  err.setMessage(e.message);
  if (e.candidates.length > 0) {
    const list = new CandidatesList();
    for (const c of e.candidates)
      list.add(c);
    err.setCandidates(list);
  }
  switch (e.kind) {
    case 'NOT_FOUND':
      (code404 ?? code422)(err);
      break;
    case 'CONFLICT':
      (code409 ?? code422)(err);
      break;
    case 'SYNTAX':
      code422(err);
      break;
  }
}
