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

import xy.ai.workbench.editor.mdast.TextRegion;
import xy.ai.workbench.editor.spellcheck.SpellCheckInstaller;
import xy.ai.workbench.editor.spellcheck.SpellingQuickAssistProcessor;

public class AISourceViewerConfiguration extends SourceViewerConfiguration {
	private static final int LIMIT = 2 * 512 * 1024;

	private final AITextEditor editor;

	public AISourceViewerConfiguration(AITextEditor editor) {
		this.editor = editor;
	}

	@Override
	public IReconciler getReconciler(ISourceViewer sourceViewer) {
		return SpellCheckInstaller.createReconciler(sourceViewer, editor);
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

		DefaultDamagerRepairer dr = new DefaultDamagerRepairer(new AIRuleScanner(font, editor)) {
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

				TextRegion astRegion = editor != null ? editor.getLastAstChangeRegion() : null;
				if (astRegion != null) {
					int offset = Math.max(0, Math.min(astRegion.offset(), document.getLength()));
					int end = Math.max(offset, Math.min(astRegion.offset() + astRegion.length(),
							document.getLength()));
					return new Region(offset, end - offset);
				}

				return super.getDamageRegion(partition, e, documentPartitioningChanged);
			}
		};
		reconciler.setDamager(dr, IDocument.DEFAULT_CONTENT_TYPE);
		reconciler.setRepairer(dr, IDocument.DEFAULT_CONTENT_TYPE);

		return reconciler;
	}
}
