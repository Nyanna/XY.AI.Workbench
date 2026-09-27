package xy.ai.mcpc.ast.engine;

import com.github.javaparser.ast.CompilationUnit;
import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpHandler;
import xy.ai.mcpc.ast.openapi.AppendInfoNodesValidateServer;
import xy.ai.mcpc.ast.openapi.components.CandidatesList;
import xy.ai.mcpc.ast.openapi.components.CodeRequest;
import xy.ai.mcpc.ast.openapi.components.Error;
import xy.ai.mcpc.ast.openapi.components.MutationResponse;
import xy.ai.mcpc.ast.openapi.components.SourceRequest;
import xy.ai.mcpc.ast.openapi.request.nodes.nodeid.insert.post.json.InsertRequest;
import xy.ai.mcpc.ast.openapi.request.nodes.post.json.LocateRequest;
import xy.ai.mcpc.ast.openapi.response.append.AppendResponse;
import xy.ai.mcpc.ast.openapi.response.info.InfoResponse;
import xy.ai.mcpc.ast.openapi.response.info.code200.json.EngineInfo;
import xy.ai.mcpc.ast.openapi.response.nodes.NodesResponse;
import xy.ai.mcpc.ast.openapi.response.nodes.code200.json.LocateResponse;
import xy.ai.mcpc.ast.openapi.response.nodes.code200.json.NodesList;
import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.NodesNodeIdResponse;
import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.delete.NodesNodeIdDeleteResponse;
import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.insert.NodesNodeIdInsertResponse;
import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.insert.code200.json.InsertResponse;
import xy.ai.mcpc.ast.openapi.response.nodes.nodeid.replace.NodesNodeIdReplaceResponse;
import xy.ai.mcpc.ast.openapi.response.validate.ValidateResponse;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.List;
import java.util.function.Consumer;

/**
 * Wires the JavaParser-based {@link JavaAstEngine}/{@link DocumentCache} into the
 * generated JSON/HTTP server contract. A request either names a {@code path}
 * (cached, persisted to disk on every mutation) or carries a stateless {@code source}
 * (parsed in-memory only, never written anywhere); when both are given for an
 * existing file, a mismatch is reported as 409 (the client's view of the file is
 * stale).
 */
public final class JavaAstServer extends AppendInfoNodesValidateServer {
  private final JavaAstEngine engine = new JavaAstEngine();
  private final DocumentCache cache = new DocumentCache(engine);

  /**
   * Safety net around the generated dispatcher: any {@link Throwable} it lets through
   * (bugs, but also e.g. {@link LinkageError} from a misconfigured runtime classpath)
   * would otherwise abort the connection silently, since the generated handler only
   * catches {@link Exception}. Logs to stderr and still answers with a 500 instead.
   */
  @Override
  public HttpHandler createHttpHandler() {
    HttpHandler delegate = super.createHttpHandler();
    return (HttpExchange exchange) -> {
      try {
        delegate.handle(exchange);
      } catch (Throwable t) {
        t.printStackTrace();
        byte[] bytes = ("internal error: " + t).getBytes(StandardCharsets.UTF_8);
        try {
          exchange.getResponseHeaders().set("Content-Type", "text/plain");
          exchange.sendResponseHeaders(500, bytes.length);
          exchange.getResponseBody().write(bytes);
        } catch (IOException ignored) {
          // connection already broken; nothing more we can do.
        } finally {
          exchange.close();
        }
      }
    };
  }

  private record Resolved(CompilationUnit cu, Path path) {
  }

  private Resolved resolve(String pathStr, String sourceOverride) {
    if (pathStr != null && !pathStr.isBlank()) {
      Path path = Paths.get(pathStr);
      if (Files.exists(path)) {
        DocumentCache.Entry entry;
        try {
          entry = cache.get(path);
        } catch (IOException e) {
          throw new AstEngineException(AstEngineException.Kind.SYNTAX, "cannot read " + pathStr + ": " + e.getMessage());
        }
        if (sourceOverride != null && !sourceOverride.equals(entry.source()))
          throw new AstEngineException(AstEngineException.Kind.CONFLICT, "file changed on disk since the given source was captured: " + pathStr);
        return new Resolved(entry.cu(), path);
      }
      CompilationUnit cu = sourceOverride != null ? engine.parseCompilationUnit(sourceOverride) : engine.emptyCompilationUnit();
      return new Resolved(cu, path);
    }
    if (sourceOverride == null)
      throw new AstEngineException(AstEngineException.Kind.SYNTAX, "either 'path' or 'source' is required");
    return new Resolved(engine.parseCompilationUnit(sourceOverride), null);
  }

