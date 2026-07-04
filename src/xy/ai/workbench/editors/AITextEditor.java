package xy.ai.workbench.editors;

import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.IVerticalRuler;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.ui.editors.text.TextEditor;

import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;

public class AITextEditor extends TextEditor {
	public AITextEditor() {
		super();
		setSourceViewerConfiguration(new AISourceViewerConfiguration());
	}

	@Override
	protected ISourceViewer createSourceViewer(Composite parent, IVerticalRuler ruler, int styles) {
		ISourceViewer sourceViewer = super.createSourceViewer(parent, ruler, styles);
		return sourceViewer;
	}

	@Override
	public void createPartControl(Composite parent) {
		super.createPartControl(parent);
		SpellCheckInstaller.installPainter(getSourceViewer());
	}

	@Override
	protected boolean getInitialWordWrapStatus() {
		return true;
	}
}