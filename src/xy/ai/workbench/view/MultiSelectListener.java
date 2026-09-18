package xy.ai.workbench.view;

import java.util.ArrayList;
import java.util.Arrays;

import org.eclipse.swt.SWT;
import org.eclipse.swt.widgets.List;

import xy.ai.workbench.view.session.Clearable;

public class MultiSelectListener implements Clearable {
	private ArrayList<Integer> selectedIndices = new ArrayList<>();
	private final List component;

	public MultiSelectListener(List component) {
		this.component = component;
		component.addListener(SWT.MouseDown, event -> {
			int clickedIndex = component.getSelectionIndex();

			if (selectedIndices.contains(clickedIndex))
				selectedIndices.remove(Integer.valueOf(clickedIndex));
			else
				selectedIndices.add(clickedIndex);

			int[] selection = selectedIndices.stream().mapToInt(Integer::intValue).sorted().toArray();
			component.setSelection(selection);
		});
	}

	public void setSelection(String[] items) {
		java.util.List<String> all = Arrays.asList(component.getItems());
		selectedIndices.clear();
		if (items != null)
			for (String item : items) {
				int idx = all.indexOf(item);
				if (idx >= 0)
					selectedIndices.add(idx);
			}
		int[] selection = selectedIndices.stream().mapToInt(Integer::intValue).sorted().toArray();
		component.setSelection(selection);
	}

	@Override
	public void clear() {
		selectedIndices.clear();
		component.deselectAll();
	}
}