  private String persist(Resolved resolved) {
    if (resolved.path() != null)
      try {
        return cache.save(resolved.path(), resolved.cu());
      } catch (IOException e) {
        throw new AstEngineException(AstEngineException.Kind.SYNTAX, "cannot write " + resolved.path() + ": " + e.getMessage());
      }
    return engine.print(resolved.cu());
  }

  private AddressableNode find(CompilationUnit cu, String nodeId) {
    List<AddressableNode> located = NodeLocator.locateAll(cu);
    for (AddressableNode a : located) if (a.id.equals(nodeId))
      return a;
    List<String> candidates = located.stream().map(a -> a.id).filter(id -> id.contains(nodeId) || nodeId.contains(id)).limit(5).toList();
    throw new AstEngineException(AstEngineException.Kind.NOT_FOUND, "node not found: " + nodeId, candidates);
  }

  private static Error errorOf(AstEngineException e) {
    Error err = new Error();
    err.setMessage(e.getMessage());
    if (!e.candidates().isEmpty()) {
      CandidatesList list = new CandidatesList();
      for (String c : e.candidates()) list.add(c);
      err.setCandidates(list);
    }
    return err;
  }

  private static void applyError(AstEngineException e, Consumer<Error> code404, Consumer<Error> code409, Consumer<Error> code422) {
    Error err = errorOf(e);
    switch(e.kind()) {
      case NOT_FOUND ->
        (code404 != null ? code404 : code422).accept(err);
      case CONFLICT ->
        (code409 != null ? code409 : code422).accept(err);
      case SYNTAX ->
        code422.accept(err);
    }
  }

  private xy.ai.mcpc.ast.openapi.components.Node toDto(AddressableNode a, boolean includeCode) {
    xy.ai.mcpc.ast.openapi.components.Node dto = new xy.ai.mcpc.ast.openapi.components.Node();
    dto.setId(a.id);
    dto.setType(a.type);
    if (a.name != null)
      dto.setName(a.name);
    dto.setLineno((long) a.lineno);
    dto.setEndLineno((long) a.endLineno);
    dto.setParentType(a.parentType);
    dto.setExpandable(a.expandable);
    dto.setIsDefinition(a.isDefinition);
    if (a.isDefinition) {
      dto.setSignature(engine.signature(a.astNode, 80));
      String doc = engine.docstring(a.astNode, 80);
      if (doc != null)
        dto.setDocstring(doc);
    }
    if (includeCode)
      dto.setCode(engine.print(a.astNode));
    return dto;
  }

  @Override
  protected AppendResponse appendTopLevel(CodeRequest request) {
    AppendResponse response = new AppendResponse();
    try {
      Resolved resolved = resolve(request.getPath(), request.getSource());
      JavaAstEngine.AppendResult appended = engine.append(resolved.cu(), request.getCode());
      String source = persist(new Resolved(appended.cu(), resolved.path()));
      xy.ai.mcpc.ast.openapi.response.append.code200.json.AppendResponse body = new xy.ai.mcpc.ast.openapi.response.append.code200.json.AppendResponse();
      body.setSource(source);
      body.setUnitsAppended((long) appended.units());
      response.setCode200(body);
    } catch (AstEngineException e) {
      response.setCode422(errorOf(e));
    }
    return response;
  }

  @Override
  protected InfoResponse getEngineInfo() {
    InfoResponse response = new InfoResponse();
    EngineInfo info = new EngineInfo();
    info.setName("java-javaparser");
    info.setValidatesSyntax(true);
    response.setCode200(info);
    return response;
  }

