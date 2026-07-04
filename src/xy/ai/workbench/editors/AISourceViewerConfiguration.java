package xy.ai.workbench.editors;

import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.presentation.IPresentationReconciler;
import org.eclipse.jface.text.presentation.PresentationReconciler;
import org.eclipse.jface.text.reconciler.IReconciler;
import org.eclipse.jface.text.rules.DefaultDamagerRepairer;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.SourceViewerConfiguration;
import org.eclipse.swt.graphics.Font;

import xy.ai.workbench.editors.spellcheck.SpellCheckInstaller;

public class AISourceViewerConfiguration extends SourceViewerConfiguration {

	@Override
	public IReconciler getReconciler(ISourceViewer sourceViewer) {
		return SpellCheckInstaller.createReconciler(sourceViewer);
	}

	@Override
	public IPresentationReconciler getPresentationReconciler(ISourceViewer sourceViewer) {
		PresentationReconciler reconciler = new PresentationReconciler();
		Font font = sourceViewer.getTextWidget().getFont();

		DefaultDamagerRepairer dr = new DefaultDamagerRepairer(new AIRuleScanner(font));
		reconciler.setDamager(dr, IDocument.DEFAULT_CONTENT_TYPE);
		reconciler.setRepairer(dr, IDocument.DEFAULT_CONTENT_TYPE);

		return reconciler;
	}
}