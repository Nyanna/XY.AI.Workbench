package xy.ai.workbench.mdast.nodes;

public enum Category {
	Section, // can contains blocks and inline elements
	Block, // terminal don't contains children
	Inline // inline elements witthing a section
}
