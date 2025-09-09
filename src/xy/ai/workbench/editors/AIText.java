package xy.ai.workbench.editors;

import org.eclipse.jface.text.TextAttribute;
import org.eclipse.swt.SWT;
import org.eclipse.swt.graphics.Color;
import org.eclipse.swt.graphics.RGB;
import org.eclipse.swt.widgets.Display;

public class AIText {
	public static final TextAttribute GRAY_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_BLACK),
			new Color(Display.getCurrent(), new RGB(230, 230, 230)), SWT.NONE);

	public static final TextAttribute BLUE_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_WHITE),
			new Color(Display.getCurrent(), new RGB(200, 200, 255)), SWT.NONE);

	public static final TextAttribute DEFAULT_ATTR = new TextAttribute(
			Display.getCurrent().getSystemColor(SWT.COLOR_WIDGET_FOREGROUND), null, SWT.NONE);
}