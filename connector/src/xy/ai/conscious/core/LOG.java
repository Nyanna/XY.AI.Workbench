package xy.ai.conscious.core;

import xy.ai.conscious.api.ext.ILogging;

public class LOG {
	public static ILogging log = new ILogging() {
	};

	public static void info(String message) {
		log.info(message);
	}

	public static void info(String message, Throwable throwable) {
		log.info(message, throwable);
	}

	public static void error(String message) {
		log.error(message);
	}

	public static void error(String message, Throwable throwable) {
		log.error(message, throwable);
	}
}
