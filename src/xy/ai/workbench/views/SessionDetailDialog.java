package xy.ai.workbench.views;

import org.eclipse.jface.dialogs.Dialog;
import org.eclipse.jface.dialogs.IDialogConstants;
import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Point;
import org.eclipse.swt.layout.FillLayout;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Control;
import org.eclipse.swt.widgets.Shell;
import org.eclipse.swt.widgets.Text;

import xy.ai.workbench.connectors.claudecode.CCSession;
import xy.ai.workbench.connectors.claudecode.SessionParameters;

public class SessionDetailDialog extends Dialog {
	private final CCSession session;

	protected SessionDetailDialog(Shell parentShell, CCSession session) {
		super(parentShell);
		this.session = session;
		setShellStyle(getShellStyle() | SWT.RESIZE);
	}

	@Override
	protected Control createDialogArea(Composite parent) {
		Composite comp = (Composite) super.createDialogArea(parent);
		comp.setLayout(new FillLayout());

		Text text = new Text(comp, SWT.MULTI | SWT.READ_ONLY | SWT.WRAP | SWT.V_SCROLL | SWT.H_SCROLL | SWT.BORDER);
		text.setText(buildDetailText(session));
		return comp;
	}

	@Override
	protected void createButtonsForButtonBar(Composite parent) {
		createButton(parent, IDialogConstants.OK_ID, IDialogConstants.OK_LABEL, true);
	}

	@Override
	protected void configureShell(Shell newShell) {
		super.configureShell(newShell);
		newShell.setText("Session Details");
	}

	@Override
	protected Point getInitialSize() {
		return new Point(600, 420);
	}

	private static String buildDetailText(CCSession s) {
		SessionParameters p = s.getParameters();
		StringBuilder sb = new StringBuilder();
		sb.append("Session ID: ").append(s.getID()).append("\n");
		sb.append("TTL: ").append(ttl(s)).append("\n");
		sb.append("Model: ").append(p.model != null ? p.model.name() : "").append("\n");
		sb.append("Effort: ").append(p.reasoning != null ? p.reasoning.name() : "").append("\n");
		sb.append("Tools: ").append(p.tools != null ? String.join(", ", p.tools) : "").append("\n");
		sb.append("File: ").append(p.getFilePath() != null ? p.getFilePath() : "").append("\n");
		sb.append("Stats: ").append(s.stats.print()).append("\n");
		sb.append("\nSystemprompt:\n").append(p.systemPrompt != null ? p.systemPrompt : "");
		return sb.toString();
	}

	private static String ttl(CCSession s) {
		long remaining = s.getRemainingTtlMinutes();
		return remaining < 0 ? "—" : remaining + " min";
	}
}