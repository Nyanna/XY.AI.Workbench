package xy.ai.workbench.handlers;

import java.io.ByteArrayInputStream;
import java.io.InputStream;

public class NewChatWizard extends AbstractNewFileWizard {
	@Override
	protected String getFileName() {
		return "new_chat.md";
	}

	@Override
	protected InputStream getFileContent() {
		return new ByteArrayInputStream(String.join("\n", //
				"# Chat Heading", //
				"User:", //
				"How...").getBytes());
	}
}
