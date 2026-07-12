package xy.ai.workbench.editors.spellcheck;

import java.net.URI;
import java.net.URLEncoder;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.charset.StandardCharsets;
import java.time.Duration;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * Sends text to a local LanguageTool server and returns spelling/grammar problems.
 * <p>
 * Markup regions (fenced code blocks, lines starting with '@', Markdown
 * comment lines, inline code, URLs, file paths, @mentions) are masked with
 * spaces before checking, line by line, so their character offsets remain
 * identical to the document offsets returned by LT.
 */
public class LanguageToolClient {

    private static final String ENDPOINT = "http://localhost:8010/v2/check";
    private static final String LANGUAGE = "de-DE";
    private static final String DISABLED_RULES =
            "WHITESPACE_RULE,DOPPELTES_LEERZEICHEN,LEERZEICHEN_VOR_SATZZEICHEN";
    private static final Duration TIMEOUT = Duration.ofSeconds(30);

    // Inline markup within a single line: inline code, URLs, file paths,
    // @mentions. None of these can legitimately contain a line break, so
    // matching is always scoped to one line at a time (see maskText) -
    // that guarantees this regex never runs over text that a block-level
    // rule (fence / '@' line / comment line) has already claimed.
    private static final Pattern MARKUP_RE = Pattern.compile(
            "`[^`\\n]+`|https?://\\S+|/\\S+|@\\S+");
	private static final int LIMIT = 512 * 1024;

    private final HttpClient httpClient = HttpClient.newBuilder()
            .connectTimeout(TIMEOUT)
            .build();
    private final ObjectMapper objectMapper = new ObjectMapper();

    /**
     * Checks the given text and returns a list of problems.
     * Returns an empty list if LT is unreachable or returns no matches.
     *
     * @param text the full document text to check
     */
    public List<SpellingProblem> check(String text) {
        List<SpellingProblem> problems = new ArrayList<>();
        if (text == null || text.isBlank() || text.length() > LIMIT) {
            return problems;
        }

        String masked = maskText(text);
        String responseBody = callLanguageTool(masked);
        if (responseBody == null) {
            return problems;
        }

        try {
            JsonNode root = objectMapper.readTree(responseBody);
            JsonNode matches = root.path("matches");
            if (matches.isArray()) {
                for (JsonNode match : matches) {
                    int offset = match.path("offset").asInt();
                    int length = match.path("length").asInt();
                    String message = match.path("message").asText();
                    if (length > 0) {
                        List<String> suggestions = parseSuggestions(match);
                        problems.add(new SpellingProblem(offset, length, message, suggestions));
                    }
                }
            }
        } catch (Exception e) {
            // ignore parse errors – LT returned unexpected content
        }
        return problems;
    }

    /**
     * Replaces all markup regions with space characters of the same length so
     * that offsets in the returned text match 1-to-1 with the original document.
     */
    private static final int MAX_SUGGESTIONS = 3;

    private List<String> parseSuggestions(JsonNode match) {
        List<String> suggestions = new ArrayList<>();
        JsonNode replacements = match.path("replacements");
        for (int i = 0; i < Math.min(replacements.size(), MAX_SUGGESTIONS); i++) {
            String value = replacements.get(i).path("value").asText();
            if (!value.isEmpty()) {
                suggestions.add(value);
            }
        }
        return suggestions;
    }

    /**
     * Masks markup regions with space characters of the same length so that
     * offsets in the masked text match 1-to-1 with the original document.
     * <p>
     * The whole text is walked exactly once, line by line, tracking a
     * single "am I inside a fenced code block" flag. For each line, block
     * level rules (fence / '@' line / comment line) are checked first and,
     * if any applies, the whole line is blanked. Only lines that are not
     * claimed by a block rule are scanned with {@link #MARKUP_RE} - and
     * only within that single line's bounds. This ordering (blocks before
     * inline regex, one line at a time) makes it structurally impossible
     * for the inline regex to run across an already-masked block boundary,
     * which was the root cause of previous mis-masking around fenced code.
     */
    private String maskText(String text) {
        char[] chars = text.toCharArray();
        boolean inFence = false;
        int lineStart = 0;
        int len = text.length();

        while (lineStart <= len) {
            int newline = text.indexOf('\n', lineStart);
            int lineEnd = (newline == -1) ? len : newline;
            String line = text.substring(lineStart, lineEnd);
            String trimmed = line.stripLeading();
            boolean isFenceLine = trimmed.startsWith("```");

            if (inFence) {
                Arrays.fill(chars, lineStart, lineEnd, ' ');
                if (isFenceLine) {
                    inFence = false;
                }
            } else if (isFenceLine) {
                Arrays.fill(chars, lineStart, lineEnd, ' ');
                inFence = true;
            } else if (trimmed.startsWith("@") || trimmed.startsWith("#:")) {
                // whole line excluded: '@' lines and Markdown comment lines
                Arrays.fill(chars, lineStart, lineEnd, ' ');
            } else {
                // no block rule applies: mask inline markup within this line only
                Matcher markup = MARKUP_RE.matcher(line);
                while (markup.find()) {
                    Arrays.fill(chars, lineStart + markup.start(), lineStart + markup.end(), ' ');
                }
            }

            if (newline == -1) {
                break;
            }
            lineStart = newline + 1;
        }

        return new String(chars);
    }

    private String callLanguageTool(String text) {
        try {
            String body = "language=" + URLEncoder.encode(LANGUAGE, StandardCharsets.UTF_8)
                    + "&disabledRules=" + URLEncoder.encode(DISABLED_RULES, StandardCharsets.UTF_8)
                    + "&text=" + URLEncoder.encode(text, StandardCharsets.UTF_8);

            HttpRequest request = HttpRequest.newBuilder()
                    .uri(URI.create(ENDPOINT))
                    .timeout(TIMEOUT)
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .POST(HttpRequest.BodyPublishers.ofString(body, StandardCharsets.UTF_8))
                    .build();

            HttpResponse<String> response =
                    httpClient.send(request, HttpResponse.BodyHandlers.ofString());

            if (response.statusCode() == 200) {
                return response.body();
            }
        } catch (Exception e) {
            // LT not reachable – fail silently
        }
        return null;
    }
}
