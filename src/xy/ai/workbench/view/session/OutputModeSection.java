package xy.ai.workbench.view.session;

import java.util.Arrays;
import java.util.stream.Collectors;

import org.eclipse.swt.SWT;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Combo;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.forms.widgets.FormToolkit;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.OutputMode;

/** Output mode selector below the system prompt area. */
public class OutputModeSection {

	public static void create(FormToolkit toolkit, Composite body, ConfigManager cfg) {
		Composite bottom = new Composite(body, SWT.NONE);
		bottom.setLayout(new GridLayout(2, false));

		toolkit.createLabel(bottom, "Output:");
		Combo outputMode = new Combo(bottom, SWT.DROP_DOWN | SWT.READ_ONLY);
		String[] outputOptions = Arrays.stream(OutputMode.values()).map(e -> e.name()).collect(Collectors.toList())
				.toArray(new String[0]);
		outputMode.setItems(outputOptions);
		outputMode.addSelectionListener(SelectionListener
				.widgetSelectedAdapter(e -> cfg.setOuputMode(OutputMode.valueOf(outputMode.getText()))));
		cfg.addOutputModeObs(m -> outputMode.setText(m.name()), true);
	}
}
