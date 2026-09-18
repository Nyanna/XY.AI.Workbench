package xy.ai.workbench.view.session;

import org.eclipse.swt.SWT;
import org.eclipse.swt.layout.GridData;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Display;
import org.eclipse.swt.widgets.Text;
import org.eclipse.ui.forms.widgets.FormToolkit;

import xy.ai.workbench.connector.harness.PromptHandler;

/** Rolling CSV log of token usage per answer. */
public class UsageLogSection {

	public static void create(FormToolkit toolkit, Composite body, PromptHandler prompt, Display display) {
		Composite footer = new Composite(body, SWT.NONE);
		footer.setLayout(new GridLayout(1, false));
		footer.setLayoutData(new GridData(GridData.FILL_HORIZONTAL));

		Text usageLog = toolkit.createText(footer, "", SWT.BORDER | SWT.WRAP | SWT.V_SCROLL);
		usageLog.setText("Total, Out, Reason, CRead, CCreate, In\n");
		GridData gridData = new GridData(SWT.FILL, SWT.FILL, true, true);
		gridData.heightHint = 50;
		usageLog.setLayoutData(gridData);

		prompt.addAnswerObs(a -> {
			display.asyncExec(() -> {
				if (a != null && a.stats.inputToken > 0) {
					String text = usageLog.getText();
					if (text == null || text.isEmpty())
						text = "Total, Out, Reason, CRead, CCreate, In\n";

					String newtext = String.format("%6d,%6d,%5d,%6d,%6d,%6d\n", a.stats.totalToken, a.stats.outputToken,
							a.stats.reasoningToken, a.stats.cacheRead, a.stats.cacheCreate, a.stats.inputToken);
					int idx = text.indexOf('\n');
					text = text.substring(0, idx + 1) + newtext + text.substring(idx + 1);

					usageLog.setText(text);
				}
			});
		});
	}
}
