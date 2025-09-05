package xy.ai.workbench.tools;

import java.time.Duration;

public class Time {

	public static String secsToReadable(long seconds) {
		if (seconds < 0)
			return "";

		Duration duration = Duration.ofSeconds(seconds);

		long d = duration.toDays();
		long h = duration.toHoursPart();
		long m = duration.toMinutesPart();
		long s = duration.toSecondsPart();

		StringBuilder result = new StringBuilder();

		if (d > 0)
			result.append(d).append("d ");
		if (h > 0)
			result.append(h).append("h ");
		if (m > 0)
			result.append(m).append("m ");
		if (s > 0 || result.length() == 0)
			result.append(s).append("s");

		return result.toString().trim();
	}
}