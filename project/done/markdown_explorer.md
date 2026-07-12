Der Markdown Explorer in `/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/MarkdownContentProvider.java` muss die ".gitignore" Dateien beachten und darf keine Markdown Dateien aus ignorierten Verzeichnissen anzeigen.
#: Control Request:
#: {
#:   "id" : "7d95ab1d-8fb5-4b59-a439-2be25aabe31b",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/MarkdownContentProvider.java"
#:   }
#: }
/allow 7d95ab1d-8fb5-4b59-a439-2be25aabe31b
#: Control Request:
#: {
#:   "id" : "0235a9aa-f251-4088-8032-542f1b7a87e3",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.views.explorer;\nimport xy.ai.workbench.views.FlatEndingContentProvider;\n\npublic class MarkdownContentProvider extends FlatEndingContentProvider{\n\tpublic MarkdownContentProvider() {\n\t\tsuper(\".md\");\n\t}\n}\n"
#:     }
#:   }
#: }
/allow 0235a9aa-f251-4088-8032-542f1b7a87e3
#: Control Request:
#: {
#:   "id" : "de646658-69cf-4595-99d0-eb0b5bc44677",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/FlatEndingContentProvider.java"
#:   }
#: }
/allow de646658-69cf-4595-99d0-eb0b5bc44677
#: Control Request:
#: {
#:   "id" : "08e29504-9470-40de-87b7-99552def5f3a",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.views;\n\nimport java.util.ArrayList;\nimport java.util.List;\nimport java.util.Set;\nimport java.util.TreeSet;\n\nimport org.eclipse.core.resources.IContainer;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IProject;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.resources.IResourceVisitor;\nimport org.eclipse.core.resources.IWorkspaceRoot;\nimport org.eclipse.core.runtime.CoreException;\nimport org.eclipse.jface.viewers.ITreeContentProvider;\n\npublic class FlatEndingContentProvider implements ITreeContentProvider {\n\tprivate String ending;\n\n\tpublic FlatEndingContentProvider(String ending) {\n\t\tthis.ending = ending;\n\t}\n\n\t@Override\n\tpublic Object[] getElements(Object root) {\n\t\tObject inputElement = ((IWorkspaceRoot) root).getProjects();\n\t\tSet<IContainer> dirs = new TreeSet<>((a, b) -> a.getName().compareTo(b.getName()));\n\t\tfor (var proj : (Object[]) inputElement)\n\t\t\ttry {\n\t\t\t\tIResourceVisitor visitor = resource -> {\n\t\t\t\t\tif (resource instanceof IFile) {\n\t\t\t\t\t\tIFile file = (IFile) resource;\n\t\t\t\t\t\tif (file.getName().endsWith(ending))\n\t\t\t\t\t\t\tdirs.add(file.getParent());\n\t\t\t\t\t}\n\t\t\t\t\treturn true;\n\t\t\t\t};\n\t\t\t\t((IProject) proj).accept(visitor);\n\t\t\t} catch (CoreException e) {\n\t\t\t}\n\t\treturn dirs.toArray();\n\t}\n\n\t@Override\n\tpublic Object[] getChildren(Object parentElement) {\n\t\tif (parentElement instanceof IContainer) {\n\t\t\tIContainer parent = (IContainer) parentElement;\n\t\t\tList<IFile> files = new ArrayList<>();\n\t\t\ttry {\n\t\t\t\tIResourceVisitor visitor = resource -> {\n\t\t\t\t\tif (resource instanceof IFile) {\n\t\t\t\t\t\tIFile file = (IFile) resource;\n\t\t\t\t\t\tif (file.getName().endsWith(ending))\n\t\t\t\t\t\t\tfiles.add(file);\n\t\t\t\t\t}\n\t\t\t\t\treturn true;\n\t\t\t\t};\n\t\t\t\tparent.accept(visitor, 1, IResource.NONE);\n\t\t\t} catch (CoreException e) {\n\t\t\t}\n\t\t\treturn files.toArray();\n\t\t}\n\t\treturn new Object[0];\n\t}\n\n\t@Override\n\tpublic boolean hasChildren(Object element) {\n\t\treturn getChildren(element).length > 0;\n\t}\n\n\t@Override\n\tpublic Object getParent(Object element) {\n\t\tif (element instanceof IResource)\n\t\t\treturn ((IResource) element).getParent();\n\t\treturn null;\n\t}\n}"
#:     }
#:   }
#: }
/allow 08e29504-9470-40de-87b7-99552def5f3a
#: Control Request:
#: {
#:   "id" : "3b87c0a6-c0e4-47fb-92ee-71b117f34611",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench"
#:   }
#: }
/allow 3b87c0a6-c0e4-47fb-92ee-71b117f34611
#: Control Request:
#: {
#:   "id" : "d1b483ec-135a-4511-a840-8c5ec1e0d1d4",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "error" : "Not a regular file: /home/user/xyan/xy.ai.workbench"
#:     },
#:     "isError" : true
#:   }
#: }
/allow d1b483ec-135a-4511-a840-8c5ec1e0d1d4
#: Control Request:
#: {
#:   "id" : "f69a7a50-dab3-4f5c-a182-fdba16ef7109",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer"
#:   }
#: }
/allow f69a7a50-dab3-4f5c-a182-fdba16ef7109
#: Control Request:
#: {
#:   "id" : "c84d76cc-41a7-44de-be73-37f3b4ccc75e",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "error" : "Not a regular file: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer"
#:     },
#:     "isError" : true
#:   }
#: }
/allow c84d76cc-41a7-44de-be73-37f3b4ccc75e
#: Control Request:
 {
   "id" : "5960f9c7-7ad9-436f-96db-67b756f264d2",
   "phase" : "request",
   "toolName" : "read",
   "arguments" : {
     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/MarkdownExplorerView.java"
   }
 }