  @Override
  protected NodesResponse listNodes(LocateRequest request) {
    NodesResponse response = new NodesResponse();
    try {
      Resolved resolved = resolve(request.getPath(), request.getSource());
      boolean includeCode = Boolean.TRUE.equals(request.getIncludeCode());
      NodesList list = new NodesList();
      for (AddressableNode a : NodeLocator.locateAll(resolved.cu())) list.add(toDto(a, includeCode));
      LocateResponse body = new LocateResponse();
      body.setNodes(list);
      response.setCode200(body);
    } catch (AstEngineException e) {
      response.setCode422(errorOf(e));
    }
    return response;
  }

  @Override
  protected NodesNodeIdResponse getNode(String nodeId, SourceRequest request) {
    NodesNodeIdResponse response = new NodesNodeIdResponse();
    try {
      Resolved resolved = resolve(request.getPath(), request.getSource());
      AddressableNode target = find(resolved.cu(), nodeId);
      response.setCode200(toDto(target, true));
    } catch (AstEngineException e) {
      applyError(e, response::setCode404, response::setCode409, response::setCode422);
    }
    return response;
  }

  @Override
  protected NodesNodeIdDeleteResponse deleteNode(String nodeId, SourceRequest request) {
    NodesNodeIdDeleteResponse response = new NodesNodeIdDeleteResponse();
    try {
      Resolved resolved = resolve(request.getPath(), request.getSource());
      AddressableNode target = find(resolved.cu(), nodeId);
      engine.delete(target);
      String source = persist(resolved);
      MutationResponse body = new MutationResponse();
      body.setSource(source);
      response.setCode200(body);
    } catch (AstEngineException e) {
      applyError(e, response::setCode404, response::setCode409, response::setCode422);
    }
    return response;
  }

  @Override
  protected NodesNodeIdInsertResponse insertRelativeToNode(String nodeId, InsertRequest request) {
    NodesNodeIdInsertResponse response = new NodesNodeIdInsertResponse();
    try {
      Resolved resolved = resolve(request.getPath(), request.getSource());
      AddressableNode target = find(resolved.cu(), nodeId);
      String position = request.getPosition() == null ? "after" : request.getPosition().rawValue();
      int units = engine.insert(target, request.getCode(), position);
      String source = persist(resolved);
      InsertResponse body = new InsertResponse();
      body.setSource(source);
      body.setUnitsInserted((long) units);
      response.setCode200(body);
    } catch (AstEngineException e) {
      applyError(e, response::setCode404, response::setCode409, response::setCode422);
    }
    return response;
  }

  @Override
  protected NodesNodeIdReplaceResponse replaceNode(String nodeId, CodeRequest request) {
    NodesNodeIdReplaceResponse response = new NodesNodeIdReplaceResponse();
    try {
      Resolved resolved = resolve(request.getPath(), request.getSource());
      AddressableNode target = find(resolved.cu(), nodeId);
      engine.replace(target, request.getCode());
      String source = persist(resolved);
      MutationResponse body = new MutationResponse();
      body.setSource(source);
      response.setCode200(body);
    } catch (AstEngineException e) {
      applyError(e, response::setCode404, response::setCode409, response::setCode422);
    }
    return response;
  }

  @Override
  protected ValidateResponse validateSource(SourceRequest request) {
    ValidateResponse response = new ValidateResponse();
    xy.ai.mcpc.ast.openapi.response.validate.code200.json.ValidateResponse body = new xy.ai.mcpc.ast.openapi.response.validate.code200.json.ValidateResponse();
    try {
      String source = request.getSource();
      if (source == null) {
        if (request.getPath() == null || request.getPath().isBlank())
          throw new AstEngineException(AstEngineException.Kind.SYNTAX, "either 'path' or 'source' is required");
        source = Files.readString(Paths.get(request.getPath()), StandardCharsets.UTF_8);
      }
      String error = engine.validate(source);
      if (error != null)
        body.setError(error);
    } catch (IOException e) {
      body.setError("cannot read " + request.getPath() + ": " + e.getMessage());
    } catch (AstEngineException e) {
      body.setError(e.getMessage());
    }
    response.setCode200(body);
    return response;
  }
}
