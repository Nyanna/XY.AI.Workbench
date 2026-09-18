package xy.ai.workbench.view.session;

import org.eclipse.swt.SWT;
import org.eclipse.swt.events.SelectionListener;
import org.eclipse.swt.layout.GridLayout;
import org.eclipse.swt.widgets.Button;
import org.eclipse.swt.widgets.Composite;

import xy.ai.workbench.ConfigManager;
import xy.ai.workbench.connector.harness.PromptHandler;

/** Prompt/Enqueue/Batch action buttons. */
public class ActionButtonsSection {

	public static void create(Composite body, ConfigManager cfg, PromptHandler prompt) {
		Composite actions = new Composite(body, SWT.NONE);
		actions.setLayout(new GridLayout(3, false));

		Button btn = new Button(actions, SWT.PUSH);
		btn.setText("Prompt");
		btn.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> prompt.execute(btn.getDisplay())));

		Button bbtn = new Button(actions, SWT.PUSH);
		bbtn.setText("Enqueue");
		bbtn.addSelectionListener(SelectionListener.widgetSelectedAdapter(e -> prompt.queueAsync(bbtn.getDisplay())));

		Button bsbtn = new Button(actions, SWT.PUSH);
		bsbtn.setText("Batch");
		bsbtn.addSelectionListener(
				SelectionListener.widgetSelectedAdapter(e -> prompt.queueAndSubmit(bsbtn.getDisplay())));

		cfg.addModelObs(m -> {
			boolean hasBatch = m.cap.isSupportBatch();
			bbtn.setEnabled(hasBatch);
			bsbtn.setEnabled(hasBatch);
			body.layout();
		}, true);
	}
}
