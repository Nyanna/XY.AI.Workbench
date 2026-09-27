package xy.ai.workbench.layout;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import jakarta.inject.Inject;

import org.eclipse.e4.core.di.annotations.Optional;
import org.eclipse.e4.ui.di.UIEventTopic;
import org.eclipse.e4.ui.model.application.ui.MElementContainer;
import org.eclipse.e4.ui.model.application.ui.MUIElement;
import org.eclipse.e4.ui.model.application.ui.basic.MPartSashContainer;
import org.eclipse.e4.ui.model.application.ui.basic.MPartSashContainerElement;
import org.eclipse.e4.ui.model.application.ui.basic.MWindow;
import org.eclipse.e4.ui.workbench.UIEvents;
import org.eclipse.e4.ui.workbench.modeling.EModelService;
import org.eclipse.swt.events.ControlAdapter;
import org.eclipse.swt.events.ControlEvent;
import org.eclipse.swt.widgets.Composite;
import org.eclipse.swt.widgets.Control;
import org.eclipse.swt.widgets.Shell;
import org.eclipse.ui.IPageLayout;
import org.eclipse.swt.graphics.Rectangle;
import org.osgi.service.event.Event;

import java.util.HashSet;
import java.util.Set;

public class FixedWidthPanelAddon {
	private static final List<String> FIXED_ELEMENT_IDS = List.of("org.eclipse.ui.navigator.ProjectExplorer",
			"xy.ai.workbench.view.AISessionView");
	private static final int SASH_WIDTH = 5;

	private final Map<String, Integer> fixedSizeCache = new HashMap<>();
	private volatile boolean adjusting = false;

	@Inject
	private EModelService modelService;
	private final Set<MWindow> attachedWindows = new HashSet<>();

	@Inject
	@Optional
	private void handleWidgetSet(@UIEventTopic(UIEvents.UIElement.TOPIC_WIDGET) Event event) {
		Object element = event.getProperty(UIEvents.EventTags.ELEMENT);
		if (!(element instanceof MWindow window) || attachedWindows.contains(window))
			return;
		if (!(window.getWidget() instanceof Shell shell) || shell.isDisposed())
			return;

		attachedWindows.add(window);
		shell.addControlListener(new ControlAdapter() {
			@Override
			public void controlResized(ControlEvent e) {
				handleResize(window);
			}
		});
		handleResize(window);
	}

	private void handleResize(MWindow window) {
		if (adjusting)
			return;
		for (String elementId : FIXED_ELEMENT_IDS) {
			adjustSashLevel(window, elementId, true);
			adjustSashLevel(window, elementId, false);
		}
	}

	private void adjustSashLevel(MWindow window, String fixedElementId, boolean widthAxis) {
		MUIElement fixedElement = modelService.find(fixedElementId, window);
		MUIElement editorArea = modelService.find(IPageLayout.ID_EDITOR_AREA, window);
		if (fixedElement == null || editorArea == null)
			return;

		MPartSashContainerElement[] sashChildren = findCommonSashChildren(fixedElement, editorArea);
		if (sashChildren == null)
			return;
		MPartSashContainerElement fixedSashChild = sashChildren[0];
		MPartSashContainerElement growingSashChild = sashChildren[1];
		MElementContainer<?> commonParent = fixedSashChild.getParent();
		MPartSashContainer sashContainer = (MPartSashContainer) commonParent;

		if (sashContainer.isHorizontal() != widthAxis)
			return; // wrong axis

		Object widget = sashContainer.getWidget();
		int total;
		if (widget instanceof Rectangle r) {
			total = widthAxis ? r.width : r.height;
		} else if (widget instanceof Composite composite && !composite.isDisposed())
			total = widthAxis ? composite.getClientArea().width : composite.getClientArea().height;
		else
			return;
		if (total <= 0)
			return;

		Integer cached = fixedSizeCache.get(fixedElementId);
		int fixedSize = cached != null ? cached : cacheFixedSize(fixedElementId, fixedSashChild, widthAxis);
		int sashSpace = SASH_WIDTH * Math.max(0, countVisibleChildren(sashContainer) - 1);
		int growingSize = Math.max(0, total - fixedSize - sashSpace);

		adjusting = true;
		try {
			fixedSashChild.setContainerData(Integer.toString(fixedSize));
			growingSashChild.setContainerData(Integer.toString(growingSize));
		} finally {
			adjusting = false;
		}
	}

	private int cacheFixedSize(String elementId, MPartSashContainerElement fixedSashChild, boolean widthAxis) {
		Object widget = fixedSashChild.getWidget();
		int size;
		if (widget instanceof Control control) {
			if (control.isDisposed())
				return 0;
			size = widthAxis ? control.getBounds().width : control.getBounds().height;
		} else if (widget instanceof Rectangle bounds)
			size = widthAxis ? bounds.width : bounds.height;
		else
			return 0;
		if (size <= 0)
			return 0;
		fixedSizeCache.put(elementId, size);
		return size;
	}

	private int countVisibleChildren(MPartSashContainer sashContainer) {
		int count = 0;
		for (MPartSashContainerElement child : sashContainer.getChildren())
			if (child.isVisible())
				count++;
		return count;
	}

	private MPartSashContainerElement[] findCommonSashChildren(MUIElement a, MUIElement b) {
		List<MUIElement> chainA = buildAncestorChain(a);
		List<MUIElement> chainB = buildAncestorChain(b);
		for (MUIElement ancestor : chainA) {
			if (!(ancestor instanceof MPartSashContainer))
				continue;
			int idxB = chainB.indexOf(ancestor);
			if (idxB < 0)
				continue;
			int idxA = chainA.indexOf(ancestor);
			MUIElement childA = idxA > 0 ? chainA.get(idxA - 1) : a;
			MUIElement childB = idxB > 0 ? chainB.get(idxB - 1) : b;
			if (childA instanceof MPartSashContainerElement sashChildA
					&& childB instanceof MPartSashContainerElement sashChildB)
				return new MPartSashContainerElement[] { sashChildA, sashChildB };
		}
		return null;
	}

	private List<MUIElement> buildAncestorChain(MUIElement element) {
		List<MUIElement> chain = new ArrayList<>();
		MUIElement current = element;
		while (current != null) {
			chain.add(current);
			current = current.getParent();
		}
		return chain;
	}
}
