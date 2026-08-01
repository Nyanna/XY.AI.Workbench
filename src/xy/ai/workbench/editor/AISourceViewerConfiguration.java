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
import org.eclipse.jface.text.rules.DefaultDamagerRepairer;
import org.eclipse.jface.text.rules.ITokenScanner;
import org.eclipse.jface.text.source.ISourceViewer;
import org.eclipse.jface.text.source.SourceViewerConfiguration;

import xy.ai.workbench.editor.spellcheck.SpellingQuickAssistProcessor;

public class AISourceViewerConfiguration extends SourceViewerConfiguration {

	private static final int LIMIT = 2 * 512 * 1024;

	private AIRuleScanner rules = new AIRuleScanner();
	private DefaultDamagerRepairer dmg;

	public AISourceViewerConfiguration(EditorManager updateManager) {
		rules.setUpdateManager(updateManager);
		dmg = new DamagerRepairer(rules, updateManager);
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

		rules.init(sourceViewer.getTextWidget().getFont());
		reconciler.setDamager(dmg, IDocument.DEFAULT_CONTENT_TYPE);
		reconciler.setRepairer(dmg, IDocument.DEFAULT_CONTENT_TYPE);

		return reconciler;
	}

	public DefaultDamagerRepairer getDamager() {
		return dmg;
	}

	private static class DamagerRepairer extends DefaultDamagerRepairer {
		private EditorManager updateManager;

		private DamagerRepairer(ITokenScanner scanner, EditorManager updateManager) {
			super(scanner);
			this.updateManager = updateManager;
		}

		@Override
		public void createPresentation(TextPresentation presentation, ITypedRegion region) {
			if (fDocument != null && fDocument.getLength() > LIMIT)
				addRange(presentation, region.getOffset(), region.getLength(), AIRuleScanner.DEFAULT_ATTR);
			else
				super.createPresentation(presentation, region);
		}

		@Override
		public IRegion getDamageRegion(ITypedRegion partition, DocumentEvent e, boolean documentPartitioningChanged) {
			var node = updateManager.getAst().find(e.getOffset(), e.getOffset() + e.getLength()).getNode();
			// gate fo only update manager changes
			if (node != null && e.getOffset() == node.getOffset() && e.getLength() == node.length())
				return new Region(node.getOffset(), node.length());
			return new Region(0, 0);
		}
	}
}
