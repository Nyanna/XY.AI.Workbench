package xy.ai.conscious.autoapprove;

import xy.ai.conscious.api.IActiveEditor.Selection;
import xy.ai.conscious.connector.harness.Prompt;

/**
 * Prompt to run next, described like an operator action: the prompt it derives
 * from and the selection that would have been made in the editor.
 */
public record FollowUp(Prompt origin, Selection selection) {
}
