package xy.ai.workbench.model;

import java.util.List;

import xy.ai.workbench.Model;

/**
 * Pluggable model resolution. One resolver instance is responsible for a single provider
 * (identified by {@link Model.KeyPattern}) and resolves the models available for a given API key.
 * Implementations must not throw: HTTP/parsing errors are caught, logged and a reasonable
 * (possibly default) result is returned instead.
 */
public interface ModelResolver {

	/** Resolves the models available for the given (already key-pattern-matched) API key. */
	List<Model> resolve(String apiKey);
}
