//! Wires the `syn`-based engine ([`rust_ast_engine`] + [`DocumentCache`]) into the
//! generated JSON/HTTP server contract. A request either names a `path` (cached,
//! persisted to disk on every mutation) or carries a stateless `source` (parsed
//! in-memory only, never written anywhere); when both are given for an existing
//! file, a mismatch is reported as 409 (the client's view of the file is stale).
//! Mirrors Java's `JavaAstServer`.
use std::path::{Path, PathBuf};
use crate::engine::addressable_node::AddressableNode;
use crate::engine::ast_engine_exception::{AstEngineException, AstResult, Kind};
use crate::engine::document_cache::DocumentCache;
use crate::engine::node_locator;
use crate::engine::rust_ast_engine;
use crate::generated::engine::components;
use crate::generated::engine::request;
use crate::generated::engine::response;
use crate::generated::engine::AppendInfoNodesValidateServer::AppendInfoNodesValidateServer;
pub struct RustAstServer {
    cache: DocumentCache,
}
impl RustAstServer {
    pub fn new() -> Self {
        Self {
            cache: DocumentCache::new(),
        }
    }
}
struct Resolved {
    file: syn::File,
    path: Option<PathBuf>,
}
impl RustAstServer {
    fn resolve(
        &self,
        path_str: Option<String>,
        source_override: Option<String>,
    ) -> AstResult<Resolved> {
        if let Some(p) = path_str.filter(|p| !p.trim().is_empty()) {
            let path = PathBuf::from(&p);
            if path.exists() {
                let entry = self.cache.get(&path)?;
                if let Some(src) = &source_override {
                    if *src != entry.source {
                        return Err(
                            AstEngineException::new(
                                Kind::Conflict,
                                format!(
                                    "file changed on disk since the given source was captured: {}",
                                    p
                                ),
                            ),
                        );
                    }
                }
                return Ok(Resolved {
                    file: entry.file,
                    path: Some(path),
                });
            }
            let file = match source_override {
                Some(src) => rust_ast_engine::parse_file(&src)?,
                None => rust_ast_engine::empty_file(),
            };
            return Ok(Resolved { file, path: Some(path) });
        }
        let source = source_override
            .ok_or_else(|| AstEngineException::new(
                Kind::Syntax,
                "either 'path' or 'source' is required",
            ))?;
        Ok(Resolved {
            file: rust_ast_engine::parse_file(&source)?,
            path: None,
        })
    }
    fn persist(&self, resolved: Resolved) -> AstResult<String> {
        match resolved.path {
            Some(path) => self.cache.save(&path, resolved.file),
            None => Ok(rust_ast_engine::print_file(&resolved.file)),
        }
    }
    fn find(&self, file: &syn::File, node_id: &str) -> AstResult<AddressableNode> {
        let located = node_locator::locate_all(file);
        if let Some(found) = located.iter().find(|n| n.id == node_id) {
            return Ok(found.clone());
        }
        let candidates: Vec<String> = located
            .iter()
            .map(|n| n.id.clone())
            .filter(|id| id.contains(node_id) || node_id.contains(id.as_str()))
            .take(5)
            .collect();
        Err(
            AstEngineException::with_candidates(
                Kind::NotFound,
                format!("node not found: {}", node_id),
                candidates,
            ),
        )
    }
    fn error_dto(e: &AstEngineException) -> components::Error::Error {
        let mut err = components::Error::Error::new(serde_json::Value::Null);
        err.set_Message(Some(e.message().to_string()));
        if !e.candidates().is_empty() {
            let mut list = components::CandidatesList::CandidatesList::new(
                serde_json::Value::Null,
            );
            for c in e.candidates() {
                list.add(c.clone());
            }
            err.set_Candidates(Some(list));
        }
        err
    }
    fn to_dto(
        file: &syn::File,
        node: &AddressableNode,
        include_code: bool,
    ) -> components::Node::Node {
        let mut dto = components::Node::Node::new(serde_json::Value::Null);
        dto.set_Id(Some(node.id.clone()));
        dto.set_Type(Some(node.node_type.clone()));
        if let Some(name) = &node.name {
            dto.set_Name(Some(name.clone()));
        }
        dto.set_Lineno(Some(node.lineno));
        dto.set_EndLineno(Some(node.end_lineno));
        dto.set_ParentType(Some(node.parent_type.clone()));
        dto.set_Expandable(Some(node.expandable));
        dto.set_IsDefinition(Some(node.is_definition));
        if node.is_definition {
            dto.set_Signature(Some(rust_ast_engine::signature(file, node, 80)));
            if let Some(doc) = rust_ast_engine::docstring(file, node) {
                dto.set_Docstring(Some(doc));
            }
        }
        if include_code {
            dto.set_Code(Some(rust_ast_engine::print_node(file, node)));
        }
        dto
    }
}
#[allow(non_snake_case)]
impl AppendInfoNodesValidateServer for RustAstServer {
    fn appendTopLevel(
        &self,
        request: components::CodeRequest::CodeRequest,
    ) -> response::append::AppendResponse::AppendResponse {
        let mut response = response::append::AppendResponse::AppendResponse::new();
        let outcome: AstResult<(String, i64)> = (|| {
            let resolved = self.resolve(request.get_Path(), request.get_Source())?;
            let code = request.get_Code().unwrap_or_default();
            let (file, units) = rust_ast_engine::append(resolved.file, &code)?;
            let source = self
                .persist(Resolved {
                    file,
                    path: resolved.path,
                })?;
            Ok((source, units))
        })();
        match outcome {
            Ok((source, units)) => {
                let mut body = response::append::code200::json::AppendResponse::AppendResponse::new(
                    serde_json::Value::Null,
                );
                body.set_Source(Some(source));
                body.set_UnitsAppended(Some(units));
                response.setCode200(body);
            }
            Err(e) => response.setCode422(Self::error_dto(&e)),
        }
        response
    }
    fn getEngineInfo(&self) -> response::info::InfoResponse::InfoResponse {
        let mut response = response::info::InfoResponse::InfoResponse::new();
        let mut info = response::info::code200::json::EngineInfo::EngineInfo::new(
            serde_json::Value::Null,
        );
        info.set_Name(Some("rust-syn".to_string()));
        info.set_ValidatesSyntax(Some(true));
        response.setCode200(info);
        response
    }
    fn listNodes(
        &self,
        request: request::nodes::post::json::LocateRequest::LocateRequest,
    ) -> response::nodes::NodesResponse::NodesResponse {
        let mut response = response::nodes::NodesResponse::NodesResponse::new();
        let outcome: AstResult<
            response::nodes::code200::json::LocateResponse::LocateResponse,
        > = (|| {
            let resolved = self.resolve(request.get_Path(), request.get_Source())?;
            let include_code = request.get_IncludeCode().unwrap_or(false);
            let mut list = response::nodes::code200::json::NodesList::NodesList::new(
                serde_json::Value::Null,
            );
            for node in node_locator::locate_all(&resolved.file) {
                list.add(Self::to_dto(&resolved.file, &node, include_code));
            }
            let mut body = response::nodes::code200::json::LocateResponse::LocateResponse::new(
                serde_json::Value::Null,
            );
            body.set_Nodes(Some(list));
            Ok(body)
        })();
        match outcome {
            Ok(body) => response.setCode200(body),
            Err(e) => response.setCode422(Self::error_dto(&e)),
        }
        response
    }
    fn getNode(
        &self,
        nodeId: String,
        request: components::SourceRequest::SourceRequest,
    ) -> response::nodes::nodeid::NodesNodeIdResponse::NodesNodeIdResponse {
        let mut response = response::nodes::nodeid::NodesNodeIdResponse::NodesNodeIdResponse::new();
        let outcome: AstResult<components::Node::Node> = (|| {
            let resolved = self.resolve(request.get_Path(), request.get_Source())?;
            let node = self.find(&resolved.file, &nodeId)?;
            Ok(Self::to_dto(&resolved.file, &node, true))
        })();
        match outcome {
            Ok(dto) => response.setCode200(dto),
            Err(e) => {
                let err = Self::error_dto(&e);
                match e.kind() {
                    Kind::NotFound => response.setCode404(err),
                    Kind::Conflict => response.setCode409(err),
                    Kind::Syntax => response.setCode422(err),
                }
            }
        }
        response
    }
    fn deleteNode(
        &self,
        nodeId: String,
        request: components::SourceRequest::SourceRequest,
    ) -> response::nodes::nodeid::delete::NodesNodeIdDeleteResponse::NodesNodeIdDeleteResponse {
        let mut response = response::nodes::nodeid::delete::NodesNodeIdDeleteResponse::NodesNodeIdDeleteResponse::new();
        let outcome: AstResult<String> = (|| {
            let resolved = self.resolve(request.get_Path(), request.get_Source())?;
            let node = self.find(&resolved.file, &nodeId)?;
            let mut file = resolved.file;
            rust_ast_engine::delete(&mut file, &node)?;
            self.persist(Resolved {
                file,
                path: resolved.path,
            })
        })();
        match outcome {
            Ok(source) => {
                let mut body = components::MutationResponse::MutationResponse::new(
                    serde_json::Value::Null,
                );
                body.set_Source(Some(source));
                response.setCode200(body);
            }
            Err(e) => {
                let err = Self::error_dto(&e);
                match e.kind() {
                    Kind::NotFound => response.setCode404(err),
                    Kind::Conflict => response.setCode409(err),
                    Kind::Syntax => response.setCode422(err),
                }
            }
        }
        response
    }
    fn insertRelativeToNode(
        &self,
        nodeId: String,
        request: request::nodes::nodeid::insert::post::json::InsertRequest::InsertRequest,
    ) -> response::nodes::nodeid::insert::NodesNodeIdInsertResponse::NodesNodeIdInsertResponse {
        let mut response = response::nodes::nodeid::insert::NodesNodeIdInsertResponse::NodesNodeIdInsertResponse::new();
        let outcome: AstResult<(String, i64)> = (|| {
            let resolved = self.resolve(request.get_Path(), request.get_Source())?;
            let node = self.find(&resolved.file, &nodeId)?;
            let position = request
                .get_Position()
                .map(|p| p.raw_value())
                .unwrap_or_else(|| "after".to_string());
            let code = request.get_Code().unwrap_or_default();
            let mut file = resolved.file;
            let units = rust_ast_engine::insert(&mut file, &node, &code, &position)?;
            let source = self
                .persist(Resolved {
                    file,
                    path: resolved.path,
                })?;
            Ok((source, units))
        })();
        match outcome {
            Ok((source, units)) => {
                let mut body = response::nodes::nodeid::insert::code200::json::InsertResponse::InsertResponse::new(
                    serde_json::Value::Null,
                );
                body.set_Source(Some(source));
                body.set_UnitsInserted(Some(units));
                response.setCode200(body);
            }
            Err(e) => {
                let err = Self::error_dto(&e);
                match e.kind() {
                    Kind::NotFound => response.setCode404(err),
                    Kind::Conflict => response.setCode409(err),
                    Kind::Syntax => response.setCode422(err),
                }
            }
        }
        response
    }
    fn replaceNode(
        &self,
        nodeId: String,
        request: components::CodeRequest::CodeRequest,
    ) -> response::nodes::nodeid::replace::NodesNodeIdReplaceResponse::NodesNodeIdReplaceResponse {
        let mut response = response::nodes::nodeid::replace::NodesNodeIdReplaceResponse::NodesNodeIdReplaceResponse::new();
        let outcome: AstResult<String> = (|| {
            let resolved = self.resolve(request.get_Path(), request.get_Source())?;
            let node = self.find(&resolved.file, &nodeId)?;
            let code = request.get_Code().unwrap_or_default();
            let mut file = resolved.file;
            rust_ast_engine::replace(&mut file, &node, &code)?;
            self.persist(Resolved {
                file,
                path: resolved.path,
            })
        })();
        match outcome {
            Ok(source) => {
                let mut body = components::MutationResponse::MutationResponse::new(
                    serde_json::Value::Null,
                );
                body.set_Source(Some(source));
                response.setCode200(body);
            }
            Err(e) => {
                let err = Self::error_dto(&e);
                match e.kind() {
                    Kind::NotFound => response.setCode404(err),
                    Kind::Conflict => response.setCode409(err),
                    Kind::Syntax => response.setCode422(err),
                }
            }
        }
        response
    }
    fn validateSource(
        &self,
        request: components::SourceRequest::SourceRequest,
    ) -> response::validate::ValidateResponse::ValidateResponse {
        let mut response = response::validate::ValidateResponse::ValidateResponse::new();
        let mut body = response::validate::code200::json::ValidateResponse::ValidateResponse::new(
            serde_json::Value::Null,
        );
        let error = (|| -> AstResult<Option<String>> {
            let source = match request.get_Source() {
                Some(s) => s,
                None => {
                    let p = request
                        .get_Path()
                        .filter(|p| !p.trim().is_empty())
                        .ok_or_else(|| AstEngineException::new(
                            Kind::Syntax,
                            "either 'path' or 'source' is required",
                        ))?;
                    std::fs::read_to_string(Path::new(&p))
                        .map_err(|e| AstEngineException::new(
                            Kind::Syntax,
                            format!("cannot read {}: {}", p, e),
                        ))?
                }
            };
            Ok(rust_ast_engine::validate(&source))
        })();
        match error {
            Ok(Some(msg)) => body.set_Error(Some(msg)),
            Ok(None) => {}
            Err(e) => body.set_Error(Some(e.message().to_string())),
        }
        response.setCode200(body);
        response
    }
}
