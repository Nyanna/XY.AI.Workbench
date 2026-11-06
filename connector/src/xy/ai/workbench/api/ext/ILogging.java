package xy.ai.workbench.api.ext;

import java.io.StringWriter;
import java.io.PrintWriter;

public interface ILogging {

	default void info(String message) {
		info(message, null);
	}

	default void info(String message, Throwable throwable) {
		StringWriter sw = new StringWriter();
		throwable.printStackTrace(new PrintWriter(sw));
		System.out.println(message + System.lineSeparator() + sw);
	}

	default void error(String message) {
		error(message, null);
	}

	default void error(String message, Throwable throwable) {
		StringWriter sw = new StringWriter();
		throwable.printStackTrace(new PrintWriter(sw));
		System.err.println(message + System.lineSeparator() + sw);
	}
}
