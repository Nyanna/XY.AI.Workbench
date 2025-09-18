package xy.ai.workbench.handlers;

import java.io.ByteArrayInputStream;
import java.io.InputStream;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

public class NewChatWizard extends AbstractNewFileWizard {
	@Override
	protected String getFileName() {
		return "chat" + LocalDateTime.now().format(DateTimeFormatter.ofPattern("yyyyMMdd.HHmmss")) + ".md";
	}

	@Override
	protected InputStream getFileContent() {
		return new ByteArrayInputStream(String.join("\n", //
				"# Chat Heading", //
				"User:", //
				"How...").getBytes());
	}
}
