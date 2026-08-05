package xy.ai.workbench.view;

import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

import org.eclipse.core.resources.IFile;
import org.eclipse.core.resources.IProject;
import org.eclipse.core.resources.IResourceVisitor;
import org.eclipse.core.resources.ResourcesPlugin;
import org.eclipse.core.runtime.CoreException;
import org.eclipse.swt.SWT;
import org.eclipse.swt.widgets.FileDialog;
import org.eclipse.swt.widgets.Shell;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.fasterxml.jackson.dataformat.yaml.YAMLGenerator;

import xy.ai.workbench.LOG;
import xy.ai.workbench.OutputMode;

public class PresetHandler {
	private static final String PROMPT_TXT = ".prompt.txt";

	private static final Pattern FRONT_MATTER = Pattern.compile("\\A---\\s*\\n(.*?\\n)?---\\s*\\n?",
			Pattern.DOTALL);

	private static final ObjectMapper YAML = new ObjectMapper(
			new YAMLFactory().disable(YAMLGenerator.Feature.WRITE_DOC_START_MARKER));

	public static class FrontMatter {
		public List<String> tools;
		public String outputMode;
	}

	public static class Preset {
		public String[] body;
		public String[] tools;
		public OutputMode outputMode;
	}

	public static IFile[] listPresetFiles() {
		List<IFile> files = new ArrayList<>();
		for (IProject project : ResourcesPlugin.getWorkspace().getRoot().getProjects()) {
			try {
				IResourceVisitor visitor = resource -> {
					if (resource instanceof IFile) {
						IFile file = (IFile) resource;
						if (file.getName().endsWith(PROMPT_TXT))
							files.add(file);
					}
					return true;
				};
				project.accept(visitor);
			} catch (CoreException e) {
				LOG.error(e.getMessage(), e);
			}
		}
		files.sort(Comparator.comparing(f -> f.getFullPath().toString()));
		return files.toArray(new IFile[0]);
	}

	public static Preset loadPreset(IFile file) {
		try {
			String content = new String(Files.readAllBytes(file.getLocation().toFile().toPath()),
					StandardCharsets.UTF_8);
			return parsePreset(content);
		} catch (IOException e) {
			throw new IllegalStateException(e);
		}
	}

	public static Preset parsePreset(String content) {
		Preset preset = new Preset();
		String body = content;

		Matcher matcher = FRONT_MATTER.matcher(content);
		if (matcher.find() && matcher.start() == 0) {
			String yaml = matcher.group(1);
			body = content.substring(matcher.end());
			if (yaml != null && !yaml.isBlank()) {
				try {
					FrontMatter fm = YAML.readValue(yaml, FrontMatter.class);
					if (fm.tools != null)
						preset.tools = fm.tools.toArray(new String[0]);
					if (fm.outputMode != null && !fm.outputMode.isBlank())
						preset.outputMode = OutputMode.valueOf(fm.outputMode.trim());
				} catch (Exception e) {
					LOG.error(e.getMessage(), e);
				}
			}
		}

		if (body.endsWith("\n"))
			body = body.substring(0, body.length() - 1);
		preset.body = body.split("\n", -1);
		return preset;
	}

	public static void writePreset(String[] body, String[] tools, OutputMode outputMode, Shell shell) {
		FileDialog dialog = new FileDialog(shell, SWT.SAVE);
		dialog.setFilterPath(ResourcesPlugin.getWorkspace().getRoot().getProjects()[0].getFullPath().toOSString());
		dialog.setFilterExtensions(new String[] { "*.prompt.txt" });
		dialog.setFilterNames(new String[] { "Prompt Files (*prompt.txt)" });
		String filePath = dialog.open();
		if (filePath == null)
			return;

		String content = serializePreset(body, tools, outputMode);
		try (PrintWriter writer = new PrintWriter(new FileWriter(filePath, StandardCharsets.UTF_8))) {
			writer.print(content);
		} catch (IOException e) {
			throw new IllegalStateException(e);
		}
	}

	public static String serializePreset(String[] body, String[] tools, OutputMode outputMode) {
		FrontMatter fm = new FrontMatter();
		if (tools != null && tools.length > 0)
			fm.tools = Arrays.asList(tools);
		if (outputMode != null)
			fm.outputMode = outputMode.name();

		StringBuilder sb = new StringBuilder();
		if (fm.tools != null || fm.outputMode != null) {
			try {
				String yaml = YAML.writeValueAsString(fm);
				sb.append("---\n");
				sb.append(yaml);
				sb.append("---\n");
			} catch (Exception e) {
				LOG.error(e.getMessage(), e);
			}
		}
		sb.append(String.join("\n", body));
		return sb.toString();
	}
}
