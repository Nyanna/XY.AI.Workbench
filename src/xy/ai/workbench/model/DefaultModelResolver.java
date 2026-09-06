package xy.ai.workbench.model;

import java.util.Arrays;
import java.util.List;

import xy.ai.workbench.Model;
import xy.ai.workbench.Model.KeyPattern;

/** Simple/dumb resolver: returns the static model list curated in {@link Model#DEFAULTS}. */
public class DefaultModelResolver implements ModelResolver {

	private final KeyPattern provider;

	public DefaultModelResolver(KeyPattern provider) {
		this.provider = provider;
	}

	@Override
	public List<Model> resolve(String apiKey) {
		return Arrays.asList(Model.defaultsFor(provider));
	}
}
