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
 * Markup regions (fenced code blocks, inline code, URLs, file paths, @mentions)
 * and lines starting with '@' are masked with spaces before checking, so their
 * character offsets remain identical to the document offsets returned by LT.
 */
public class LanguageToolClient {

    private static final String ENDPOINT = "http://localhost:8010/v2/check";
    private static final String LANGUAGE = "de-DE";
    private static final String DISABLED_RULES =
            "WHITESPACE_RULE,DOPPELTES_LEERZEICHEN,LEERZEICHEN_VOR_SATZZEICHEN";
    private static final Duration TIMEOUT = Duration.ofSeconds(30);

    // Lines whose first non-space character is '@' are excluded entirely.
    private static final Pattern AT_LINE_RE =
            Pattern.compile("(?m)^[ \\t]*@[^\\n]*");
    private static final Pattern COMMENT_LINE_RE =
            Pattern.compile("(?m)^[ \\t]*#:[^\\n]*");

    // Inline markup regions: fenced code, inline code, URLs, file paths, @mentions.
    private static final Pattern MARKUP_RE = Pattern.compile(
            "```[\\s\\S]*?```|`[^`]+`|https?://\\S+|/\\S+|@\\S+");

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
        if (text == null || text.isBlank()) {
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

    private String maskText(String text) {
        char[] chars = text.toCharArray();

        // 1. Mask entire lines starting with '@'
        Matcher atLine = AT_LINE_RE.matcher(text);
        while (atLine.find()) {
            Arrays.fill(chars, atLine.start(), atLine.end(), ' ');
        }

        // 2. ignore Markdown comment lines
        Matcher commentLine = COMMENT_LINE_RE.matcher(text);
        while (commentLine.find()) {
            Arrays.fill(chars, commentLine.start(), commentLine.end(), ' ');
        }

        // 3. Mask inline markup (re-run on original text so already-masked
        //    regions don't interfere with regex matching)
        Matcher markup = MARKUP_RE.matcher(text);
        while (markup.find()) {
            Arrays.fill(chars, markup.start(), markup.end(), ' ');
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