/allow 5960f9c7-7ad9-436f-96db-67b756f264d2
#: Control Request:
#: {
#:   "id" : "c2316dbc-b3bc-40ce-b3c9-1f0b419425f5",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.views.explorer;\n\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IResourceChangeEvent;\nimport org.eclipse.core.resources.IResourceChangeListener;\nimport org.eclipse.core.resources.ResourcesPlugin;\nimport org.eclipse.jface.viewers.IOpenListener;\nimport org.eclipse.jface.viewers.ISelection;\nimport org.eclipse.jface.viewers.IStructuredSelection;\nimport org.eclipse.jface.viewers.OpenEvent;\nimport org.eclipse.jface.viewers.TreeViewer;\nimport org.eclipse.swt.widgets.Composite;\nimport org.eclipse.swt.widgets.Display;\nimport org.eclipse.ui.IWorkbenchPage;\nimport org.eclipse.ui.PartInitException;\nimport org.eclipse.ui.PlatformUI;\nimport org.eclipse.ui.ide.IDE;\nimport org.eclipse.ui.navigator.CommonNavigator;\nimport org.eclipse.ui.navigator.CommonViewer;\n\nimport xy.ai.workbench.LOG;\n\npublic class MarkdownExplorerView extends CommonNavigator {\n\tprivate IResourceChangeListener resourceChangeListener;\n\n\t@Override\n\tpublic void createPartControl(Composite aParent) {\n\t\tsuper.createPartControl(aParent);\n\t\tCommonViewer viewer = getCommonViewer();\n\t\tviewer.setInput(ResourcesPlugin.getWorkspace().getRoot());\n\t\tviewer.setComparator(new ModificationDateComparator());\n\n\t\tresourceChangeListener = new IResourceChangeListener() {\n\t\t\t@Override\n\t\t\tpublic void resourceChanged(IResourceChangeEvent event) {\n\t\t\t\tDisplay.getDefault().asyncExec(() -> {\n\t\t\t\t\tif (getCommonViewer() != null && !getCommonViewer().getControl().isDisposed())\n\t\t\t\t\t\tgetCommonViewer().refresh();\n\t\t\t\t});\n\t\t\t}\n\t\t};\n\n\t\tResourcesPlugin.getWorkspace().addResourceChangeListener(resourceChangeListener,\n\t\t\t\tIResourceChangeEvent.POST_CHANGE);\n\t}\n\n\t@Override\n\tprotected void initListeners(TreeViewer viewer) {\n\t\tsuper.initListeners(viewer);\n\n\t\tviewer.addOpenListener(new IOpenListener() {\n\t\t\t@Override\n\t\t\tpublic void open(OpenEvent event) {\n\t\t\t\tISelection selection = event.getSelection();\n\t\t\t\tif (selection instanceof IStructuredSelection) {\n\t\t\t\t\tObject element = ((IStructuredSelection) selection).getFirstElement();\n\t\t\t\t\tif (element instanceof IFile)\n\t\t\t\t\t\topenFile((IFile) element);\n\t\t\t\t}\n\t\t\t}\n\t\t});\n\t}\n\n\tprivate void openFile(IFile file) {\n\t\tIWorkbenchPage page = PlatformUI.getWorkbench().getActiveWorkbenchWindow().getActivePage();\n\t\ttry {\n\t\t\tIDE.openEditor(page, file);\n\t\t} catch (PartInitException e) {\n\t\t\tLOG.error(e.getMessage(), e);\n\t\t}\n\t}\n\n\t@Override\n\tpublic void dispose() {\n\t\tif (resourceChangeListener != null)\n\t\t\tResourcesPlugin.getWorkspace().removeResourceChangeListener(resourceChangeListener);\n\t\tsuper.dispose();\n\t}\n}"
#:     }
#:   }
#: }
/allow c2316dbc-b3bc-40ce-b3c9-1f0b419425f5
#: Control Request:
#: {
#:   "id" : "af66f461-7996-4682-8ee4-b59bfa7e1414",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/GitIgnoreUtil.java"
#:   }
#: }
/allow af66f461-7996-4682-8ee4-b59bfa7e1414
#: Control Request:
#: {
#:   "id" : "27d327c4-ed94-4f22-9182-5291c907efba",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "error" : "File not found: /home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/GitIgnoreUtil.java"
#:     },
#:     "isError" : true
#:   }
#: }
/allow 27d327c4-ed94-4f22-9182-5291c907efba
#: Control Request:
#: {
#:   "id" : "7b26143e-ea85-4707-8615-3b6a1718ee04",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/META-INF/MANIFEST.MF"
#:   }
#: }
/allow 7b26143e-ea85-4707-8615-3b6a1718ee04
#: Control Request:
#: {
#:   "id" : "1f68dc5b-cb30-48ce-ba84-198ccea58938",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "Manifest-Version: 1.0\nBundle-ManifestVersion: 2\nBundle-Name: XY.AI.Workbench\nBundle-SymbolicName: XY.AI.Workbench;singleton:=true\nBundle-Version: 250918.02\nImport-Package: jakarta.inject;version=\"[2.0.0,3.0.0)\",\n org.eclipse.search.ui,\n org.eclipse.search.ui.text,\n org.eclipse.ui.forms.widgets,\n org.eclipse.ui.navigator,\n org.eclipse.ui.navigator.resources\nBundle-Activator: xy.ai.workbench.Activator\nBundle-ClassPath: libs/openai/httpclient5-5.5.jar,\n libs/openai/httpcore5-5.3.5.jar,\n libs/openai/jackson-annotations-3.0-rc5.jar,\n libs/openai/jackson-core-2.20.0.jar,\n libs/openai/jackson-databind-2.20.0.jar,\n libs/openai/jackson-datatype-jdk8-2.20.0.jar,\n libs/openai/jackson-datatype-jsr310-2.20.0.jar,\n libs/openai/jackson-module-kotlin-2.20.0.jar,\n libs/openai/jackson-module-parameter-names-2.20.0.jar,\n libs/openai/kotlin-reflect-2.2.10.jar,\n libs/openai/kotlin-stdlib-2.2.10.jar,\n libs/openai/kotlin-stdlib-jdk8-2.2.10.jar,\n libs/openai/logging-interceptor-5.1.0.jar,\n libs/openai/okhttp-5.1.0.jar,\n libs/openai/okhttp-jvm-5.1.0.jar,\n libs/openai/okio-3.16.0.jar,\n libs/openai/okio-jvm-3.16.0.jar,\n .,\n libs/openai/slf4j-api-2.0.17.jar,\n libs/google/api-common-2.53.0.jar,\n libs/google/auto-value-1.11.0.jar,\n libs/google/error_prone_annotations-2.41.0.jar,\n libs/google/google-auth-library-credentials-1.38.0.jar,\n libs/google/google-auth-library-oauth2-http-1.38.0.jar,\n libs/google/google-genai-1.15.0.jar,\n libs/google/google-http-client-2.0.0.jar,\n libs/google/google-http-client-gson-2.0.0.jar,\n libs/google/gson-2.13.1.jar,\n libs/google/guava-33.4.8-jre.jar,\n libs/anthropic/anthropic-java-2.6.0.jar,\n libs/anthropic/anthropic-java-client-okhttp-2.6.0.jar,\n libs/anthropic/anthropic-java-core-2.6.0.jar,\n libs/google/auto-value-annotations-1.11.0.jar,\n libs/openai/openai-java-3.4.1.jar,\n libs/openai/openai-java-client-okhttp-3.4.1.jar,\n libs/openai/openai-java-core-3.4.1.jar\nBundle-Vendor: Xyan Kruse\nRequire-Bundle: org.eclipse.ui,\n org.eclipse.core.runtime,\n org.eclipse.jface.text,\n org.eclipse.core.resources,\n org.eclipse.ui.editors,\n org.eclipse.ui.ide,\n org.eclipse.ui.navigator;bundle-version=\"3.13.100\"\nBundle-RequiredExecutionEnvironment: JavaSE-21\nAutomatic-Module-Name: XY.AI.Workbench\nBundle-ActivationPolicy: lazy\n"
#:     }
#:   }
#: }
/allow 1f68dc5b-cb30-48ce-ba84-198ccea58938
#: Control Request:
#: {
#:   "id" : "fb147044-66e9-4856-b28c-bb2d96a886b4",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/ModificationDateComparator.java"
#:   }
#: }
/allow fb147044-66e9-4856-b28c-bb2d96a886b4
#: Control Request:
#: {
#:   "id" : "7e3567f0-8dcd-4818-a212-e9e7c275c339",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.views.explorer;\n\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.jface.viewers.*;\n\npublic class ModificationDateComparator extends ViewerComparator {\n\n\t@Override\n\tpublic int compare(Viewer viewer, Object e1, Object e2) {\n\t\tif (!(e1 instanceof IResource) || !(e2 instanceof IResource))\n\t\t\treturn super.compare(viewer, e1, e2);\n\n\t\tIResource r1 = (IResource) e1;\n\t\tIResource r2 = (IResource) e2;\n\n\t\tif (r1.getType() != r2.getType())\n\t\t\treturn Integer.compare(r1.getType(), r2.getType());\n\n\t\treturn Long.compare(r2.getLocalTimeStamp(), r1.getLocalTimeStamp());\n\t}\n}"
#:     }
#:   }
#: }
/allow 7e3567f0-8dcd-4818-a212-e9e7c275c339
#: Control Request:
#: {
#:   "id" : "319e9cf7-f3f3-474a-a3d5-6a8af874d465",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/plugin.xml"
#:   }
#: }
/allow 319e9cf7-f3f3-474a-a3d5-6a8af874d465
#: Control Request:
#: {
#:   "id" : "73cd0f91-ea33-4763-84a8-fc4cca24a929",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<?eclipse version=\"3.4\"?>\n<plugin>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.editors\">\n\t\t<editor\n\t\t\tclass=\"xy.ai.workbench.editors.AISessionEditor\"\n\t\t\tcontributorClass=\"xy.ai.workbench.editors.AISessionEditorContributor\"\n\t\t\textensions=\"md\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.editors.AISessionEditor\"\n\t\t\tname=\"XY.AI Editor\"\n\t\t\tsymbolicFontName=\"org.eclipse.jface.bannerfont\">\n\t\t</editor>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.help.contexts\">\n\t\t<contexts\n\t\t\tfile=\"contexts.xml\">\n\t\t</contexts>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.views\">\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench\"\n\t\t\tname=\"AI\">\n\t\t</category>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.views.AISessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.views.AISessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.Ai Session Settings\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.views.AIBatchView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.views.AIBatchView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"XY.AI Batch List\">\n\t\t</view>\n\t\t<view\n\t\t\tname=\"AI Explorer\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.views.explorer.MarkdownExplorerView\"\n\t\t\tid=\"xy.ai.workbench.views.AIExplorer\">\n\t\t</view>\n\t\t<view\n\t\t\tallowMultiple=\"false\"\n\t\t\tcategory=\"XY.AI.Workbench\"\n\t\t\tclass=\"xy.ai.workbench.views.ClaudeCodeSessionView\"\n\t\t\ticon=\"icons/sample.png\"\n\t\t\tid=\"xy.ai.workbench.views.ClaudeCodeSessionView\"\n\t\t\tinject=\"true\"\n\t\t\tname=\"Claude Code Sessions\">\n\t\t</view>\n\t</extension>\n\t<extension point=\"org.eclipse.ui.navigator.navigatorContent\">\n\t\t<navigatorContent\n\t\t\tname=\"AI Content\"\n            contentProvider=\"xy.ai.workbench.views.explorer.MarkdownContentProvider\"\n\t\t\tlabelProvider=\"xy.ai.workbench.views.explorer.MarkdownLabelProvider\"\n\t\t\tid=\"xy.ai.workbench.views.content.AIContent\"\n\t\t\tpriority=\"normal\">\n\t\t\t<triggerPoints>\n\t\t\t\t<instanceof value=\"org.eclipse.core.resources.IResource\" />\n\t\t\t</triggerPoints>\n\t\t\t<possibleChildren>\n\t\t\t\t<instanceof value=\"org.eclipse.core.resources.IResource\" />\n\t\t\t</possibleChildren>\n\t       <commonWizard\n\t            type=\"new\"\n\t            wizardId=\"XY.AI.Workbench.wizard.newfile.chat\"\n\t            menuGroupId=\"new.ext\">\n            <enablement>\n                  <instanceof value=\"org.eclipse.core.resources.IFolder\"/>\n            </enablement>\n\t       </commonWizard>\n\t       <commonWizard\n\t            type=\"new\"\n\t            wizardId=\"XY.AI.Workbench.wizard.newfile.markdown\"\n\t            menuGroupId=\"new.ext\">\n            <enablement>\n                  <instanceof value=\"org.eclipse.core.resources.IFolder\"/>\n            </enablement>\n\t       </commonWizard>\n\t       <commonWizard\n\t            type=\"new\"\n\t            wizardId=\"XY.AI.Workbench.wizard.newfile.context\"\n\t            menuGroupId=\"new.ext\">\n            <enablement>\n                  <instanceof value=\"org.eclipse.core.resources.IFolder\"/>\n            </enablement>\n\t       </commonWizard>\n\t\t</navigatorContent>\n\t</extension>\n\t<extension point=\"org.eclipse.ui.navigator.viewer\">\n\t\t<viewer\n        inheritBindingsFromViewer=\"org.eclipse.ui.navigator.ProjectExplorer\"\n        popupMenuId=\"org.eclipse.ui.navigator.ProjectExplorer#PopupMenu\"\n        viewerId=\"xy.ai.workbench.views.AIExplorer\">\n  \t\t</viewer>\n\t   <viewerContentBinding viewerId=\"xy.ai.workbench.views.AIExplorer\">\n\t      <includes>\n\t         <contentExtension pattern=\"xy.ai.workbench.views.content.AIContent\"/>\n\t      </includes>\n\t      <excludes>\n\t\t      <contentExtension \n\t\t          pattern=\"^(?!xy\\.ai\\.workbench\\.views\\.content\\.AIContent$).*\">\n\t\t      </contentExtension>\n\t      </excludes>\n\t   </viewerContentBinding>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.commands\">\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench.commands.category\"\n\t\t\tname=\"AI\">\n\t\t</category>\n\t\t<command\n\t\t\tcategoryId=\"XY.AI.Workbench.commands.category\"\n\t\t\tid=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\tname=\"Submit prompt\">\n\t\t</command>\n\t\t<command\n\t\t\tcategoryId=\"XY.AI.Workbench.commands.category\"\n\t\t\tid=\"XY.AI.Workbench.commands.editor.comment\"\n\t\t\tname=\"Toggle AI Comment\">\n\t\t</command>\n\t\t<command\n\t\t\tcategoryId=\"XY.AI.Workbench.commands.category\"\n\t\t\tid=\"XY.AI.Workbench.commands.copyPath\"\n\t\t\tname=\"Copy Path\">\n\t\t</command>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.handlers\">\n\t\t<handler\n\t\t\tclass=\"xy.ai.workbench.handlers.SessionExecuteHandler\"\n\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\">\n\t\t</handler>\n\t\t<handler\n\t\t\tclass=\"xy.ai.workbench.editors.CommentHandler\"\n\t\t\tcommandId=\"XY.AI.Workbench.commands.editor.comment\">\n\t\t\t<activeWhen>\n\t\t\t\t<and>\n\t\t\t\t\t<with variable=\"activeEditorId\">\n\t\t\t\t\t\t<equals\n\t\t\t\t\t\t\tvalue=\"xy.ai.workbench.editors.AISessionEditor\">\n\t\t\t\t\t\t</equals>\n\t\t\t\t\t</with>\n\t\t\t\t\t<with variable=\"activeEditor\">\n\t\t\t\t\t\t<instanceof\n\t\t\t\t\t\t\tvalue=\"xy.ai.workbench.editors.AISessionEditor\" />\n\t\t\t\t\t</with>\n\t\t\t\t</and>\n\t\t\t</activeWhen>\n\t\t</handler>\n\t\t<handler\n\t\t\tclass=\"xy.ai.workbench.handlers.CopyPathHandler\"\n\t\t\tcommandId=\"XY.AI.Workbench.commands.copyPath\">\n\t\t</handler>\n\t</extension>\n\n\t<extension\n\t\tpoint=\"org.eclipse.ui.bindings\">\n\t\t<key\n\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\tcontextId=\"org.eclipse.ui.contexts.window\"\n\t\t\tschemeId=\"org.eclipse.ui.defaultAcceleratorConfiguration\"\n\t\t\tsequence=\"M1+Enter\">\n\t\t</key>\n\t\t<key\n\t\t\tcommandId=\"XY.AI.Workbench.commands.editor.comment\"\n\t\t\tcontextId=\"org.eclipse.ui.textEditorScope\"\n\t\t\tschemeId=\"org.eclipse.ui.defaultAcceleratorConfiguration\"\n\t\t\tsequence=\"Shift+Ctrl+C\">\n\t\t</key>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.menus\">\n\t\t<menuContribution\n\t\t\tlocationURI=\"menu:org.eclipse.ui.main.menu?after=additions\">\n\t\t\t<menu\n\t\t\t\tid=\"XY.AI.Workbench.menus.aiMenu\"\n\t\t\t\tlabel=\"AI\"\n\t\t\t\tmnemonic=\"A\">\n\t\t\t\t<command\n\t\t\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\t\t\ticon=\"icons/sample.png\"\n\t\t\t\t\tid=\"XY.AI.Workbench.menus.submitPromptCommand\"\n\t\t\t\t\tmnemonic=\"S\">\n\t\t\t\t</command>\n\t\t\t</menu>\n\t\t</menuContribution>\n\t\t<menuContribution\n\t\t\tlocationURI=\"toolbar:org.eclipse.ui.main.toolbar?after=additions\">\n\t\t\t<toolbar\n\t\t\t\tid=\"XY.AI.Workbench.toolbars.aiToolbar\">\n\t\t\t\t<command\n\t\t\t\t\tcommandId=\"XY.AI.Workbench.commands.submitPromptCommand\"\n\t\t\t\t\ticon=\"icons/sample.png\"\n\t\t\t\t\tid=\"XY.AI.Workbench.toolbars.submitPromptCommand\"\n\t\t\t\t\ttooltip=\"Submit the current prompt\">\n\t\t\t\t</command>\n\t\t\t</toolbar>\n\t\t</menuContribution>\n\t\t<menuContribution\n\t\t\tlocationURI=\"popup:org.eclipse.ui.popup.any?after=additions\">\n\t\t\t<command\n\t\t\t\tcommandId=\"XY.AI.Workbench.commands.copyPath\"\n\t\t\t\tlabel=\"Copy Path\"\n\t\t\t\tmnemonic=\"P\"\n\t\t\t\tstyle=\"push\">\n\t\t\t\t<visibleWhen\n\t\t\t\t\tcheckEnabled=\"false\">\n\t\t\t\t\t<with\n\t\t\t\t\t\tvariable=\"selection\">\n\t\t\t\t\t\t<iterate\n\t\t\t\t\t\t\tifEmpty=\"false\">\n\t\t\t\t\t\t\t<adapt\n\t\t\t\t\t\t\t\ttype=\"org.eclipse.core.resources.IResource\">\n\t\t\t\t\t\t\t</adapt>\n\t\t\t\t\t\t</iterate>\n\t\t\t\t\t</with>\n\t\t\t\t</visibleWhen>\n\t\t\t</command>\n\t\t</menuContribution>\n\t\t<menuContribution\n\t\t\tlocationURI=\"popup:#TextEditorContext?after=additions\">\n\t\t\t<command\n\t\t\t\tcommandId=\"XY.AI.Workbench.commands.copyPath\"\n\t\t\t\tlabel=\"Copy Path\"\n\t\t\t\tmnemonic=\"P\"\n\t\t\t\tstyle=\"push\">\n\t\t\t</command>\n\t\t</menuContribution>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.perspectiveExtensions\">\n\t\t<perspectiveExtension\n\t\t\ttargetID=\"org.eclipse.jdt.ui.JavaPerspective\">\n\t\t\t<view\n\t\t\t\tid=\"xy.ai.workbench.views.AIBatchView\"\n\t\t\t\tratio=\"0.5\"\n\t\t\t\trelationship=\"bottom\"\n\t\t\t\trelative=\"org.eclipse.ui.views.ProblemView\">\n\t\t\t</view>\n\t\t</perspectiveExtension>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.fontDefinitions\">\n\t\t<fontDefinition\n\t\t\tid=\"XY.AI.Workbench.fontDefinition1\"\n\t\t\tlabel=\"XY.AI Editor Font\"\n\t\t\tvalue=\"1|Arial|10.0|0|WINDOWS|1|0|0|0|0|0|0|0|0|3|0|0|0|Arial;\">\n\t\t</fontDefinition>\n\t</extension>\n\t<extension\n\t\tid=\"xy.ai.workbench.promptmarker\"\n\t\tname=\"XY.AI Prompt Marker\"\n\t\tpoint=\"org.eclipse.core.resources.markers\">\n\t\t<persistent\n\t\t\tvalue=\"true\">\n\t\t</persistent>\n\t\t<super\n\t\t\ttype=\"org.eclipse.core.resources.taskmarker\">\n\t\t</super>\n\t</extension>\n\t<extension\n\t\tpoint=\"org.eclipse.ui.newWizards\">\n\t\t<wizard\n\t\t\tid=\"XY.AI.Workbench.wizard.newfile.markdown\"\n\t\t\tname=\"Markdown File\"\n\t\t\tclass=\"xy.ai.workbench.handlers.NewMarkdownWizard\"\n\t\t\tcategory=\"XY.AI.Workbench.newfile.category\"\n\t\t\ticon=\"icons/sample.png\">\n\t\t\t<description>\n\t\t\t\tCreates a new Markdown file.\n\t\t\t</description>\n\t\t</wizard>\n\t\t<wizard\n\t\t\tid=\"XY.AI.Workbench.wizard.newfile.context\"\n\t\t\tname=\"Prompt Context\"\n\t\t\tclass=\"xy.ai.workbench.handlers.NewPromptContextWizard\"\n\t\t\tcategory=\"XY.AI.Workbench.newfile.category\"\n\t\t\ticon=\"icons/sample.png\">\n\t\t\t<description>\n\t\t\t\tCreates a new Prompt Context file.\n\t\t\t</description>\n\t\t</wizard>\n\t\t<wizard\n\t\t\tid=\"XY.AI.Workbench.wizard.newfile.chat\"\n\t\t\tname=\"AI Chat\"\n\t\t\tclass=\"xy.ai.workbench.handlers.NewChatWizard\"\n\t\t\tcategory=\"XY.AI.Workbench.newfile.category\"\n\t\t\ticon=\"icons/sample.png\">\n\t\t\t<description>\n\t\t\t\tCreates a new AI Chat file.\n\t\t\t</description>\n\t\t</wizard>\n\t\t<category\n\t\t\tid=\"XY.AI.Workbench.newfile.category\"\n\t\t\tname=\"AI Files\">\n\t\t</category>\n\t</extension>\n\n\t<extension point=\"org.eclipse.ui.editors.annotationTypes\">\n\t\t<type name=\"xy.ai.workbench.editors.spellcheck.spelling\"/>\n\t</extension>\n\n</plugin>\n"
#:     }
#:   }
#: }
/allow 73cd0f91-ea33-4763-84a8-fc4cca24a929
#: Control Request:
#: {
#:   "id" : "882a71f0-d5ed-48d4-845b-1e8b53cd9c3c",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/GitIgnoreRule.java",
#:     "mode" : "replace",
#:     "content" : "package xy.ai.workbench.views.explorer;\n\nimport java.util.regex.Pattern;\n\n/**\n * A single parsed line of a \".gitignore\" file, compiled into two regular\n * expressions:\n * <ul>\n * <li>{@code exactPattern} matches when the checked path itself is the\n * ignored entry.</li>\n * <li>{@code nestedPattern} matches when the checked path is located\n * <i>inside</i> an ignored directory.</li>\n * </ul>\n */\nfinal class GitIgnoreRule {\n\n\tprivate final Pattern exactPattern;\n\tprivate final Pattern nestedPattern;\n\tprivate final boolean negate;\n\tprivate final boolean dirOnly;\n\n\tprivate GitIgnoreRule(Pattern exactPattern, Pattern nestedPattern, boolean negate, boolean dirOnly) {\n\t\tthis.exactPattern = exactPattern;\n\t\tthis.nestedPattern = nestedPattern;\n\t\tthis.negate = negate;\n\t\tthis.dirOnly = dirOnly;\n\t}\n\n\tboolean isNegate() {\n\t\treturn negate;\n\t}\n\n\t/**\n\t * @param relativePath path of the checked resource, relative to the\n\t *                      directory that contains the \".gitignore\" file,\n\t *                      using '/' as separator\n\t * @param isDirectory   whether the checked resource itself is a folder\n\t */\n\tboolean matches(String relativePath, boolean isDirectory) {\n\t\tif (exactPattern.matcher(relativePath).matches())\n\t\t\treturn !dirOnly || isDirectory;\n\t\treturn nestedPattern.matcher(relativePath).matches();\n\t}\n\n\tstatic GitIgnoreRule parse(String rawLine) {\n\t\tif (rawLine == null)\n\t\t\treturn null;\n\t\tString line = rawLine.stripTrailing();\n\t\tif (line.isEmpty() || line.startsWith(\"#\"))\n\t\t\treturn null;\n\n\t\tboolean negate = false;\n\t\tif (line.startsWith(\"!\")) {\n\t\t\tnegate = true;\n\t\t\tline = line.substring(1);\n\t\t}\n\t\tline = line.replace(\"\\\\ \", \" \").replace(\"\\\\!\", \"!\").replace(\"\\\\#\", \"#\");\n\t\tif (line.isEmpty())\n\t\t\treturn null;\n\n\t\tboolean dirOnly = line.endsWith(\"/\");\n\t\tif (dirOnly)\n\t\t\tline = line.substring(0, line.length() - 1);\n\t\tif (line.isEmpty())\n\t\t\treturn null;\n\n\t\tboolean anchored = line.startsWith(\"/\");\n\t\tString pattern = anchored ? line.substring(1) : line;\n\t\tif (!anchored && pattern.indexOf('/') >= 0)\n\t\t\tanchored = true;\n\t\tif (pattern.isEmpty())\n\t\t\treturn null;\n\n\t\tString regex = toRegex(pattern);\n\t\tString prefix = anchored ? \"\" : \"(?:.*/)?\";\n\t\tPattern exact = Pattern.compile(\"^\" + prefix + regex + \"$\");\n\t\tPattern nested = Pattern.compile(\"^\" + prefix + regex + \"/.*$\");\n\t\treturn new GitIgnoreRule(exact, nested, negate, dirOnly);\n\t}\n\n\tprivate static String toRegex(String pattern) {\n\t\tStringBuilder regex = new StringBuilder();\n\t\tint n = pattern.length();\n\t\tfor (int i = 0; i < n; i++) {\n\t\t\tchar c = pattern.charAt(i);\n\t\t\tif (c == '*') {\n\t\t\t\tif (i + 1 < n && pattern.charAt(i + 1) == '*') {\n\t\t\t\t\tboolean slashBefore = i == 0 || pattern.charAt(i - 1) == '/';\n\t\t\t\t\tboolean slashAfter = i + 2 < n && pattern.charAt(i + 2) == '/';\n\t\t\t\t\tif (slashBefore && slashAfter) {\n\t\t\t\t\t\tregex.append(\"(?:.*/)?\");\n\t\t\t\t\t\ti += 2; // consumes \"**\", trailing '/' is skipped by the loop increment\n\t\t\t\t\t\tcontinue;\n\t\t\t\t\t}\n\t\t\t\t\tregex.append(\".*\");\n\t\t\t\t\ti++; // consumes the second '*'\n\t\t\t\t\tcontinue;\n\t\t\t\t}\n\t\t\t\tregex.append(\"[^/]*\");\n\t\t\t} else if (c == '?') {\n\t\t\t\tregex.append(\"[^/]\");\n\t\t\t} else if (c == '/') {\n\t\t\t\tregex.append('/');\n\t\t\t} else if (\"\\\\.^$|()[]{}+\".indexOf(c) >= 0) {\n\t\t\t\tregex.append('\\\\').append(c);\n\t\t\t} else {\n\t\t\t\tregex.append(c);\n\t\t\t}\n\t\t}\n\t\treturn regex.toString();\n\t}\n}\n"
#:   }
#: }
/allow 882a71f0-d5ed-48d4-845b-1e8b53cd9c3c
#: Control Request:
#: {
#:   "id" : "5bb0808a-d024-466f-9df2-776a6b2dc67b",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/GitIgnoreFilter.java",
#:     "mode" : "replace",
#:     "content" : "package xy.ai.workbench.views.explorer;\n\nimport java.nio.charset.StandardCharsets;\nimport java.util.ArrayList;\nimport java.util.Collections;\nimport java.util.HashMap;\nimport java.util.List;\nimport java.util.Map;\n\nimport org.eclipse.core.resources.IContainer;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IResource;\nimport org.eclipse.core.runtime.IPath;\nimport org.eclipse.core.runtime.Path;\n\n/**\n * Determines whether a workspace resource is excluded by one or more\n * \".gitignore\" files located between the project root and the resource\n * itself, applying the same precedence rules as Git (rules of a\n * \".gitignore\" file closer to the resource, respectively lines further down\n * within a file, take precedence over earlier ones; \"!\" negates a\n * previous match).\n */\npublic final class GitIgnoreFilter {\n\n\tprivate GitIgnoreFilter() {\n\t}\n\n\tprivate static final Map<String, CachedRules> CACHE = new HashMap<>();\n\n\tpublic static boolean isIgnored(IResource resource) {\n\t\tif (resource == null || resource.getType() == IResource.PROJECT)\n\t\t\treturn false;\n\n\t\tIPath relativePath = resource.getProjectRelativePath();\n\t\tif (relativePath == null || relativePath.isEmpty())\n\t\t\treturn false;\n\n\t\tString[] segments = relativePath.segments();\n\t\tif (segments.length > 0 && \".git\".equals(segments[0]))\n\t\t\treturn true;\n\n\t\tboolean isDirectory = resource instanceof IContainer;\n\t\tboolean ignored = false;\n\t\tIContainer dir = resource.getProject();\n\n\t\tfor (int i = 0; i < segments.length; i++) {\n\t\t\tList<GitIgnoreRule> rules = getRules(dir);\n\t\t\tif (!rules.isEmpty()) {\n\t\t\t\tStringBuilder relPath = new StringBuilder();\n\t\t\t\tfor (int j = i; j < segments.length; j++) {\n\t\t\t\t\tif (relPath.length() > 0)\n\t\t\t\t\t\trelPath.append('/');\n\t\t\t\t\trelPath.append(segments[j]);\n\t\t\t\t}\n\t\t\t\tString path = relPath.toString();\n\t\t\t\tfor (GitIgnoreRule rule : rules)\n\t\t\t\t\tif (rule.matches(path, isDirectory))\n\t\t\t\t\t\tignored = !rule.isNegate();\n\t\t\t}\n\t\t\tif (i < segments.length - 1) {\n\t\t\t\tIResource child = dir.findMember(segments[i]);\n\t\t\t\tif (child instanceof IContainer)\n\t\t\t\t\tdir = (IContainer) child;\n\t\t\t\telse\n\t\t\t\t\tbreak;\n\t\t\t}\n\t\t}\n\t\treturn ignored;\n\t}\n\n\tprivate static List<GitIgnoreRule> getRules(IContainer dir) {\n\t\tIFile gitignore = dir.getFile(new Path(\".gitignore\"));\n\t\tif (!gitignore.exists())\n\t\t\treturn Collections.emptyList();\n\n\t\tString key = gitignore.getFullPath().toString();\n\t\tlong stamp = gitignore.getModificationStamp();\n\n\t\tsynchronized (CACHE) {\n\t\t\tCachedRules cached = CACHE.get(key);\n\t\t\tif (cached != null && cached.stamp == stamp)\n\t\t\t\treturn cached.rules;\n\n\t\t\tList<GitIgnoreRule> rules = parse(gitignore);\n\t\t\tCACHE.put(key, new CachedRules(stamp, rules));\n\t\t\treturn rules;\n\t\t}\n\t}\n\n\tprivate static List<GitIgnoreRule> parse(IFile gitignore) {\n\t\tList<GitIgnoreRule> rules = new ArrayList<>();\n\t\ttry (var in = gitignore.getContents()) {\n\t\t\tString content = new String(in.readAllBytes(), StandardCharsets.UTF_8);\n\t\t\tfor (String line : content.split(\"\\r?\\n\")) {\n\t\t\t\tGitIgnoreRule rule = GitIgnoreRule.parse(line);\n\t\t\t\tif (rule != null)\n\t\t\t\t\trules.add(rule);\n\t\t\t}\n\t\t} catch (Exception e) {\n\t\t\t// unreadable \".gitignore\" files are simply ignored\n\t\t}\n\t\treturn rules;\n\t}\n\n\tprivate static final class CachedRules {\n\t\tfinal long stamp;\n\t\tfinal List<GitIgnoreRule> rules;\n\n\t\tCachedRules(long stamp, List<GitIgnoreRule> rules) {\n\t\t\tthis.stamp = stamp;\n\t\t\tthis.rules = rules;\n\t\t}\n\t}\n}\n"
#:   }
#: }
/allow 5bb0808a-d024-466f-9df2-776a6b2dc67b
#: Control Request:
#: {
#:   "id" : "740db21b-9fbd-4e5b-b0c7-40f6636fdc29",
#:   "phase" : "request",
#:   "toolName" : "write",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/MarkdownContentProvider.java",
#:     "mode" : "replace",
#:     "content" : "package xy.ai.workbench.views.explorer;\n\nimport java.util.ArrayList;\nimport java.util.List;\n\nimport org.eclipse.core.resources.IContainer;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IResource;\n\nimport xy.ai.workbench.views.FlatEndingContentProvider;\n\npublic class MarkdownContentProvider extends FlatEndingContentProvider {\n\tpublic MarkdownContentProvider() {\n\t\tsuper(\".md\");\n\t}\n\n\t@Override\n\tpublic Object[] getElements(Object root) {\n\t\treturn filterIgnored(super.getElements(root));\n\t}\n\n\t@Override\n\tpublic Object[] getChildren(Object parentElement) {\n\t\treturn filterIgnored(super.getChildren(parentElement));\n\t}\n\n\t/**\n\t * Removes every element that is located inside a directory excluded by a\n\t * \".gitignore\" file (or that is itself excluded).\n\t */\n\tprivate Object[] filterIgnored(Object[] elements) {\n\t\tList<Object> result = new ArrayList<>(elements.length);\n\t\tfor (Object element : elements) {\n\t\t\tif (element instanceof IResource && GitIgnoreFilter.isIgnored((IResource) element))\n\t\t\t\tcontinue;\n\t\t\tresult.add(element);\n\t\t}\n\t\treturn result.toArray();\n\t}\n}\n"
#:   }
#: }
/allow 740db21b-9fbd-4e5b-b0c7-40f6636fdc29
#: Control Request:
#: {
#:   "id" : "05988457-7c02-49fe-b3ee-d379ea19ac85",
#:   "phase" : "request",
#:   "toolName" : "change",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/MarkdownContentProvider.java",
#:     "start" : "import org.eclipse.core.resources.IContainer;\nimport org.eclipse.core.resources.IFile;\nimport org.eclipse.core.resources.IResource;",
#:     "end" : "import org.eclipse.core.resources.IResource;",
#:     "content" : "import org.eclipse.core.resources.IResource;"
#:   }
#: }
/allow 05988457-7c02-49fe-b3ee-d379ea19ac85
#: Control Request:
#: {
#:   "id" : "c08b5034-d6c3-44da-b18a-254f643ecf37",
#:   "phase" : "request",
#:   "toolName" : "read",
#:   "arguments" : {
#:     "path" : "/home/user/xyan/xy.ai.workbench/src/xy/ai/workbench/views/explorer/MarkdownContentProvider.java"
#:   }
#: }
/allow c08b5034-d6c3-44da-b18a-254f643ecf37
#: Control Request:
#: {
#:   "id" : "bec11ecc-a918-42a3-a9ba-f76a7ab28479",
#:   "phase" : "result",
#:   "toolName" : "read",
#:   "result" : {
#:     "content" : [ ],
#:     "structuredContent" : {
#:       "content" : "package xy.ai.workbench.views.explorer;\n\nimport java.util.ArrayList;\nimport java.util.List;\n\nimport org.eclipse.core.resources.IResource;\n\nimport xy.ai.workbench.views.FlatEndingContentProvider;\n\npublic class MarkdownContentProvider extends FlatEndingContentProvider {\n\tpublic MarkdownContentProvider() {\n\t\tsuper(\".md\");\n\t}\n\n\t@Override\n\tpublic Object[] getElements(Object root) {\n\t\treturn filterIgnored(super.getElements(root));\n\t}\n\n\t@Override\n\tpublic Object[] getChildren(Object parentElement) {\n\t\treturn filterIgnored(super.getChildren(parentElement));\n\t}\n\n\t/**\n\t * Removes every element that is located inside a directory excluded by a\n\t * \".gitignore\" file (or that is itself excluded).\n\t */\n\tprivate Object[] filterIgnored(Object[] elements) {\n\t\tList<Object> result = new ArrayList<>(elements.length);\n\t\tfor (Object element : elements) {\n\t\t\tif (element instanceof IResource && GitIgnoreFilter.isIgnored((IResource) element))\n\t\t\t\tcontinue;\n\t\t\tresult.add(element);\n\t\t}\n\t\treturn result.toArray();\n\t}\n}\n"
#:     }
#:   }
#: }
/allow bec11ecc-a918-42a3-a9ba-f76a7ab28479
#: ReasoningToken: 16356
#: ReasoningToken: 43
Das sieht gut aus. Zusammenfassung der Änderungen:

## Übersicht

**Neue Dateien:**

1. **`GitIgnoreRule.java`** – parst eine einzelne Zeile einer `.gitignore`-Datei und kompiliert sie zu zwei Regex-Mustern:
   - `exactPattern` – trifft, wenn die geprüfte Ressource selbst der ignorierte Eintrag ist
   - `nestedPattern` – trifft, wenn die Ressource *innerhalb* eines ignorierten Verzeichnisses liegt
   
   Unterstützt werden `*`, `?`, `**`, Verzeichnis-Anker (`/foo`), verzeichnisspezifische Muster (`foo/`) sowie Negation (`!foo`).

2. **`GitIgnoreFilter.java`** – die zentrale Prüf-Utility mit `isIgnored(IResource)`:
   - läuft den Pfad vom Projekt-Root bis zur Ressource entlang,
   - wertet an jeder Ebene die dort vorhandene `.gitignore` aus (git-typische Präzedenz: nähere Datei / spätere Zeile gewinnt, `!` hebt vorherige Treffer wieder auf),
   - ignoriert automatisch alles unterhalb von `.git`,
   - cached geparste Regeln pro `.gitignore`-Datei anhand des Modification-Stamps, um wiederholtes Parsen zu vermeiden.

**Geänderte Datei:**

3. **`MarkdownContentProvider.java`** – überschreibt jetzt `getElements(...)` und `getChildren(...)` der Basisklasse `FlatEndingContentProvider` und filtert die Ergebnisse via `GitIgnoreFilter.isIgnored(...)` heraus. Da `hasChildren(...)` in der Basisklasse intern `getChildren(...)` aufruft (virtuelle Methode), profitiert es automatisch mit – Ordner, deren einzige `.md`-Dateien ignoriert sind, werden korrekt als "leer" behandelt und nicht mehr als aufklappbar angezeigt.

Damit zeigt der Markdown-Explorer keine Markdown-Dateien mehr aus Verzeichnissen an, die per `.gitignore` (auf beliebiger Ebene zwischen Projekt-Root und Datei) ausgeschlossen sind.