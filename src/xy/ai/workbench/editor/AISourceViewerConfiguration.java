package xy.ai.workbench.editor;

import org.eclipse.jface.text.DocumentEvent;
import org.eclipse.jface.text.IDocument;
import org.eclipse.jface.text.IRegion;
import org.eclipse.jface.text.ITypedRegion;
import org.eclipse.jface.text.Region;
import org.eclipse.jface.text.TextPresentation;
import org.eclipse.jface.text.presentation.IPresentationReconciler;
import org.eclipse.jface.text.presentation.PresentationReconciler;
import org.eclipse.jface.text.quickassist.IQuickAssistAssistant;
import org.eclipse.jface.text.quickassist.QuickAssistAssistant;
import org.eclipse.jface.text.reconciler.IReconciler;
import org.eclipse.jface.text.rules.DefaultDamagerRepairer;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.SourceViewerConfiguration;
import org.eclipse.swt.graphics.Font;

import xy.ai.workbench.editor.spellcheck.SpellCheckReconciler;
import xy.ai.workbench.editor.spellcheck.SpellingQuickAssistProcessor;
import xy.ai.workbench.editor.update.EditorManager;

public class AISourceViewerConfiguration extends SourceViewerConfiguration {
	private static final int LIMIT = 2 * 512 * 1024;

	private final EditorManager updateManager;


	public AISourceViewerConfiguration(EditorManager updateManager) {
		this.updateManager = updateManager;
	}

	@Override
	public IReconciler getReconciler(ISourceViewer sourceViewer) {
		return new SpellCheckReconciler(sourceViewer, updateManager);
	}

	@Override
	public IQuickAssistAssistant getQuickAssistAssistant(ISourceViewer sourceViewer) {
		QuickAssistAssistant assistant = new QuickAssistAssistant();
		assistant.setQuickAssistProcessor(new SpellingQuickAssistProcessor());
		return assistant;
	}

	@Override
	public IPresentationReconciler getPresentationReconciler(ISourceViewer sourceViewer) {
		PresentationReconciler reconciler = new PresentationReconciler();
		Font font = sourceViewer.getTextWidget().getFont();

		DefaultDamagerRepairer dr = new DefaultDamagerRepairer(new AIRuleScanner(font, updateManager)) {
			@Override
			public void createPresentation(TextPresentation presentation, ITypedRegion region) {
				if (fDocument != null && fDocument.getLength() > LIMIT) {
					addRange(presentation, region.getOffset(), region.getLength(), AIRuleScanner.DEFAULT_ATTR);
					return;
				}
				super.createPresentation(presentation, region);
			}

			@Override
			public IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e,
					boolean documentPartitioningChanged) {
				IDocument document = sourceViewer.getDocument();
				if (document == null)
					return partition;
				if (document.getLength() > LIMIT)
					return new Region(0, 1);

				// No AST-based damage region here anymore: the AST reparse (and thus the
				// precise, authoritative changed region) is debounced centrally in
				// EditorManager, which actively pushes a repaint for that region via
				// invalidateTextPresentation() once it is available. This default,
				// per-edit damage computation only provides transient, best-effort
				// highlighting in between - falling behind briefly is acceptable.
				return super.getDamageRegion(partition, e, documentPartitioningChanged);
			}
		};
		reconciler.setDamager(dr, IDocument.DEFAULT_CONTENT_TYPE);
		reconciler.setRepairer(dr, IDocument.DEFAULT_CONTENT_TYPE);

		return reconciler;
	}
}
