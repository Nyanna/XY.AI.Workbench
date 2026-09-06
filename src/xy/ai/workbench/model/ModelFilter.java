package xy.ai.workbench.model;

import java.util.regex.Pattern;

/** Filters out non-mainstream model categories (image, video, audio, embedding, moderation, ...). */
public final class ModelFilter {

	private static final Pattern BLACKLIST = Pattern.compile(
			"(?i).*(dall-e|dalle|whisper|tts|speech|audio|embedding|moderation|realtime|transcri|image|video|" //
					+ "guard|safety|computer-use|codex|davinci|babbage|curie|ada-)" + ".*");

	private ModelFilter() {
	}

	public static boolean isMainstream(String modelId) {
		return modelId != null && !BLACKLIST.matcher(modelId).matches();
	}
}
