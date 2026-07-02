package xy.ai.workbench;

public enum AgentProfile {
	basic("default"), author, research, code, python, web_research("web-research"), markdown,
	github_research("github-research");

	public final String name;

	AgentProfile() {
		this.name = this.name();
	}

	AgentProfile(String name) {
		this.name = name;
	}
	
	public static AgentProfile fromName(String value) {
		for(var e : values())
			if(e.name.equals(value))
				return e;
		throw new IllegalArgumentException("Not a valid AgentProfile name");
	}
}
